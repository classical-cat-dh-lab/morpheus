/* The engine protocol and immutable release URLs isolate existing reading sessions. */
const PREFIX = 'morph-release-';
let saving, controller;
const openDB = () => new Promise((resolve,reject) => {
  const request = indexedDB.open('morph-offline',1);
  request.onupgradeneeded = () => request.result.createObjectStore('state');
  request.onsuccess = () => resolve(request.result);
  request.onerror = () => reject(new Error('Offline storage is unavailable.'));
  request.onblocked = () => reject(new Error('Offline storage is blocked by another page.'));
});
async function state(value) {
  const db = await openDB();
  return new Promise((resolve,reject) => {
    const transaction = db.transaction('state', value === undefined ? 'readonly' : 'readwrite');
    const store = transaction.objectStore('state');
    const request = value === undefined ? store.get('active') : store.put(value,'active');
    let result; request.onsuccess = () => { result = request.result; };
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
async function status() {
  const active = await state(); if (!active) return {ready:false};
  if (!(await caches.keys()).includes(active.cache)) return {ready:false};
  const cache = await caches.open(active.cache), metadata = await cache.match('/_morph/manifest');
  if (!metadata) return {ready:false};
  const manifest = await metadata.json(); validate(manifest,active.release);
  for (const file of manifest.files) {
    const response = await cache.match(file.url); if (!response) return {ready:false};
    const bytes = await response.arrayBuffer(); if (bytes.byteLength !== file.bytes || await digest(bytes) !== file.sha256) return {ready:false};
  }
  return {ready:true,release:active.release};
}
async function network(url, signal) {
  const timeout = AbortSignal.timeout(30000);
  const response = await fetch(url,{cache:'no-store',signal:AbortSignal.any([signal,timeout])});
  if (!response.ok || new URL(response.url).origin !== self.location.origin) throw new Error(`Download failed (${response.status}).`);
  return response;
}
async function save(message, port) {
  if (saving) throw new Error('An offline download is already running.');
  saving = true; controller = new AbortController();
  const cacheName = PREFIX + message.release + '-' + crypto.randomUUID();
  try {
    const response = await network(message.manifest,controller.signal), manifest = await response.json();
    validate(manifest,message.release);
    const cache = await caches.open(cacheName);
    let completed = 0;
    for (const file of manifest.files) {
      const response = await network(file.url,controller.signal), bytes = await response.arrayBuffer();
      if (bytes.byteLength !== file.bytes || await digest(bytes) !== file.sha256) throw new Error(`Integrity check failed for ${file.url}.`);
      await cache.put(file.url,new Response(bytes,{status:200,headers:response.headers}));
      port.postMessage({completed:++completed,total:manifest.files.length});
    }
    await cache.put(message.manifest,new Response(JSON.stringify(manifest),{headers:{'Content-Type':'application/json'}}));
    await cache.put('/_morph/manifest',new Response(JSON.stringify(manifest),{headers:{'Content-Type':'application/json'}}));
    if (controller.signal.aborted) throw new Error('Download cancelled.');
    await state({cache:cacheName,release:manifest.id});
    // Identical complete copies share every immutable URL. Replacing one does
    // not strand older pages, and repeated repairs must not consume storage forever.
    for (const key of (await caches.keys()).filter(k=>k.startsWith(PREFIX)&&k!==cacheName)) {
      try {
        const previous=await(await caches.open(key)).match('/_morph/manifest');
        if (previous && (await previous.json()).id===manifest.id) await caches.delete(key);
      } catch { /* Cleanup failure does not invalidate the activated complete copy. */ }
    }
    // Retain prior complete versions while existing pages can still use them.
    return {ready:true,release:manifest.id};
  } catch (error) { await caches.delete(cacheName); throw error; }
  finally { saving = false; controller = undefined; }
}
self.addEventListener('install',event=>event.waitUntil(self.skipWaiting()));
self.addEventListener('activate',event=>event.waitUntil(self.clients.claim()));
self.addEventListener('message',event=>{
  const port=event.ports[0]; if (!port) return;
  event.waitUntil((async()=>{
    try {
      const message=event.data;
      let result;
      if (message.action==='status') result=await status();
      else if (message.action==='cancel') { controller?.abort(); result={cancelled:true}; }
      else if (message.action==='save') result=await save(message,port);
      else throw new Error('Unknown offline operation.');
      port.postMessage({done:true,result});
    } catch(error) { port.postMessage({done:true,error:String(error.message??error)}); }
  })());
});
self.addEventListener('fetch',event=>{
  if (event.request.method!=='GET' || new URL(event.request.url).origin!==self.location.origin) return;
  event.respondWith((async()=>{
    if (event.request.mode==='navigate') {
      try { const response=await fetch(event.request,{signal:AbortSignal.timeout(8000)}); if (response.ok) return response; } catch {}
      const active=await state();
      const cached=active && await(await caches.open(active.cache)).match(event.request,{ignoreSearch:true});
      return cached ?? new Response('This page is not saved offline. Reconnect to save the complete application.',{status:503,headers:{'Content-Type':'text/plain'}});
    }
    const keys=await caches.keys();
    let active; try { active=await state(); } catch {}
    const complete=keys.filter(k=>k.startsWith(PREFIX));
    // A repaired release must supersede an older cache with the same asset URLs.
    const ordered=active ? [active.cache,...complete.filter(k=>k!==active.cache)] : complete.reverse();
    for (const key of ordered) {
      const cache=await caches.open(key); if (!await cache.match('/_morph/manifest')) continue;
      const cached=await cache.match(event.request); if(cached)return cached;
    }
    return fetch(event.request);
  })());
});
