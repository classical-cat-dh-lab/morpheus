import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {gunzipSync} from 'node:zlib';
import {createHash} from 'node:crypto';
import {analysisRecords,passageParts,parseAnalysis} from '../browser/reading.mjs';
import {prepareInput} from '../browser/input.mjs';
import {lemmaKey,dictionaryBucket,matchingEntries} from '../browser/dictionary.mjs';
const data=(id,key)=>JSON.parse(gunzipSync(readFileSync(new URL(`../dictionaries/${id}/${dictionaryBucket(key)}.json.gz`,import.meta.url))));
test('passage spans preserve punctuation, decomposed accents, whitespace and repeated occurrences',()=>{
  const original='λόγος,\n\nἄνθρωπος — λόγος!'.normalize('NFD'),prepared=prepareInput(original,'grc');
  const parts=passageParts(original,prepared.records);
  assert.equal(parts.map(p=>p.text).join(''),original);
  assert.deepEqual(parts.filter(p=>p.index!==undefined).map(p=>p.index),[0,1,2]);
  assert.throws(()=>passageParts('abc',[{start:0,end:2},{start:1,end:3}]),/span/);
});
test('display parsing keeps occurrence order, duplicates and unknown words',()=>{
  const p=prepareInput('lo/gos\nzzzz\nlo/gos','grc','original');
  const prepared={...p,records:[{input:'lo/gos'},{input:'zzzz'},{input:'lo/gos'}]};
  const r=analysisRecords(prepared,'lo/gos\n<NL>N lo/gos  masc nom sg</NL><NL>N lo/gos  masc nom sg</NL>\nlo/gos\n<NL>N lo/gos  other</NL>\n');
  assert.deepEqual(r.map(x=>x.analyses.length),[2,0,1]);assert.equal(r[2].analyses[0],'N lo/gos  other');
});
test('Perseus display prefix is a working form, not part of the dictionary headword',()=>{
  assert.deepEqual(parseAnalysis('V amo_,amo  pres ind act 1st sg\t\tconj1'),{type:'V',work:'amo_',lemma:'amo',fields:'pres ind act 1st sg\t\tconj1'});
  assert.equal(parseAnalysis('V katelu_/samen,katalu/w  aor ind act 1st pl').lemma,'katalu/w');
  assert.equal(parseAnalysis('N lo/gos  masc nom sg').lemma,'lo/gos');
});
test('edition join preserves case and numbered homonyms; never guesses a prefix or accent',()=>{
  assert.deepEqual(lemmaKey('gallus#1','lat'),{base:'gallus',number:1});
  assert.equal(matchingEntries(data('ls','gallus'),'gallus#1','lat')[0].id,'n19272');
  assert.deepEqual(matchingEntries(data('ls','Gallus'),'Gallus','lat').map(e=>e.number),[2,3,4]);
  assert.equal(matchingEntries(data('ls','Gallus'),'Gallus#1','lat').length,0);
  assert.equal(matchingEntries(data('ls','amo'),'a_mo','lat')[0].id,'n2280');
  assert.equal(matchingEntries(data('lsj','lo/gos'),'lo/gos','grc')[0].id,'n63772');
  assert.match(matchingEntries(data('lsj','a)/nqrwpos'),'a)/nqrwpos','grc')[0].preview.map(s=>s.text).join(''),/man,.*opp\.\s+gods,/s);
  assert.equal(matchingEntries(data('lsj','katalu/w'),'katalu/w','grc').length,1);
  assert.equal(matchingEntries(data('lsj','kata/-lu/w'),'kata/-lu/w','grc').length,0);
  assert.equal(matchingEntries(data('ls','amo'),'amo','lat')[0].preview.map(s=>s.text).join('').includes('to love'),true);
});
test('every complete dictionary shard matches its manifest; source identities are unique',()=>{
  const manifest=JSON.parse(readFileSync(new URL('../dictionaries/manifest.json',import.meta.url)));
  for(const [id,dictionary]of Object.entries(manifest.dictionaries)){
    let count=0;const identities=new Set();
    for(const file of dictionary.files){
      const bytes=readFileSync(new URL('../dictionaries/'+file.path,import.meta.url));
      assert.equal(bytes.length,file.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),file.sha256);
      const entries=JSON.parse(gunzipSync(bytes));
      for(const [key,rows]of Object.entries(entries)){
        assert.equal(file.path,id+'/'+dictionaryBucket(key)+'.json.gz');
        for(const row of rows){const identity=row.source+'#'+row.id;assert.equal(identities.has(identity),false);identities.add(identity);count++;assert.ok(row.html.length);}
      }
    }
    assert.equal(count,dictionary.entries);
  }
});
