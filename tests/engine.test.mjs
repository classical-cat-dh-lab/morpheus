import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {runEngine,validateRequest,identity,protocol} from '../engine/runtime.mjs';
const request=(word,extra={})=>({protocol,id:'test',profile:identity.profile,implementation:identity.implementation,data:identity.data,operation:'cruncher',argv:['-T'],stdin:new TextEncoder().encode(word),...extra});
const options={locateFile:name=>fileURLToPath(new URL('../engine/'+name,import.meta.url))};
test('shipped engine/data identities match the artifact bytes',()=>{
  for(const[name,entry]of Object.entries(identity.assets)){const bytes=readFileSync(new URL('../engine/'+name,import.meta.url));assert.equal(bytes.length,entry.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),entry.sha256);}
  const qualified=JSON.parse(readFileSync(new URL('../rust/evidence/qualification.json',import.meta.url)));
  assert.equal(identity.implementation,qualified.implementation);assert.equal(identity.data,qualified.data);
  for(const file of qualified.artifacts.filter(f=>f.path.startsWith('artifact/')&&!f.path.endsWith('/cli.mjs'))){
    const bytes=readFileSync(new URL('../engine/'+file.path.slice('artifact/'.length),import.meta.url));
    assert.equal(bytes.length,file.bytes,file.path);assert.equal(createHash('sha256').update(bytes).digest('hex'),file.sha256,file.path);
  }
});
test('fresh Greek and Latin runs retain independent state and exact bytes',async()=>{
  const greek=await runEngine(request('lo/gos\n'),options);
  assert.equal(new TextDecoder().decode(greek.stdout),'lo/gos\n<NL>N lo/gos  masc nom sg\t\t\tos_ou</NL>\n');assert.deepEqual(greek.termination,{kind:'exit',code:0});
  const latin=await runEngine(request('amo\n',{argv:['-T','-L']}),options);
  assert.equal(new TextDecoder().decode(latin.stdout),'amo\n<NL>V amo_,amo  pres ind act 1st sg\t\t\tconj1,are_vb</NL>\n');
  const again=await runEngine(request('lo/gos\n'),options);assert.deepEqual(again.stdout,greek.stdout);
  const esse=await runEngine(request('esse\n',{argv:['-T','-L']}),options);
  assert.deepEqual(esse.termination,{kind:'exit',code:0});
  assert.match(new TextDecoder().decode(esse.stdout),/esse[\s\S]*sum/);
});
test('original file workflow preserves generated channels',async()=>{
  const result=await runEngine(request('lo/gos\nzzzz\n',{operation:'cruncher-file'}),options);
  assert.deepEqual(result.termination,{kind:'exit',code:0});
  assert.match(new TextDecoder().decode(result.files['input.morph']),/lo\/gos/);
  assert.equal(new TextDecoder().decode(result.files['input.failed']),'zzzz\n');
  assert.match(new TextDecoder().decode(result.files['input.stats']),/FINAL:  words 2, analyzed 1/);
});
test('host boundary rejects unsupported identity/options and unsafe input instead of truncating',()=>{
  for(const req of [request('x'.repeat(49)),request('λ'),request('a\0'),request('a',{profile:'future'}),request('a',{argv:['-I']}),request('a',{argv:['-e']}),request('a',{argv:['-o','/tmp/output']}),request('a\n'.repeat(1001))])assert.throws(()=>validateRequest(req));
});
