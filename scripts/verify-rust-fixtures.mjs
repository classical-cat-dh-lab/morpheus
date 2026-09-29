import {runner,fingerprint} from './verify-rust.mjs';
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
const [reference,candidate,cTrace,rTrace,inputFile,output]=process.argv.slice(2);if(!output)throw Error('Usage: verify-rust-fixtures.mjs C RUST C_TRACE RUST_TRACE INPUT OUTPUT');
const c=await runner(reference),r=await runner(candidate),ct=await runner(cTrace,true),rt=await runner(rTrace,true);
const out=output+'.details';
const fixtures=JSON.parse(readFileSync(inputFile)).fixtures.map(x=>({...x,stdin:Buffer.from(x.stdin,'base64')}));
const report={fixtures:[],failures:[],events:0};
for(let i=0;i<fixtures.length;i++){
 const f=fixtures[i],a=await c(f),b=await r(f),x=await ct(f),y=await rt(f);const io=JSON.stringify(fingerprint(a))===JSON.stringify(fingerprint(b)),tr=x.trace.equals(y.trace),transparent=JSON.stringify(fingerprint(a))===JSON.stringify(fingerprint(x))&&JSON.stringify(fingerprint(b))===JSON.stringify(fingerprint(y));report.events+=x.trace.toString().split('\n').length-1;report.fixtures.push({id:f.id,argv:f.argv,stdin:f.stdin.toString('base64'),io,tr,transparent});
 if(!io||!tr||!transparent){report.failures.push(f.id);mkdirSync(out+'/adversarial-failures',{recursive:true});for(const[n,z]of[['c',x],['rust',y]]){writeFileSync(out+'/adversarial-failures/'+i+'-'+n+'.trace',z.trace);writeFileSync(out+'/adversarial-failures/'+i+'-'+n+'.stdout',z.stdout);}}
 if(i%100===0)console.log(i,report.failures.length,'failures');
}
writeFileSync(output,JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify({count:report.fixtures.length,failures:report.failures,events:report.events}));

if(report.failures.length)process.exitCode=1;
