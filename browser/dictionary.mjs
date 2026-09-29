// Presentation-only join. No dictionary field is written back to the engine record.
export const adapter = 'morph-lexicon/1';
export function lemmaKey(lemma, language) {
  const key = language === 'lat' ? lemma.replace(/[_^]/g, '') : lemma;
  const match = key.match(/^(.*?)#?(\d+)$/);
  return {base: match ? match[1] : key, number: match ? Number(match[2]) : null};
}
export function dictionaryBucket(base) {
  let value = 2166136261;
  for (const byte of new TextEncoder().encode(base)) value = Math.imul(value ^ byte, 16777619) >>> 0;
  return (value % 128).toString(16).padStart(2, '0');
}
export function matchingEntries(records, lemma, language) {
  const {base, number} = lemmaKey(lemma, language);
  return (records[base] ?? []).filter(entry => number === null || entry.number === number);
}
const root = new URL('../dictionaries/', import.meta.url);
let manifestPromise;
const shards = new Map();
async function get(url) {
  const response = await fetch(url, {signal: AbortSignal.timeout(30000)});
  if (!response.ok) throw new Error(`Dictionary download failed (${response.status}).`);
  return response;
}
async function manifest() {
  if (!manifestPromise) manifestPromise = get(new URL('manifest.json', root)).then(r => r.json()).then(data => {
    if (data.schema !== 'morph-dictionaries/1' || data.adapter !== adapter || data.buckets !== 128) throw new Error('Unsupported dictionary edition.');
    return data;
  }).catch(error => { manifestPromise = undefined; throw error; });
  return manifestPromise;
}
export async function lookupDictionary(lemma, language) {
  const edition = await manifest(), id = language === 'grc' ? 'lsj' : 'ls', dictionary = edition.dictionaries[id];
  const {base} = lemmaKey(lemma, language), key = id + '/' + dictionaryBucket(base) + '.json.gz';
  if (!shards.has(key)) {
    const promise = (async () => {
      const expected = dictionary.files.find(file => file.path === key);
      if (!expected) throw new Error('Missing dictionary shard.');
      const data = await (await get(new URL(key, root))).arrayBuffer();
      const digest = [...new Uint8Array(await crypto.subtle.digest('SHA-256', data))].map(b => b.toString(16).padStart(2,'0')).join('');
      if (data.byteLength !== expected.bytes || digest !== expected.sha256) throw new Error('Dictionary integrity check failed. Re-save the offline copy.');
      return new Response(new Blob([data]).stream().pipeThrough(new DecompressionStream('gzip'))).json();
    })();
    shards.set(key, promise);
    promise.catch(() => { if (shards.get(key) === promise) shards.delete(key); });
    // A passage is rendered on selection; bound the retained decoded dictionaries.
    while (shards.size > 6) shards.delete(shards.keys().next().value);
  }
  const records = await shards.get(key);
  return {dictionary: id, title: dictionary.title, edition: edition.source, adapter, entries: matchingEntries(records, lemma, language)};
}
