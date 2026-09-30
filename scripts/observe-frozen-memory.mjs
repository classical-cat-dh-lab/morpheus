/** Read frozen Wasm linear memory; intervention is explicit and single-byte only. */
import {readFileSync,writeFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {resolve,basename} from 'node:path';
import {createHash} from 'node:crypto';
import {fingerprint} from './verify-rust.mjs';
const [referencePath,binaryPath,outputPath,watchRaw='',injectRaw='',inputPath]=process.argv.slice(2);
if(!outputPath)throw Error('Usage: observe-frozen-memory.mjs REFERENCE ORIGINAL_OR_WASM OUTPUT [WATCH_ADDRESS] [INJECT_BYTE] [INPUT_FILE]');
const reference=pathToFileURL(resolve(referencePath)+'/');
const factory=(await import(new URL('morpheus.mjs',reference))).default;
const data=readFileSync(new URL('morpheus.data',reference));
const binary=binaryPath==='original'?'original':basename(binaryPath);
const watch=watchRaw.split(',').filter(Boolean).map(Number),inject=injectRaw===''?null:Number(injectRaw);
if(watch.some(x=>!Number.isSafeInteger(x)||x<0)||inject!==null&&(!Number.isInteger(inject)||inject<0||inject>255))throw Error('Invalid byte address or injected value');
const input=inputPath?readFileSync(inputPath):Buffer.from('dictatoriis\nobiectivis\n');
let injections=[],memory,events=[],writes=[],lastWrite=new Map(),pending=new Map(),sequence=0,position=0;
let out=[],err=[];
const bytes=()=>new Uint8Array(memory.buffer);
const text=p=>{const a=bytes();let e=p;while(a[e]&&e<p+60)e++;return Buffer.from(a.slice(p,e)).toString('latin1');};
const hex=(p,n)=>Buffer.from(bytes().slice(p,p+n)).toString('hex');
function snap(kind,frame,word,extra={}){return{kind,sequence:++sequence,frame,raw:text(word+764),word:text(word+824),work:hex(frame,60),save:hex(frame+64,60),...extra};}
const wasmBinary=readFileSync(binary==='original'?new URL('morpheus.wasm',reference):resolve(binaryPath));
const wasmModule=new WebAssembly.Module(wasmBinary);
const m=await factory({noInitialRun:true,getPreloadedPackage:()=>data.buffer.slice(data.byteOffset,data.byteOffset+data.byteLength),
 instantiateWasm(imports,receive){
  imports.causality={snapshot:(tag,frame,word,sp)=>{if(tag===206){const source=text(word);if(watch.some(x=>x>=frame&&x<=frame+source.length))events.push({kind:'strcpy',sequence:++sequence,dest:frame,source,sp,stack:new Error().stack.split('\n').filter(x=>x.includes('wasm://')).join('\n')});}else events.push(snap('entry'+tag,frame,word,{sp}));},read:(p,frame,word)=>{const a=bytes(),addr=p+2,before=a[addr],tail=a[p+1]===0;if(tail&&inject!==null&&!injections.length&&(watch.length===0||watch.includes(addr))){a[addr]=inject;injections.push({address:addr,before,after:inject});}events.push(snap('read',frame,word,{offset:p-frame,address:addr,next:a[p+1],following:a[addr],before,tail,lastWrite:lastWrite.get(addr)??null}));return p;}};
  // Binaryen's observation hooks return the original pointer/value unchanged.
  // An unexpected import is an error, never an invented runtime implementation.
  const hooks=new Set(['load_ptr','store_ptr','memory_grow_pre','memory_grow_post',...['i32','i64','f32','f64'].flatMap(t=>['load_val_'+t,'store_val_'+t])]);
  for(const {module,name} of WebAssembly.Module.imports(wasmModule)){
   if(imports[module]?.[name])continue;
   if(module!=='env'||!hooks.has(name))throw Error('Unexpected instrumentation import: '+module+'.'+name);
   imports.env[name]=(...args)=>args.at(-1);
  }
  imports.env.store_ptr=(id,n,offset,p)=>{const item={id,n,address:p+offset,sequence:++sequence};const a=pending.get(id)??[];a.push(item);pending.set(id,a);return p;};
  for(const type of ['i32','i64','f32','f64'])imports.env['store_val_'+type]=(id,value)=>{const item=pending.get(id)?.pop();if(item&&item.address<106704){const r={...item,type,value:String(value)};for(let i=0;i<item.n;i++)lastWrite.set(item.address+i,r);if(watch.some(x=>x>=item.address&&x<item.address+item.n))writes.push({...r,stack:new Error().stack.split('\n').filter(x=>x.includes('wasm://')).join('\n')});}return value;};
  const instance=new WebAssembly.Instance(wasmModule,imports);memory=instance.exports.memory;receive(instance);return instance.exports;
 },stdin:()=>position<input.length?input[position++]:null,stdout:b=>{if(b!==null)out.push(b);},stderr:b=>{if(b!==null)err.push(b);},preRun:[m=>{m.ENV.MORPHLIB='/morphlib';m.ENV.LC_ALL='C';m.ENV.TZ='UTC';m.FS.mkdir('/job');m.FS.chdir('/job');m.FS.writeFile('input.words',input);}]
});
let termination;try{termination={kind:'exit',code:m.callMain(['-T','-L','input'])};}catch(e){termination={kind:'trap',message:String(e)}}
const files={};for(const n of ['input.morph','input.failed','input.stats'])if(m.FS.analyzePath(n).exists)files[n]=Buffer.from(m.FS.readFile(n));
const result={stdout:Buffer.from(out),stderr:Buffer.from(err),termination,files};
const report={binary,input:input.toString('latin1'),inject,injections,platform:{node:process.version,system:process.platform,architecture:process.arch},wasmSha256:createHash('sha256').update(wasmBinary).digest('hex'),fingerprint:fingerprint(result),memorySha256:createHash('sha256').update(bytes()).digest('hex'),events,writes};
writeFileSync(outputPath,JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({output:basename(outputPath),events:events.length,writes:writes.length,injections,memorySha256:report.memorySha256}));
