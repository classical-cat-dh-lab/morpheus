/** Raw Rust/C differential runner. Never substitutes C for a failed Rust run. */
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
const sha=b=>createHash('sha256').update(b).digest('hex');
export async function runner(directory,trace=false){
 const factory=(await import(pathToFileURL(join(resolve(directory),'morpheus.mjs')).href)).default;
 const data=readFileSync(join(directory,'morpheus.data')),wasmBinary=readFileSync(join(directory,'morpheus.wasm'));
 return async ({stdin,argv,operation='cruncher'})=>{
  const out=[],err=[];let offset=0;
  const m=await factory({noInitialRun:true,wasmBinary,getPreloadedPackage:()=>data.buffer.slice(data.byteOffset,data.byteOffset+data.byteLength),locateFile:name=>join(resolve(directory),name),stdin:()=>offset<stdin.length?stdin[offset++]:null,stdout:b=>{if(b!==null)out.push(b)},stderr:b=>{if(b!==null)err.push(b)},preRun:[m=>{m.ENV.MORPHLIB='/morphlib';m.ENV.LC_ALL='C';m.ENV.TZ='UTC';m.FS.mkdir('/job');m.FS.chdir('/job');if(trace)m.ENV.MORPH_TRACE_FILE='/job/trace';if(operation==='cruncher-file')m.FS.writeFile('input.words',stdin)}]});
  let termination;try{termination={kind:'exit',code:m.callMain([...argv,...(operation==='cruncher-file'?['input']:[])])}}catch(e){termination={kind:'trap',message:String(e)}}
  const files={};for(const n of ['input.morph','input.failed','input.stats'])if(m.FS.analyzePath(n).exists)files[n]=Buffer.from(m.FS.readFile(n));
  return {stdout:Buffer.from(out),stderr:Buffer.from(err),termination,files,...(trace?{trace:m.morphTrace?Buffer.from(m.morphTrace.join('')):m.FS.analyzePath('trace').exists?Buffer.from(m.FS.readFile('trace')):Buffer.alloc(0)}:{})};
 }
}
export function fingerprint(r){return{stdout:sha(r.stdout),stderr:sha(r.stderr),termination:r.termination,files:Object.fromEntries(Object.entries(r.files).map(([k,v])=>[k,sha(v)]))}}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href){
 const [reference,candidate,inputs,receipt,...tracePaths]=process.argv.slice(2);if(!receipt)throw Error('Usage: verify-rust.mjs C_ARTIFACT RUST_ARTIFACT CORPUS RECEIPT [C_TRACE RUST_TRACE]');
 const corpus=JSON.parse(readFileSync(inputs)),c=await runner(reference),r=await runner(candidate);const ct=tracePaths.length?await runner(tracePaths[0],true):null,rt=tracePaths.length?await runner(tracePaths[1],true):null;
 const report={recorded:new Date().toISOString(),corpus:corpus.provenance,artifacts:{c:sha(readFileSync(join(reference,'morpheus.wasm'))),rust:sha(readFileSync(join(candidate,'morpheus.wasm')))},groups:[],failures:[],traceEvents:0};
 for(const language of ['grc','lat']){
  const words=corpus[language];for(let i=0;i<words.length;i+=25){for(const operation of ['cruncher','cruncher-file']){
   const input={stdin:Buffer.from(words.slice(i,i+25).join('\n')+'\n'),argv:['-T',...(language==='lat'?['-L']:[])],operation};const a=await c(input),b=await r(input),af=fingerprint(a),bf=fingerprint(b);const equal=JSON.stringify(af)===JSON.stringify(bf);const row={language,first:i,count:Math.min(25,words.length-i),operation,inputSha256:sha(input.stdin),equal,c:af,rust:bf};
   if(ct){const x=await ct(input),y=await rt(input);row.trace={c:sha(x.trace),rust:sha(y.trace),events:x.trace.toString().split('\n').length-1,cTransparent:JSON.stringify(fingerprint(x))===JSON.stringify(af),rustTransparent:JSON.stringify(fingerprint(y))===JSON.stringify(bf)};report.traceEvents+=row.trace.events;if(row.trace.c!==row.trace.rust||!row.trace.cTransparent||!row.trace.rustTransparent){const dir=receipt+'.failures';mkdirSync(dir,{recursive:true});for(const [n,z]of [['c',x],['rust',y]])writeFileSync(join(dir,`${language}-${i}-${operation}-${n}.trace`),z.trace);report.failures.push({language,i,operation,type:'trace'});}}
   report.groups.push(row);if(!equal){report.failures.push({language,i,operation,type:'io'});const dir=receipt+'.failures';mkdirSync(dir,{recursive:true});for(const[n,z]of[['c',a],['rust',b]]){writeFileSync(join(dir,`${language}-${i}-${operation}-${n}.stdout`),z.stdout);writeFileSync(join(dir,`${language}-${i}-${operation}-${n}.stderr`),z.stderr);}}}
   if(i%250===0)console.log(language,i,report.failures.length,'failures');}
 }
 report.passed=report.failures.length===0;writeFileSync(receipt,JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify({groups:report.groups.length,failures:report.failures,traceEvents:report.traceEvents}));if(!report.passed)process.exitCode=1;
}
