/* The engine protocol and immutable release URLs isolate existing reading sessions. */
const PREFIX = 'morph-release-';
let saving, controller, owner, jobId;
const openDB = () => new Promise((resolve,reject) => {
  const request = indexedDB.open('morph-offline',1);
  request.onupgradeneeded = () => request.result.createObjectStore('state');
  request.onsuccess = () => resolve(request.result);
  request.onerror = () => reject(new Error('Offline storage is unavailable.'));
  request.onblocked = () => reject(new Error('Offline storage is blocked by another page.'));
});
async function state(change) {
  const db = await openDB();
  return new Promise((resolve,reject) => {
    const transaction = db.transaction('state', change ? 'readwrite' : 'readonly');
    const store = transaction.objectStore('state');
    const requests = ['active','previous','pending'].map(key => store.get(key));
    let result;
    requests[2].onsuccess = () => {
      result = Object.fromEntries(requests.map((request, index) => [['active','previous','pending'][index], request.result]));
      if (change) { result = change(result); for (const key of ['active','previous','pending']) store.put(result[key] ?? null,key); }
    };
    transaction.oncomplete = () => { db.close(); resolve(result); };
    transaction.onerror = transaction.onabort = () => { db.close(); reject(new Error('Could not update offline storage.')); };
  });
}

const digest = async data => [...new Uint8Array(await crypto.subtle.digest('SHA-256',data))].map(x=>x.toString(16).padStart(2,'0')).join('');
function validate(manifest, release) {
  if (manifest.schema !== 1 || manifest.id !== release || !/^\d+\.\d+\.\d+-[a-f0-9]{16}$/.test(release) || !Array.isArray(manifest.files) || !manifest.files.length) throw new Error('Unsupported release manifest.');
  const seen = new Set();
  for (const file of manifest.files) {
    const url = new URL(file.url,self.location.origin);
    if (url.origin !== self.location.origin || !file.url.startsWith('/') || file.url.startsWith('//') || url.search || url.hash || seen.has(file.url) || !Number.isSafeInteger(file.bytes) || file.bytes < 0 || !/^[a-f0-9]{64}$/.test(file.sha256)) throw new Error('Invalid offline asset.');
    seen.add(file.url);
  }
  if (!seen.has('/')) throw new Error('Release has no launch page.');
}
async function metadata(bundle) {
  if (!bundle || !await caches.has(bundle.cache)) return;
  const response = await (await caches.open(bundle.cache)).match('/_morph/manifest');
  if (!response) return;
  const manifest = await response.json(); validate(manifest,bundle.release); return manifest;
}
async function verified(response, file) {
  if (!response) throw new Error('A saved file is missing.');
  const bytes = await response.arrayBuffer();
  if (bytes.byteLength !== file.bytes || await digest(bytes) !== file.sha256) throw new Error(`Integrity check failed for ${file.url}.`);
  // Network response URLs are intentionally not retained: modules resolve their
  // relative imports against the requested URL in the newly selected release.
  return new Response(bytes,{status:200,headers:response.headers});
}
async function healthy(bundle) {
  try {
    const manifest = await metadata(bundle); if (!manifest) return false;
    const cache = await caches.open(bundle.cache);
    for (const file of manifest.files) await verified(await cache.match(file.url),file);
    return true;
  } catch { return false; }
}
async function status() {
  const saved = await state();
  return {ready:await healthy(saved.active),release:saved.active?.release,pending:saved.pending?.verified && await healthy(saved.pending) ? saved.pending.release : undefined};
}
async function network(url, signal) {
  const timeout = AbortSignal.timeout(30000);
  const response = await fetch(url,{cache:'no-store',signal:AbortSignal.any([signal,timeout])});
  if (!response.ok || new URL(response.url).origin !== self.location.origin) throw new Error(`Download failed (${response.status}).`);
  return response;
}
async function save(message, port, client) {
  if (saving) throw new Error('An offline download is already running.');
  saving = true; controller = new AbortController(); owner = client; jobId = message.jobId;
  const cacheName = PREFIX + message.release + '-' + crypto.randomUUID();
  try {
    const response = await network(message.manifest,controller.signal), manifest = await response.json();
    validate(manifest,message.release);
    const saved = await state();
    if (saved.pending && !saved.pending.verified && saved.pending.owner && await self.clients.get(saved.pending.owner)) throw new Error('Another page is checking downloaded files.');
    const sources = [];
    for (const bundle of [saved.active,saved.previous,saved.pending]) {
      try { const old = await metadata(bundle); if (old) sources.push({cache:await caches.open(bundle.cache),files:old.files}); } catch {}
    }
    const cache = await caches.open(cacheName);
    let completed = 0, reused = 0, downloadedBytes = 0;
    for (const file of manifest.files) {
      if (controller.signal.aborted) throw new Error('Download cancelled.');
      let incoming;
      for (const source of sources) {
        const old = source.files.find(f => f.sha256 === file.sha256 && f.bytes === file.bytes);
        if (old) try { incoming = await verified(await source.cache.match(old.url),file); reused++; break; } catch {}
      }
      if (!incoming) { incoming = await verified(await network(file.url,controller.signal),file); downloadedBytes += file.bytes; }
      await cache.put(file.url,incoming);
      port.postMessage({completed:++completed,total:manifest.files.length,reused,downloadedBytes});
    }
    await cache.put(message.manifest,new Response(JSON.stringify(manifest),{headers:{'Content-Type':'application/json'}}));
    await cache.put('/_morph/manifest',new Response(JSON.stringify(manifest),{headers:{'Content-Type':'application/json'}}));
    if (controller.signal.aborted) throw new Error('Download cancelled.');
    const bundle = {cache:cacheName,release:manifest.id,owner:client,jobId:message.jobId,verified:!message.staged};
    // Legacy 0.0.2 messages retain their immediate activation behavior. New pages
    // validate the staged engine in a fresh Worker before confirming the bundle.
    await state(current => message.staged ? {...current,pending:bundle} : {active:bundle,previous:current.active?.release !== bundle.release ? current.active : current.previous,pending:null});
    return {ready:!message.staged,release:manifest.id,candidate:manifest,reused,downloadedBytes};
  } catch (error) { await caches.delete(cacheName); throw error; }
  finally { saving = false; controller = undefined; owner = undefined; jobId = undefined; }
}
async function discard(client, id) {
  let abandoned;
  await state(current => {
    if (current.pending && !current.pending.verified && current.pending.owner === client && current.pending.jobId === id) { abandoned = current.pending.cache; return {...current,pending:null}; }
    return current;
  });
  if (abandoned) await caches.delete(abandoned);
  return {discarded:Boolean(abandoned)};
}
async function confirm(message, client) {
  const saved = await state(), pending = saved.pending;
  if (!pending || pending.release !== message.id || pending.owner !== client || pending.jobId !== message.jobId || !await healthy(pending)) throw new Error('The downloaded copy must be checked again.');
  await state(current => {
    if (current.pending?.cache !== pending.cache) throw new Error('The downloaded copy changed.');
    const bundle = {...pending,verified:true};
    return !current.active || current.active.release === bundle.release ? {...current,active:bundle,pending:null} : {...current,pending:bundle};
  });
  await cleanup(); return status();
}
async function activate(id) {
  const saved = await state();
  if (!saved.pending?.verified || saved.pending.release !== id || !await healthy(saved.pending)) throw new Error('No verified update is ready.');
  await state(current => {
    if (current.pending?.cache !== saved.pending.cache) throw new Error('The update changed.');
    return {active:current.pending,previous:current.active,pending:null};
  });
  await cleanup(); return status();
}
async function cleanup() {
  if (saving) return {deferred:true};
  const clients = await self.clients.matchAll({type:'all',includeUncontrolled:true});
  const reports = await Promise.all(clients.map(client => new Promise(resolve => {
    if (client.type !== 'window') return resolve({release:/\/releases\/([^/]+)\//.exec(client.url)?.[1]});
    const channel = new MessageChannel();
    const timer = setTimeout(() => {channel.port1.close();resolve(null);},1000);
    channel.port1.onmessage = ({data}) => {clearTimeout(timer);channel.port1.close();resolve(data);};
    client.postMessage({action:'morph-release-query'},[channel.port2]);
  })));
  // Unknown/sleeping legacy pages prevent destructive cleanup.
  if (reports.some(r => !r?.release || r.busy)) return {deferred:true};
  const saved = await state(), keep = new Set([saved.active?.cache,saved.previous?.cache,saved.pending?.cache]);
  const selected = new Set([saved.active?.release,saved.previous?.release,saved.pending?.release]);
  let deleted = 0;
  for (const name of await caches.keys()) {
    if (saving || !name.startsWith(PREFIX) || keep.has(name) || reports.some(r => !selected.has(r.release) && name.startsWith(PREFIX+r.release+'-'))) continue;
    if (await caches.delete(name)) deleted++;
  }
  return {deleted};
}
self.addEventListener('install',event=>event.waitUntil(self.skipWaiting()));
self.addEventListener('activate',event=>event.waitUntil(self.clients.claim()));
self.addEventListener('message',event=>{
  const port=event.ports[0]; if (!port || !event.source?.url || new URL(event.source.url).origin !== self.location.origin) return;
  event.waitUntil((async()=>{
    try {
      const message=event.data;
      let result;
      if (message.action==='status') result=await status();
      else if (message.action==='cancel') { if (owner === event.source.id && jobId === message.jobId) controller?.abort(); await discard(event.source.id,message.jobId); result={cancelled:true}; }
      else if (message.action==='save') result=await save(message,port,event.source.id);
      else if (message.action==='confirm') result=await confirm(message,event.source.id);
      else if (message.action==='discard') result=await discard(event.source.id,message.jobId);
      else if (message.action==='activate') result=await activate(message.id);
      else if (message.action==='cleanup') result=await cleanup();
      else throw new Error('Unknown offline operation.');
      port.postMessage({done:true,result});
    } catch(error) { port.postMessage({done:true,error:String(error.message??error)}); }
  })());
});
self.addEventListener('fetch',event=>{
  if (event.request.method!=='GET' || new URL(event.request.url).origin!==self.location.origin || ['/release.json','/sw.js'].includes(new URL(event.request.url).pathname)) return;
  event.respondWith((async()=>{
    if (event.request.mode==='navigate' || new URL(event.request.url).pathname === '/manifest.webmanifest') {
      const {active} = await state();
      // The 0.0.2 shell has no update control. Its first online navigation must
      // still discover the new UI. From 0.1 onward the saved shell is local-first.
      const modern = active && !active.release.startsWith('0.0.');
      const cached = active && await(await caches.open(active.cache)).match(event.request,{ignoreSearch:true});
      if (modern && cached && await healthy(active)) return cached;
      try { const response=await fetch(event.request,{signal:AbortSignal.timeout(8000)}); if (response.ok) return response; } catch {}
      return cached ?? new Response('This page is not saved offline. Reconnect to save the complete application.',{status:503,headers:{'Content-Type':'text/plain'}});
    }
    const keys=await caches.keys();
    let active,pending; try { ({active,pending}=await state()); } catch {}
    const complete=keys.filter(k=>k.startsWith(PREFIX));
    // A repaired release must supersede an older cache with the same asset URLs.
    const preferred=[pending?.cache,active?.cache].filter(Boolean);
    const ordered=[...preferred,...complete.filter(k=>!preferred.includes(k)).reverse()];
    for (const key of ordered) {
      const cache=await caches.open(key); if (!await cache.match('/_morph/manifest')) continue;
      const cached=await cache.match(event.request); if(cached)return cached;
    }
    return fetch(event.request);
  })());
});
