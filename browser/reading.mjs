import {fromBeta} from './input.mjs';
import {lookupDictionary} from './dictionary.mjs';
const node = (tag, text, className) => { const e = document.createElement(tag); if (text !== undefined) e.textContent = text; if (className) e.className = className; return e; };
const languageTag = language => language === 'grc' ? 'grc' : 'la';
export function parseAnalysis(text) {
  const match = text.match(/^([A-Z])\s+(?:(\S+),)?(\S+)\s+([\s\S]*)$/);
  return match ? {type:match[1], work:match[2] ?? null, lemma:match[3], fields:match[4]} : null;
}
const shownLemma = (lemma, language, type) => language === 'grc' && type !== 'E' ? fromBeta(lemma) : lemma;

export function analysisRecords(prepared, stdout) {
  const queues = new Map();
  for (const match of stdout.matchAll(/^([^\n]+)\n((?:<NL>[\s\S]*?<\/NL>)+)\n/gm)) {
    if (!queues.has(match[1])) queues.set(match[1], []);
    queues.get(match[1]).push([...match[2].matchAll(/<NL>([\s\S]*?)<\/NL>/g)].map(m => m[1]));
  }
  return prepared.records.map(item => ({...item, analyses: queues.get(item.input.trim().split(/\s/)[0].replace(/\d+$/, ''))?.shift() ?? []}));
}

export function passageParts(original, records) {
  const parts = []; let cursor = 0;
  records.forEach((record, index) => {
    if (!Number.isInteger(record.start) || !Number.isInteger(record.end) || record.start < cursor || record.end <= record.start || record.end > original.length) throw new Error('Invalid original-text span.');
    parts.push({text: original.slice(cursor, record.start)}, {text: original.slice(record.start, record.end), index});
    cursor = record.end;
  });
  parts.push({text: original.slice(cursor)});
  return parts;
}

function dictionaryBody(html, dictionary) {
  // Parse inertly and copy only our passive vocabulary; never mount source HTML.
  const source = new DOMParser().parseFromString(html, 'text/html').body;
  const output = document.createDocumentFragment();
  function copy(parent, target, beta = false) {
    for (const child of parent.childNodes) {
      if (child.nodeType === 3) target.append(document.createTextNode(beta ? fromBeta(child.textContent) : child.textContent));
      else if (child.nodeType === 1) {
        const element = node(['SPAN','DIV','STRONG','EM','BR'].includes(child.tagName) ? child.tagName.toLowerCase() : 'span');
        if (['grc','la','en','he','fr'].includes(child.lang)) element.lang = child.lang;
        for (const name of ['dictionary-sense','sense-label']) if (child.classList.contains(name)) element.classList.add(name);
        copy(child, element, child.lang ? dictionary === 'lsj' && child.lang === 'grc' : beta);
        target.append(element);
      }
    }
  }
  copy(source, output); return output;
}

async function showDefinitions(root, lemma, language, type) {
  const message = node('p', 'Loading dictionary entry…', 'help'); message.setAttribute('role','status'); root.replaceChildren(message);
  try {
    if (type === 'E') { root.replaceChildren(node('p', 'This original English label has no Greek/Latin dictionary mapping.', 'help')); return; }
    const result = await lookupDictionary(lemma, language);
    if (!root.isConnected) return;
    root.replaceChildren();
    if (!result.entries.length) {
      root.append(node('p', `No matching entry in ${result.dictionary === 'lsj' ? 'LSJ' : 'Lewis & Short'} for this original lemma.`, 'help')); return;
    }
    for (const entry of result.entries) {
      const article = node('article', undefined, 'dictionary-entry');
      const label = node('p', `${result.dictionary === 'lsj' ? 'LSJ' : 'Lewis & Short'} · ${shownLemma(entry.key,language)} · ${entry.id}`, 'dictionary-credit');
      article.append(label);
      const preview = node('p', undefined, 'dictionary-excerpts');
      for (const piece of entry.preview) {
        const span = node('span', result.dictionary === 'lsj' && piece.language === 'greek' ? fromBeta(piece.text) : piece.text);
        if (piece.language === 'greek') span.lang = 'grc'; else if (piece.language === 'la') span.lang = 'la';
        preview.append(span);
      }
      article.append(preview, node('p', 'Original entry excerpt; expand for full context.', 'help excerpt-note'));
      const full = node('details', undefined, 'dictionary-full');
      full.append(node('summary', 'Complete dictionary entry'));
      const body = node('div', undefined, 'dictionary-body');
      const materialize = () => { if (full.open && !body.childNodes.length) body.append(dictionaryBody(entry.html, result.dictionary)); };
      full.addEventListener('toggle', materialize);
      full.append(body); article.append(full); root.append(article); materialize();
      const provenance = node('p', undefined, 'help dictionary-source');
      const link = node('a', 'Perseus source XML');
      link.href = 'https://github.com/PerseusDL/lexica/blob/' + result.edition + '/' + entry.source + '#' + entry.id;
      provenance.append(link, document.createTextNode(' · CC BY-SA 4.0 · Uncollated edition')); article.append(provenance);
    }
  } catch (error) {
    if (!root.isConnected) return;
    root.replaceChildren(node('p', `${error.message} Morphology is still available.`, 'error help'));
    const retry = node('button', 'Retry dictionary'); retry.type = 'button'; retry.onclick = () => showDefinitions(root, lemma, language, type); root.append(retry);
  }
}
function tokenContent(root, token, language) {
  const heading = node('h3', token.original, 'form'); heading.lang = languageTag(language); root.append(heading);
  if (!token.analyses.length) { root.append(node('p', 'No analysis in this Morpheus data edition.', 'unknown')); return; }
  const lemmas = new Map();
  for (const analysis of token.analyses) {
    const parsed = parseAnalysis(analysis);
    if (parsed && !lemmas.has(parsed.lemma)) lemmas.set(parsed.lemma, parsed.type);
  }
  const definitions = node('section', undefined, 'definitions'); definitions.setAttribute('aria-label','Dictionary meanings');
  for (const [lemma,type] of lemmas) {
    const group = node('section', undefined, 'lemma-entry');
    const title = node('h4', shownLemma(lemma,language,type), 'lemma'); title.lang = type === 'E' ? 'en' : languageTag(language);
    const content = node('div'); group.append(title,content); definitions.append(group);
    // Wait for this reading panel to be attached before asynchronous completion.
    queueMicrotask(() => { if (content.isConnected) showDefinitions(content,lemma,language,type); });
  }
  root.append(definitions);
  root.append(node('h4', 'Morphological analyses', 'analysis-heading'));
  const list = node('ol', undefined, 'analyses');
  for (const analysis of token.analyses) {
    const li = node('li', undefined, 'analysis'), parsed = parseAnalysis(analysis);
    if (!parsed) li.append(node('pre',analysis));
    else {
      const lemma = node('span',shownLemma(parsed.lemma,language,parsed.type),'lemma'); lemma.lang = parsed.type === 'E' ? 'en' : languageTag(language);
      const fields = parsed.fields.split('\t');
      li.append(lemma,node('p',`${({N:'Nominal',V:'Verb',P:'Participle',I:'Indeclinable',E:'English lemma'})[parsed.type] ?? parsed.type} · ${fields[0].trim()}`,'grammar'));
      if (parsed.work) li.append(node('p', 'Original working form: ' + shownLemma(parsed.work,language,parsed.type), 'tags'));
      const tags = fields.slice(1).filter(Boolean).join(' · '); if (tags) li.append(node('p',tags,'tags'));
    }
    list.append(li);
  }
  root.append(list);
}

export function renderReading(root, prepared, result) {
  const records = analysisRecords(prepared, new TextDecoder().decode(result.stdout));
  root.replaceChildren();
  if (records.length === 1) tokenContent(root,records[0],prepared.language);
  else {
    const controls = node('div',undefined,'reading-tabs'); controls.setAttribute('role','tablist'); controls.setAttribute('aria-label','Results view');
    const textTab = node('button','Text'), listTab = node('button','List');
    const textPanel = node('section'), listPanel = node('section'); textPanel.id = 'passage-view'; listPanel.id = 'list-view';
    const tabs = [textTab,listTab], panels = [textPanel,listPanel];
    function selectView(index) { tabs.forEach((tab,i) => { tab.setAttribute('aria-selected',String(i===index)); tab.tabIndex=i===index?0:-1; panels[i].hidden=i!==index; }); }
    tabs.forEach((tab,i) => { tab.type='button';tab.id='view-'+['text','list'][i];tab.setAttribute('role','tab');tab.setAttribute('aria-controls',panels[i].id);tab.onclick=()=>selectView(i);tab.onkeydown=e=>{if(['ArrowLeft','ArrowRight','Home','End'].includes(e.key)){e.preventDefault();const n=e.key==='Home'?0:e.key==='End'?1:1-i;selectView(n);tabs[n].focus();}};panels[i].setAttribute('role','tabpanel');panels[i].setAttribute('aria-labelledby',tab.id);controls.append(tab); });
    textPanel.append(node('p','Select a word to see its meanings and forms.','help'));
    const passage = node('div',undefined,'passage-text'), detail = node('section',undefined,'passage-detail'); passage.id='passage-text';detail.id='passage-detail';detail.setAttribute('aria-label','Selected word');passage.lang=languageTag(prepared.language);
    const buttons = [];
    const choose = index => { buttons.forEach((button,i)=>button.setAttribute('aria-pressed',String(i===index)));detail.replaceChildren();tokenContent(detail,records[index],prepared.language); };
    for (const part of passageParts(prepared.original,records)) {
      if (part.index === undefined) passage.append(document.createTextNode(part.text));
      else {
        const button = node('button',part.text,'passage-word');button.type='button';button.dataset.index=part.index;button.setAttribute('aria-controls',detail.id);button.setAttribute('aria-pressed','false');button.onclick=()=>choose(part.index);buttons.push(button);passage.append(button);
      }
    }
    textPanel.append(passage,detail);
    records.forEach(token => {
      const item=node('details',undefined,'result'),summary=node('summary'),surface=node('span',token.original,'form'),content=node('div');surface.lang=languageTag(prepared.language);
      summary.append(surface,node('span',token.analyses.length?`${token.analyses.length} ${token.analyses.length===1?'analysis':'analyses'}`:'No analysis','count'));item.append(summary,content);
      item.addEventListener('toggle',()=>{if(item.open){if(!content.childNodes.length)tokenContent(content,token,prepared.language);for(const other of listPanel.children)if(other!==item)other.open=false;}});listPanel.append(item);
    });
    root.append(controls,textPanel,listPanel);selectView(0);choose(0);
  }
  return records.filter(token=>token.analyses.length).length;
}
