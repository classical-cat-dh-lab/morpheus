import {createServer} from 'node:http';
import {readFileSync,statSync} from 'node:fs';
import {resolve,extname,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
export const types={'.html':'text/html;charset=utf-8','.mjs':'text/javascript','.js':'text/javascript','.json':'application/json','.css':'text/css','.wasm':'application/wasm','.data':'application/octet-stream','.woff2':'font/woff2','.svg':'image/svg+xml','.png':'image/png','.webmanifest':'application/manifest+json','.gz':'application/gzip','.txt':'text/plain'};
export const csp="default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; worker-src 'self'; style-src 'self'; font-src 'self'; img-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'";
export function serve(root, intercept) {
  return createServer(async(req,res)=>{
    try {
      if(intercept && await intercept(req,res))return;
      const url=new URL(req.url,'http://localhost'),path=decodeURIComponent(url.pathname);
      let file=resolve(root,'.'+path);
      if(file!==root&&!file.startsWith(root+sep))throw Error('Outside root');
      if(statSync(file).isDirectory())file=resolve(file,'index.html');
      const body=readFileSync(file);
      res.writeHead(200,{'Content-Type':types[extname(file)]??'application/octet-stream','Content-Length':body.length,'Cache-Control':'no-store','Content-Security-Policy':csp,'X-Content-Type-Options':'nosniff'});
      res.end(body);
    } catch {res.writeHead(404,{'Content-Type':'text/plain'}).end('Not found');}
  });
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const root=fileURLToPath(new URL('../site',import.meta.url)),port=Number(process.argv[2]??4173);
  serve(root).listen(port,'127.0.0.1',()=>console.log(`Morph preview: http://127.0.0.1:${port}`));
}
