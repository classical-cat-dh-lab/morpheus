// Measure dictionary joins on independently supplied corpus forms; do not infer matches.
import {readFileSync,writeFileSync} from 'node:fs';
import {gunzipSync} from 'node:zlib';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {runEngine,identity} from '../engine/runtime.mjs';
import {analysisRecords,parseAnalysis} from '../browser/reading.mjs';
import {lemmaKey,dictionaryBucket,matchingEntries} from '../browser/dictionary.mjs';
const [input,output]=process.argv.slice(2);if(!output)throw Error('Usage: node scripts/verify-dictionary-corpus.mjs INPUT_JSON OUTPUT_JSON');
const bytes=readFileSync(input),corpus=JSON.parse(bytes),report={schema:1,adapter:'morph-lexicon/1',inputSha256:createHash('sha256').update(bytes).digest('hex'),languages:{}};
const options={locateFile:name=>fileURLToPath(new URL('../engine/'+name,import.meta.url))};
for(const language of ['grc','lat']){
  const lemmas=new Map(),shards=new Map();let analyzedForms=0,formsWithDefinitions=0;
  for(let i=0;i<corpus[language].length;i+=25){
    const words=corpus[language].slice(i,i+25);
    const result=await runEngine({protocol:'morph-engine/1',id:`join-${language}-${i}`,profile:identity.profile,implementation:identity.implementation,data:identity.data,operation:'cruncher',argv:['-T',...(language==='lat'?['-L']:[])],stdin:new TextEncoder().encode(words.join('\n')+'\n')},options);
    if(result.termination.kind!=='exit'||result.termination.code!==0)throw Error('Corpus engine run did not complete.');
    const records=analysisRecords({records:words.map(input=>({input}))},new TextDecoder().decode(result.stdout));
    for(const record of records){
      if(record.analyses.length)analyzedForms++;
      let matched=false;
      for(const analysis of record.analyses){
        const parsed=parseAnalysis(analysis);if(!parsed)throw Error('Unparsed analysis.');
        const {lemma,type}=parsed;
        if(!lemmas.has(lemma)){
          const {base}=lemmaKey(lemma,language),id=language==='grc'?'lsj':'ls',key=id+'/'+dictionaryBucket(base)+'.json.gz';
          if(!shards.has(key))shards.set(key,JSON.parse(gunzipSync(readFileSync(new URL('../dictionaries/'+key,import.meta.url)))));
          lemmas.set(lemma,type==='E'?0:matchingEntries(shards.get(key),lemma,language).length);
        }
        if(lemmas.get(lemma))matched=true;
      }
      if(matched)formsWithDefinitions++;
    }
  }
  report.languages[language]={forms:corpus[language].length,analyzedForms,formsWithDictionaryEntries:formsWithDefinitions,uniqueLemmaLabels:lemmas.size,matchedLemmaLabels:[...lemmas.values()].filter(n=>n>0).length,unmatched:[...lemmas].filter(([,n])=>!n).map(([lemma])=>lemma).sort()};
}
writeFileSync(output,JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report.languages));
