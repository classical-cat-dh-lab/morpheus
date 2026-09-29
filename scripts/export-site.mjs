import {readFileSync,writeFileSync,readdirSync,mkdirSync,cpSync,rmSync} from 'node:fs';
import {resolve,relative,dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {gzipSync} from 'node:zlib';
import {csp} from './serve.mjs';
const root=fileURLToPath(new URL('../',import.meta.url)),site=resolve(root,'site');
const sha=data=>createHash('sha256').update(data).digest('hex');
function files(directory){return readdirSync(directory,{withFileTypes:true}).flatMap(e=>e.isDirectory()?files(resolve(directory,e.name)):[resolve(directory,e.name)]).sort();}
function archive(base,paths,destination){
  const chunks=[];
  for(const path of [...paths].sort()){
    const name=relative(base,path);if(name.split('/').some(p=>['.DS_Store','__pycache__'].includes(p)))continue;
    const data=readFileSync(path),header=Buffer.alloc(512);
    const put=(value,offset,size)=>{const bytes=Buffer.from(value);if(bytes.length>size)throw Error('Archive field too long: '+name);bytes.copy(header,offset);};
    const octal=(value,offset,size)=>put(value.toString(8).padStart(size-1,'0')+'\0',offset,size);
    let leaf=name,prefix='';if(Buffer.byteLength(name)>100){const split=name.lastIndexOf('/');leaf=name.slice(split+1);prefix=name.slice(0,split);}
    put(leaf,0,100);octal(0o644,100,8);octal(0,108,8);octal(0,116,8);octal(data.length,124,12);octal(0,136,12);put('        ',148,8);put('0',156,1);put('ustar\0',257,6);put('00',263,2);put('root',265,32);put('root',297,32);put(prefix,345,155);
    put([...header].reduce((a,b)=>a+b,0).toString(8).padStart(6,'0')+'\0 ',148,8);
    chunks.push(header,data,Buffer.alloc((512-data.length%512)%512));
  }
  chunks.push(Buffer.alloc(1024));writeFileSync(destination,gzipSync(Buffer.concat(chunks),{level:9}));
}
const design=JSON.parse(readFileSync(resolve(root,'design.lock.json')));
for(const [path,hash]of Object.entries(design.files))if(sha(readFileSync(resolve(root,design.path,path)))!==hash)throw Error('Changed design resource: '+path);
const inputs=['browser','engine','dictionaries','docs','licenses'].flatMap(p=>files(resolve(root,p))).concat(['README.md','LICENSE','source.lock.json','design.lock.json','dictionary-sources.lock.json','package.json'].map(p=>resolve(root,p)));
const version=JSON.parse(readFileSync(resolve(root,'package.json'))).version;
const id=version+'-'+sha(inputs.map(p=>relative(root,p)+'\0'+sha(readFileSync(p))).join('\n')).slice(0,16),base='/releases/'+id;
rmSync(site,{recursive:true,force:true});mkdirSync(site,{recursive:true});
const put=(name,data)=>{const path=resolve(site,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,data);};
for(const directory of ['browser','engine','dictionaries'])cpSync(resolve(root,directory),resolve(site,'.'+base,directory),{recursive:true});
const template=readFileSync(resolve(root,'browser/index.html'),'utf8');
const home=template.replaceAll('__BASE__',base).replaceAll('__RELEASE__',id);put('index.html',home);
for(const page of ['about','licenses']){
  const html=readFileSync(resolve(root,'docs',page+'.html'),'utf8').replaceAll('__BASE__',base);
  put(page+'/index.html',html);
}
for(const file of readdirSync(resolve(root,'licenses')))cpSync(resolve(root,'licenses',file),resolve(site,'licenses',file));
put('sw.js',readFileSync(resolve(root,'browser/sw.js')));
put('icon.svg',readFileSync(resolve(root,'browser/icon.svg')));
put('manifest.webmanifest',JSON.stringify({id:'/',name:'Morph — Greek & Latin morphology',short_name:'Morph',start_url:'/',scope:'/',display:'standalone',background_color:'#F4F5F0',theme_color:'#F4F5F0',icons:[192,512].map(size=>({src:base+`/browser/icon-${size}.png`,sizes:`${size}x${size}`,type:'image/png',purpose:'any'}))},null,2));
put('_headers',`/*\n  Content-Security-Policy: ${csp}\n  X-Content-Type-Options: nosniff\n  Referrer-Policy: strict-origin-when-cross-origin\n/releases/*\n  Cache-Control: public, max-age=31536000, immutable\n`+['/','/about/*','/licenses/*','/release.json','/manifest.webmanifest','/sw.js'].map(path=>`${path}\n  Cache-Control: no-cache\n`).join(''));
const manifest={schema:1,id,version,files:files(site).filter(p=>!['_headers','sw.js'].includes(relative(site,p))).map(p=>({url:relative(site,p)==='index.html'?'/':'/'+relative(site,p).replace(/\/index\.html$/,'/'),bytes:readFileSync(p).length,sha256:sha(readFileSync(p))}))};
put('.'+base+'/release.json',JSON.stringify(manifest,null,2)+'\n');
put('release.json',JSON.stringify(manifest,null,2)+'\n');
const artifacts=resolve(root,process.env.MORPH_RELEASE_OUTPUT??'build/releases',version);mkdirSync(artifacts,{recursive:true});
// Source delivery includes the frozen original archive and complete build inputs.
archive(root,['README.md','LICENSE','.gitignore','package.json','source.lock.json','design.lock.json','dictionary-sources.lock.json','CITATION.cff','wrangler.jsonc'].map(p=>resolve(root,p)).concat(['.github','browser','engine','dictionaries','scripts','tests','vendor','licenses','docs','evidence','patches','rust'].flatMap(p=>files(resolve(root,p)))),resolve(artifacts,`morph-${version}-source.tar.gz`));
// The static export has no recursive copy of its own downloads.
archive(site,files(site).filter(p=>!relative(site,p).startsWith('downloads/')),resolve(artifacts,`morph-${version}-static.tar.gz`));
console.log(JSON.stringify({release:id,files:manifest.files.length,offlineBytes:manifest.files.reduce((n,f)=>n+f.bytes,0),site}));
