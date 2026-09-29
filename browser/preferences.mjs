const pending = new Map();
let database;
function db() {
  return database ??= new Promise(resolve => {
    try {
      const timer = setTimeout(() => resolve(null), 1200);
      const request = indexedDB.open('morph-preferences', 1);
      request.onupgradeneeded = () => request.result.createObjectStore('preferences');
      request.onsuccess = () => { clearTimeout(timer); resolve(request.result); };
      request.onerror = request.onblocked = () => { clearTimeout(timer); resolve(null); };
    } catch { resolve(null); }
  });
}
export async function getPreference(key) {
  if (pending.has(key)) return pending.get(key);
  const store = await db();
  if (!store || pending.has(key)) return pending.get(key);
  return new Promise(resolve => {
    const timer = setTimeout(() => resolve(pending.get(key)), 1200);
    try {
      const request = store.transaction('preferences').objectStore('preferences').get(key);
      request.onsuccess = () => { clearTimeout(timer); resolve(pending.has(key) ? pending.get(key) : request.result); };
      request.onerror = () => { clearTimeout(timer); resolve(undefined); };
    } catch { clearTimeout(timer); resolve(undefined); }
  });
}
export async function setPreference(key, value) {
  pending.set(key, value);
  const store = await db();
  if (store) try { store.transaction('preferences', 'readwrite').objectStore('preferences').put(value, key); } catch {}
}
