/** Compare retry traversal against the linked original, without recompiling C. */
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {runner,fingerprint} from './verify-rust.mjs';
const [reference,rust,observed,inputFile,output]=process.argv.slice(2);
if(!output)throw Error('Usage: verify-frozen-checkpoints.mjs C_ARTIFACT RUST_TRACE OBSERVED_WASM CORPUS_OR_FIXTURES OUTPUT');
const sha=x=>createHash('sha256').update(x).digest('hex');
const original=readFileSync(join(reference,'morpheus.wasm'));
if(sha(original)!=='c91ddc0e424197e3d3cd4e4d396979367f0d37101f608bdd6e9205b841914273')throw Error('Unexpected C reference');
const expected=JSON.parse(readFileSync(new URL('../rust/evidence/retry-causality/observed.json',import.meta.url))).wasmSha256;
const wasmBinary=readFileSync(observed);
if(sha(wasmBinary)!==expected)throw Error('Use the exact observed.wasm emitted by diagnose-retry-memory.py');
const factory=(await import(pathToFileURL(join(resolve(reference),'morpheus.mjs')).href)).default;
const data=readFileSync(join(reference,'morpheus.data')),module=new WebAssembly.Module(wasmBinary);

// Same logical gk_word fields as the source checkpoints, excluding addresses,
// padding, retired bytes and timing. Layout is the pinned original Wasm ABI.
function word(memory,p){
 const u8=new Uint8Array(memory.buffer),u16=new Int16Array(memory.buffer),u32=new Uint32Array(memory.buffer),i32=new Int32Array(memory.buffer);
 const fields=['checkword:Gkword'];
 function text(at,n){let s='';for(let i=0;i<n&&u8[at+i];i++)s+=u8[at+i].toString(16).padStart(2,'0');fields.push(s);}
 function meta(at){let bits=u32[at>>>2],offset=0;for(const width of [3,4,4,3,3,6,2,4]){fields.push((bits>>>offset)&((1<<width)-1));offset+=width;}fields.push(u32[(at+4)>>>2],u32[(at+8)>>>2],u16[(at+12)>>>1],u32[(at+16)>>>2]);for(let i=0;i<12;i++)fields.push(u8[at+20+i]);text(at+32,21);}
 function gs(at){meta(at);text(at+53,60);}
 if(!p)fields.push('null');else{
  meta(p);const total=i32[(p+60)>>>2];fields.push(i32[(p+56)>>>2],total);
  for(const o of [64,704,764,824,884])text(p+o,60);
  for(const o of [124,240,356,472,588])gs(p+o);
  const analyses=u32[(p+948)>>>2];
  for(let j=0;j<total;j++){const a=analyses+j*1116;fields.push('analysis');meta(a);for(const o of [53,816,876,936,996,113,173])text(a+o,60);for(const o of [236,352,468,584,700])gs(a+o);}
 }
 return fields.join('|')+'\n';
}
async function frozen(input){
 let memory,offset=0;const trace=[],out=[],err=[];
 const m=await factory({noInitialRun:true,getPreloadedPackage:()=>data.buffer.slice(data.byteOffset,data.byteOffset+data.byteLength),
  instantiateWasm(imports,receive){
   imports.causality={snapshot:(tag,frame,p)=>{if(tag===21)trace.push(word(memory,p));},read:p=>p};
   const instance=new WebAssembly.Instance(module,imports);memory=instance.exports.memory;receive(instance);return instance.exports;
  },stdin:()=>offset<input.stdin.length?input.stdin[offset++]:null,stdout:b=>{if(b!==null)out.push(b);},stderr:b=>{if(b!==null)err.push(b);},preRun:[m=>{m.ENV.MORPHLIB='/morphlib';m.ENV.LC_ALL='C';m.ENV.TZ='UTC';m.FS.mkdir('/job');m.FS.chdir('/job');if(input.operation==='cruncher-file')m.FS.writeFile('input.words',input.stdin);}]
 });
 let termination;try{termination={kind:'exit',code:m.callMain([...input.argv,...(input.operation==='cruncher-file'?['input']:[])])};}catch(e){termination={kind:'trap',message:String(e)}}
 const files={};for(const n of ['input.morph','input.failed','input.stats'])if(m.FS.analyzePath(n).exists)files[n]=Buffer.from(m.FS.readFile(n));
 return{stdout:Buffer.from(out),stderr:Buffer.from(err),termination,files,trace:Buffer.from(trace.join(''))};
}
const raw=JSON.parse(readFileSync(inputFile)),jobs=[];
if(raw.fixtures)jobs.push(...raw.fixtures.map(x=>({...x,stdin:Buffer.from(x.stdin,'base64')})));
else for(const language of ['grc','lat'])for(let first=0;first<raw[language].length;first+=25)for(const operation of ['cruncher','cruncher-file'])jobs.push({id:`${language}:${first}:${operation}`,operation,argv:['-T',...(language==='lat'?['-L']:[])],stdin:Buffer.from(raw[language].slice(first,first+25).join('\n')+'\n')});
const c=await runner(reference),r=await runner(rust,true);
const report={checkpoint:'checkword entry (inlined at frozen function 21 entry)',originalSha256:sha(original),observedSha256:sha(wasmBinary),rustSha256:sha(readFileSync(join(rust,'morpheus.wasm'))),inputSha256:sha(readFileSync(inputFile)),groups:[],failures:[],events:0};
for(let i=0;i<jobs.length;i++){
 const input=jobs[i],a=await c(input),x=await frozen(input),y=await r(input);
 const rustTrace=Buffer.from(y.trace.toString().split('\n').filter(s=>s.startsWith('checkword:Gkword|')).map(s=>s+'\n').join(''));
 const transparent=JSON.stringify(fingerprint(a))===JSON.stringify(fingerprint(x)),io=JSON.stringify(fingerprint(a))===JSON.stringify(fingerprint(y)),tr=x.trace.equals(rustTrace),events=x.trace.toString().split('\n').length-1;
 report.groups.push({id:input.id,inputSha256:sha(input.stdin),io,transparent,tr,c:sha(x.trace),rust:sha(rustTrace),events});report.events+=events;
 if(!transparent||!io||!tr){report.failures.push(input.id);mkdirSync(output+'.failures',{recursive:true});writeFileSync(join(output+'.failures',`${i}-c.trace`),x.trace);writeFileSync(join(output+'.failures',`${i}-rust.trace`),rustTrace);}
 if(i%200===0)console.log(i,report.failures.length,'failures');
}
report.passed=report.failures.length===0;writeFileSync(output,JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify({groups:jobs.length,events:report.events,failures:report.failures}));if(!report.passed)process.exitCode=1;
