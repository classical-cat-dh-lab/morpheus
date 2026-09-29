import {identity} from '../engine/identity.mjs';
import {prepareInput, fromBeta} from './input.mjs';
import {getPreference, setPreference} from './preferences.mjs';

const $ = id => document.getElementById(id);
const release = document.querySelector('meta[name="morph-release"]').content;
let worker, sequence = 0, timer, record, registration;
const decode = bytes => new TextDecoder().decode(bytes);
const bytes64 = bytes => { let text = ''; for (const byte of bytes) text += String.fromCharCode(byte); return btoa(text); };
function status(message, error = false) { $('status').textContent = message; $('status').classList.toggle('error', error); }
function theme(value) {
  const dark = value === 'dark'; document.documentElement.dataset.theme = dark ? 'dark' : 'light';
  $('theme-toggle').textContent = dark ? 'Light' : 'Dark';
  $('theme-toggle').setAttribute('aria-label', dark ? 'Switch to light theme' : 'Switch to dark theme');
  document.querySelector('meta[name="theme-color"]').content = getComputedStyle(document.documentElement).getPropertyValue('--ds-color-surface-base').trim();
}
$('theme-toggle').onclick = () => { const value = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark'; theme(value); setPreference('theme', value); };
getPreference('theme').then(value => { if (value) theme(value); });

function inputHelp() {
  const language = $('language').value, mode = $('mode').value;
  $('input').lang = language === 'lat' ? 'la' : 'grc';
  $('input').placeholder = mode === 'original' ? (language === 'grc' ? 'lo/gos\na)/nqrwpos\nlu/w' : 'amo\narma\nvirumque') : language === 'grc' ? 'λόγος, ἄνθρωπος, λύω' : 'arma virumque canō';
  $('input-help').textContent = mode === 'original' ? 'ASCII Beta Code / Latin, one word per line. Your bytes are passed through unchanged.' : language === 'grc' ? 'Enter a word or a passage. Punctuation separates words; accents and breathings are preserved.' : 'Enter a word or a passage. Macrons and breves are removed only in the input sent to Morpheus; your original text is retained.';
  $('ignore-accents').disabled = language !== 'grc';
}
for (const id of ['language', 'mode']) $(id).onchange = () => { inputHelp(); setPreference(id, $(id).value); };
for (const [id, allowed] of [['language', ['grc','lat']], ['mode', ['unicode','original']]]) getPreference(id).then(value => { if (allowed.includes(value)) { $(id).value = value; inputHelp(); } });
inputHelp();
$('show-raw').onchange = () => { $('raw').hidden = !$('show-raw').checked || !record; };
function finish() { clearTimeout(timer); worker?.terminate(); worker = null; $('analyze').disabled = false; $('cancel').hidden = true; }
function stop(message) { sequence++; finish(); status(message); }
$('cancel').onclick = () => stop('Analysis stopped. You can start a new lookup.');
$('clear').onclick = () => { stop('Ready.'); $('input').value = ''; $('results').replaceChildren(); $('raw').hidden = true; $('back-top').hidden = true; record = null; $('input').focus(); };
$('back-top').onclick = () => { $('input').focus({preventScroll:true}); window.scrollTo({top:0,behavior:matchMedia('(prefers-reduced-motion: reduce)').matches?'instant':'smooth'}); };
$('import').onchange = async () => { const file = $('import').files[0]; if (!file) return; if (file.size > 262144) return status('Choose a text file no larger than 256 KiB.', true); $('input').value = await file.text(); $('input').focus(); $('import').value = ''; };

function node(tag, text, className) { const element = document.createElement(tag); if (text !== undefined) element.textContent = text; if (className) element.className = className; return element; }
function displayResult(prepared, result) {
  const output = decode(result.stdout), queues = new Map();
  for (const match of output.matchAll(/^([^\n]+)\n((?:<NL>[\s\S]*?<\/NL>)+)\n/gm)) {
    const analyses = [...match[2].matchAll(/<NL>([\s\S]*?)<\/NL>/g)].map(x => x[1]);
    const key = match[1]; if (!queues.has(key)) queues.set(key, []); queues.get(key).push(analyses);
  }
  $('results').replaceChildren();
  let hits = 0;
  for (const item of prepared.records) {
    const key = item.input.trim().split(/\s/)[0].replace(/\d+$/, '');
    const analyses = queues.get(key)?.shift() ?? [];
    if (analyses.length) hits++;
    const details = node('details', undefined, 'result'); details.open = prepared.records.length === 1;
    const summary = node('summary'); const form = node('span', item.original, 'form'); form.lang = prepared.language === 'grc' ? 'grc' : 'la';
    summary.append(form, node('span', analyses.length ? `${analyses.length} ${analyses.length === 1 ? 'analysis' : 'analyses'}` : 'No analysis', 'count'));
    details.append(summary);
    if (!analyses.length) details.append(node('p', 'No analysis in this Morpheus data edition.', 'unknown'));
    else {
      const list = node('ol', undefined, 'analyses');
      for (const analysis of analyses) {
        const li = node('li', undefined, 'analysis');
        const match = analysis.match(/^([A-Z])\s+(\S+)\s+([\s\S]*)$/);
        if (!match) li.append(node('pre', analysis));
        else {
          const lemma = node('span', prepared.language === 'grc' && match[1] !== 'E' ? fromBeta(match[2]) : match[2], 'lemma'); lemma.lang = match[1] === 'E' ? 'en' : form.lang;
          const fields = match[3].split('\t');
          li.append(lemma, node('p', `${({N:'Nominal',V:'Verb',P:'Participle',I:'Indeclinable',E:'English lemma'})[match[1]] ?? match[1]} · ${fields[0].trim()}`, 'grammar'));
          const tags = fields.slice(1).filter(Boolean).join(' · '); if (tags) li.append(node('p', tags, 'tags'));
        }
        list.append(li);
      }
      details.append(list);
    }
    details.addEventListener('toggle', () => { if (details.open) for (const other of $('results').children) if (other !== details) other.open = false; });
    $('results').append(details);
  }
  $('back-top').hidden = prepared.records.length < 4;
  return hits;
}

$('lookup').onsubmit = event => {
  event.preventDefault(); finish(); const id = String(++sequence); record = null; $('raw').hidden = true; $('results').replaceChildren(); $('back-top').hidden = true;
  let prepared;
  try { prepared = prepareInput($('input').value, $('language').value, $('mode').value); if (!prepared.records.length) throw new Error('Enter a word to analyze.'); }
  catch (error) { return status(error.message, true); }
  const argv = ['-T']; if (prepared.language === 'lat') argv.push('-L');
  if (prepared.language === 'grc' && $('ignore-accents').checked) argv.push('-n');
  if ($('ignore-case').checked) argv.push('-S');
  const request = {protocol:'morph-engine/1', id, profile:identity.profile, implementation:identity.implementation, data:identity.data, operation:'cruncher', argv, stdin:prepared.bytes};
  worker = new Worker(new URL('./worker.mjs', import.meta.url), {type:'module'});
  $('analyze').disabled = true; $('cancel').hidden = false; status('Starting Morpheus…');
  timer = setTimeout(() => { if (id === String(sequence)) { stop('Analysis timed out. Try a smaller input.'); $('status').classList.add('error'); } },120000);
  worker.onerror = () => { if (id === String(sequence)) { finish(); status('The analysis worker could not run. Check the saved copy or try again.', true); } };
  worker.onmessage = ({data}) => {
    if (data.id !== String(sequence)) return;
    if (data.type === 'progress') return status(data.message);
    finish();
    if (data.type === 'error') return status(data.message, true);
    const result = data.result;
    record = {prepared, request, result};
    $('delivered').textContent = prepared.delivered; $('stdout').textContent = decode(result.stdout); $('stderr').textContent = decode(result.stderr);
    $('engine-info').textContent = `Protocol ${result.protocol}; profile ${result.profile}; ${result.termination.kind}${result.termination.code === undefined ? '' : ' ' + result.termination.code}.`;
    $('raw').hidden = !$('show-raw').checked;
    if (result.termination.kind !== 'exit' || result.termination.code !== 0) return status('Morpheus did not finish successfully. Original output is available in display options.', true);
    const hits = displayResult(prepared, result); status(`Complete: ${hits} of ${prepared.records.length} ${prepared.records.length === 1 ? 'word' : 'words'} analyzed.`);
  };
  worker.postMessage(request);
};
function download(name, contents, type) { const link = node('a'); const url = URL.createObjectURL(new Blob([contents], {type})); link.href = url; link.download = name; link.click(); setTimeout(() => URL.revokeObjectURL(url), 1000); }
$('download-output').onclick = () => { if (record) download('morph-output.txt', record.result.stdout, 'text/plain'); };
$('download-record').onclick = () => {
  if (!record) return;
  const {prepared, request, result} = record;
  const saved = {schema:'morph-execution-record/1', conversion:{...prepared,bytes:undefined}, request:{...request,stdin:bytes64(request.stdin)}, result:{...result,stdout:bytes64(result.stdout),stderr:bytes64(result.stderr),files:Object.fromEntries(Object.entries(result.files).map(([k,v])=>[k,bytes64(v)]))},byteEncoding:'base64'};
  download('morph-execution.json', JSON.stringify(saved,null,2)+'\n', 'application/json');
};

function offlineMessage(action, onProgress) {
  return new Promise((resolve,reject) => {
    const channel = new MessageChannel(); let deadline;
    function renew() { clearTimeout(deadline); deadline = setTimeout(() => { channel.port1.close(); reject(new Error('Offline storage stopped responding. Please try again.')); },45000); }
    renew();
    channel.port1.onmessage = ({data}) => {
      renew();
      if (data.done) { clearTimeout(deadline); channel.port1.close(); data.error ? reject(new Error(data.error)) : resolve(data.result); }
      else onProgress?.(data);
    };
    (navigator.serviceWorker.controller ?? registration.active).postMessage({action, release, manifest:`/releases/${release}/release.json`},[channel.port2]);
  });
}
function offlineState(result) { $('offline-status').textContent = result.ready ? `Ready offline · ${result.release === release ? 'this edition' : 'saved edition ' + result.release}. Browser storage can still be cleared.` : 'No complete offline copy saved.'; }
async function prepareOffline() {
  try {
    if (!('serviceWorker' in navigator)) throw new Error('Offline installation is unavailable in this browser. Online local analysis still works.');
    registration = await navigator.serviceWorker.register('/sw.js');
    await navigator.serviceWorker.ready; $('offline-save').disabled = false;
    offlineState(await offlineMessage('status'));
  } catch (error) { $('offline-status').textContent = error.message; }
}
$('offline-save').onclick = async () => {
  $('offline-save').disabled = true; $('offline-cancel').hidden = false;
  $('offline-status').textContent = 'Preparing offline copy…';
  navigator.storage?.persist?.().catch(()=>{});
  try { offlineState(await offlineMessage('save', p => { $('offline-status').textContent = `Saving ${p.completed} of ${p.total} files…`; })); }
  catch (error) { $('offline-status').textContent = `Offline save failed: ${error.message} Your previous complete copy is retained.`; }
  finally { $('offline-save').disabled = false; $('offline-cancel').hidden = true; }
};
$('offline-cancel').onclick = () => offlineMessage('cancel').catch(error => { $('offline-status').textContent = error.message; });
prepareOffline();
