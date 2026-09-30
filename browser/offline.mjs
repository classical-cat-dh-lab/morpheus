const $ = id => document.getElementById(id);
const release = document.querySelector('meta[name="morph-release"]').content;
let registration, saved = {}, candidate, installing, busy = false, expanded = false, jobId;
const standalone = () => matchMedia('(display-mode: standalone)').matches || navigator.standalone;
function render() {
  $('offline-panel').hidden = saved.ready && !expanded && !busy && !saved.pending;
  $('offline-help').hidden = !saved.ready;
  $('offline-close').hidden = !saved.ready || busy;
  $('offline-save').hidden = Boolean(saved.pending);
  $('offline-save').disabled = busy || !registration;
  $('offline-save').textContent = candidate ? 'Download update' : saved.ready ? 'Repair offline copy' : 'Save for offline';
  $('offline-check').hidden = !saved.ready;
  $('offline-check').disabled = busy;
  $('offline-reload').hidden = !saved.pending;
  $('offline-reload').disabled = busy;
  $('offline-cancel').hidden = !busy;
  $('offline-install').hidden = !installing || standalone();
  $('install-help').hidden = Boolean(standalone());
}
function describe() {
  $('offline-status').textContent = saved.pending ? 'Update checked and ready. Reload when you finish reading.' : saved.ready ? `Ready offline · ${saved.release === release ? 'this edition' : 'saved edition ' + saved.release}. Browser storage can still be cleared.` : 'No complete offline copy saved. The full download is about 63 MB.';
  render();
}
function message(action, extra = {}, onProgress) {
  return new Promise((resolve, reject) => {
    const channel = new MessageChannel(); let deadline;
    const renew = () => { clearTimeout(deadline); deadline = setTimeout(() => { channel.port1.close(); reject(new Error('Offline storage stopped responding. Reconnect and retry.')); }, 180000); };
    renew();
    channel.port1.onmessage = ({data}) => {
      renew();
      if (data.done) { clearTimeout(deadline); channel.port1.close(); data.error ? reject(new Error(data.error)) : resolve(data.result); }
      else onProgress?.(data);
    };
    (registration.active ?? navigator.serviceWorker.controller).postMessage({action, release, manifest:`/releases/${release}/release.json`, jobId, ...extra}, [channel.port2]);
  });
}
async function smoke(manifest) {
  for (const [language, text, lemma] of [['grc','lo/gos\n','lo/gos'],['lat','amo\n','amo']]) {
    await new Promise((resolve, reject) => {
      const worker = new Worker(`/releases/${manifest.id}/browser/worker.mjs`, {type:'module'});
      const end = error => { clearTimeout(timer); worker.terminate(); error ? reject(error) : resolve(); };
      const timer = setTimeout(() => end(new Error('The saved analyzer did not respond.')), 120000);
      worker.onerror = () => end(new Error('The saved analyzer could not start.'));
      worker.onmessage = ({data}) => {
        if (data.type === 'progress') return;
        const r = data.result, expected = manifest.engine;
        if (data.type !== 'result' || r.protocol !== expected.protocol || r.implementation !== expected.implementation || r.profile !== expected.profile || r.data !== expected.data || r.termination.kind !== 'exit' || r.termination.code !== 0 || !new TextDecoder().decode(r.stdout).includes(lemma)) return end(new Error(data.message ?? 'The saved analyzer failed its check.'));
        end();
      };
      worker.postMessage({...manifest.engine, id:'offline-check', operation:'cruncher', argv:language === 'lat' ? ['-T','-L'] : ['-T'], stdin:new TextEncoder().encode(text)});
    });
  }
}
$('offline-help').onclick = () => { expanded = true; render(); };
$('offline-close').onclick = () => { expanded = false; render(); };
$('offline-check').onclick = async () => {
  busy = true; render(); $('offline-status').textContent = 'Checking for updates…';
  try {
    await registration.update();
    const response = await fetch('/release.json', {cache:'no-store', signal:AbortSignal.timeout(15000)});
    if (!response.ok) throw new Error('The update list is unavailable. Your saved copy still works.');
    const manifest = await response.json();
    if (manifest.schema !== 1 || !/^\d+\.\d+\.\d+-[a-f0-9]{16}$/.test(manifest.id) || !manifest.engine) throw new Error('Unsupported update list.');
    candidate = manifest.id !== saved.release ? manifest : undefined;
    $('offline-status').textContent = candidate ? `Version ${manifest.version} is available. Download it when ready; unchanged saved data will be reused.` : 'You have the current edition. No application data was downloaded.';
  } catch (error) { $('offline-status').textContent = error.message; }
  finally { busy = false; render(); }
};
$('offline-save').onclick = async () => {
  busy = true; expanded = true; jobId = crypto.randomUUID(); render();
  $('offline-status').textContent = 'Preparing offline copy…';
  navigator.storage?.persist?.().catch(() => {});
  try {
    const id = candidate?.id ?? release;
    const result = await message('save', {release:id, manifest:`/releases/${id}/release.json`, staged:true}, p => { $('offline-status').textContent = `Saving ${p.completed} of ${p.total} files…`; });
    $('offline-status').textContent = 'Checking the saved Greek and Latin analyzer…';
    await smoke(result.candidate);
    saved = await message('confirm', {id, jobId}); candidate = undefined;
    expanded = false; describe();
  } catch (error) {
    await message('discard', {jobId}).catch(() => {});
    $('offline-status').textContent = `Offline save failed: ${error.message} Your previous complete copy is retained.`;
  } finally { busy = false; render(); message('cleanup').catch(() => {}); }
};
$('offline-cancel').onclick = () => message('cancel', {jobId}).catch(error => { $('offline-status').textContent = error.message; });
$('offline-reload').onclick = async () => {
  busy = true; render();
  try { await message('activate', {id:saved.pending}); location.reload(); }
  catch (error) { $('offline-status').textContent = error.message; busy = false; render(); }
};
$('offline-install').onclick = async () => { const prompt = installing; installing = undefined; render(); if (prompt) { await prompt.prompt(); await prompt.userChoice; } };
addEventListener('beforeinstallprompt', event => { event.preventDefault(); installing = event; render(); });
addEventListener('appinstalled', () => { installing = undefined; render(); });
if ('serviceWorker' in navigator) navigator.serviceWorker.addEventListener('message', event => {
  if (event.data?.action === 'morph-release-query') event.ports[0]?.postMessage({release, busy});
});
async function prepare() {
try {
  if (!('serviceWorker' in navigator)) throw new Error('Offline installation is unavailable in this browser. Local analysis still works while online.');
  registration = await navigator.serviceWorker.register('/sw.js', {updateViaCache:'none'});
  await navigator.serviceWorker.ready;
  // ready may describe the older worker while a new registration is installing.
  // Keep save disabled until this deployment's handler actually takes control.
  const incoming = registration.installing ?? registration.waiting;
  if (incoming && incoming.state !== 'activated') await new Promise((resolve,reject) => {
    const timer = setTimeout(() => reject(new Error('The offline update is waiting for another tab. Close older Morph tabs and reload.')), 60000);
    const changed = () => {
      if (incoming.state === 'activated') { clearTimeout(timer); resolve(); }
      if (incoming.state === 'redundant') { clearTimeout(timer); reject(new Error('The offline update could not start. Reload to retry.')); }
    };
    incoming.addEventListener('statechange',changed); changed();
  });
  saved = await message('status'); describe();
  message('cleanup').catch(() => {});
} catch (error) { $('offline-status').textContent = error.message; render(); }

}
prepare();
