/** Differential batch/file verification. Corpus inputs are supplied separately. */
import {readFileSync,writeFileSync,mkdtempSync,rmSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {tmpdir} from 'node:os';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {runEngine,identity,protocol} from '../engine/runtime.mjs';
const [buildArg,inputArg,outputArg]=process.argv.slice(2);
if(!outputArg)throw Error('Usage: node scripts/compare-corpus.mjs BUILD INPUT_JSON RECEIPT_JSON');
const build=resolve(buildArg),corpus=JSON.parse(readFileSync(inputArg));
const sha=bytes=>createHash('sha256').update(bytes).digest('hex');
const report={schema:1,recorded:new Date().toISOString(),identity,corpus:corpus.provenance,groups:[],failures:[]};
const options={locateFile:name=>fileURLToPath(new URL('../engine/'+name,import.meta.url))};
for(const language of ['grc','lat']){
  const words=corpus[language];
  for(let i=0;i<words.length;i+=25){
    const stdin=Buffer.from(words.slice(i,i+25).join('\n')+'\n'),argv=['-T',...(language==='lat'?['-L']:[])];
    for(const operation of ['cruncher','cruncher-file']){
      const directory=mkdtempSync(join(tmpdir(),'morph-corpus-'));
      try{
        if(operation==='cruncher-file')writeFileSync(join(directory,'input.words'),stdin);
        const native=spawnSync(join(build,'native/src/anal/cruncher'),[...argv,...(operation==='cruncher-file'?['input']:[])],{cwd:directory,input:stdin,timeout:20000,maxBuffer:16777216,env:{...process.env,MORPHLIB:join(build,'native/stemlib'),LC_ALL:'C',TZ:'UTC'}});
        if(native.error)throw native.error;
        const result=await runEngine({protocol,id:`${language}:${i}:${operation}`,profile:identity.profile,implementation:identity.implementation,data:identity.data,operation,argv,stdin},options);
        const a={stdout:sha(native.stdout),stderr:sha(native.stderr),termination:native.signal?{kind:'signal',signal:native.signal}:{kind:'exit',code:native.status},files:{}};
        const b={stdout:sha(result.stdout),stderr:sha(result.stderr),termination:result.termination,files:{}};
        for(const name of Object.keys(result.files).sort()){a.files[name]=sha(readFileSync(join(directory,name)));b.files[name]=sha(result.files[name]);}
        const equal=JSON.stringify(a)===JSON.stringify(b);
        const row={language,first:i,count:Math.min(25,words.length-i),operation,stdinSha256:sha(stdin),equal,native:a,wasm:b};report.groups.push(row);
        if(!equal)report.failures.push({language,first:i,operation});
      }finally{rmSync(directory,{recursive:true,force:true});}
    }
  }
  console.log(language,words.length,'input forms checked in both modes');
}
report.passed=report.failures.length===0;writeFileSync(outputArg,JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({groups:report.groups.length,failures:report.failures}));
if(!report.passed)process.exitCode=1;
