import {resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,rmSync} from 'node:fs';
import assert from 'node:assert/strict';
import {serve} from './serve.mjs';

const root=fileURLToPath(new URL('../',import.meta.url)),output=resolve(root,'build/browser');mkdirSync(output,{recursive:true});
const {chromium,webkit,firefox}=await import(process.argv[2]?pathToFileURL(resolve(process.argv[2])).href:'playwright');
let unavailable=false,failData=false,holdData=false,releaseHold;
const server=serve(resolve(root,'site'),async(req,res)=>{
  if(unavailable){req.socket.destroy();return true;}
  if(req.url.endsWith('/morpheus.data')){
    if(failData){res.writeHead(503).end();return true;}
    if(holdData)await new Promise(r=>releaseHold=r);
  }
  return false;
});
await new Promise(r=>server.listen(0,'127.0.0.1',r));const origin='http://127.0.0.1:'+server.address().port;
const report={recorded:new Date().toISOString(),release:JSON.parse(readFileSync(resolve(root,'site/release.json'))).id,browsers:[]};
async function until(page,selector,text){await page.waitForFunction(({selector,text})=>document.querySelector(selector)?.textContent.includes(text),{selector,text},{timeout:60000});}
async function lookup(page,input){await page.locator('#input').fill(input);await page.locator('#analyze').click();await until(page,'#status','Complete:');}
const matrix={chromium,webkit,firefox};
try {
  for(const name of (process.env.MORPH_BROWSERS??'chromium,webkit,firefox').split(',')){
    const browser=await matrix[name].launch({headless:true});const row={name,version:browser.version(),checks:[]};report.browsers.push(row);
    const context=await browser.newContext(),page=await context.newPage();const errors=[];page.on('pageerror',e=>errors.push(String(e)));
    try {
      await page.goto(origin);const passage='λόγος, ἄνθρωπος.\n\nλόγος — λύω!';await lookup(page,passage);
      assert.equal(await page.locator('#passage-text').textContent(),passage);assert.equal(await page.locator('.passage-word').count(),4);
      await until(page,'#passage-detail','computation, reckoning');assert.match(await page.locator('#passage-detail').innerText(),/masc nom sg/);
      await page.locator('.passage-word').nth(1).click();await until(page,'#passage-detail','man,');assert.equal(await page.locator('.passage-word[aria-pressed=true]').textContent(),'ἄνθρωπος');
      await page.locator('.passage-word').nth(2).focus();await page.keyboard.press('Enter');await until(page,'#passage-detail','computation, reckoning');assert.equal(await page.locator('.passage-word[aria-pressed=true]').getAttribute('data-index'),'2');
      await page.locator('.dictionary-full>summary').first().click();await until(page,'.dictionary-body','λόγος');assert.equal(await page.locator('.dictionary-body script,.dictionary-body a,.dictionary-body img').count(),0);
      await page.locator('#view-list').click();assert.equal(await page.locator('#passage-view').isVisible(),false);await page.locator('.result').first().locator('summary').click();await until(page,'#list-view','computation, reckoning');
      await page.locator('#view-list').focus();await page.keyboard.press('ArrowLeft');assert.equal(await page.locator('#passage-view').isVisible(),true);
      row.checks.push('Exact passage, repeated occurrence selection, keyboard/tabs, full Greek entry and morphology');
      await page.locator('#language').selectOption('lat');await lookup(page,'arma virumque canō');assert.match(await page.locator('#status').textContent(),/3 of 3/);await page.locator('.passage-word').nth(2).click();await until(page,'#passage-detail','to produce melodious sounds');row.checks.push('Latin Unicode/macron adaptation and Lewis & Short definitions');
      await page.locator('.options>summary').click();await page.locator('#show-raw').check();await page.locator('#raw>summary').click();assert.equal(await page.locator('#delivered').textContent(),'arma\nvirumque\ncano\n');assert.match(await page.locator('#stdout').textContent(),/<NL>/);row.checks.push('Transparent delivered bytes and untouched raw output');
      const downloadPromise=page.waitForEvent('download');await page.locator('#download-record').click();const download=await downloadPromise;await download.saveAs(resolve(output,name+'-record.json'));row.checks.push('Downloadable execution record');
      await page.locator('#language').selectOption('grc');await page.locator('#mode').selectOption('original');await lookup(page,'lo/gos');assert.match(await page.locator('#status').textContent(),/1 of 1/);
      await page.locator('#input').fill('x'.repeat(49));await page.locator('#analyze').click();await until(page,'#status','48');assert.equal(await page.locator('#analyze').isEnabled(),true);row.checks.push('Original input mode and explicit length rejection');
      await lookup(page,'lu/w');await until(page,'#results','LSJ');await page.locator('#clear').click();assert.equal(await page.locator('.result').count(),0);assert.equal(await page.locator('#input').inputValue(),'');row.checks.push('New lookup clears results and cancels stale work');
      await page.locator('#mode').selectOption('unicode');await lookup(page,'οἶδα');await until(page,'#results','LSJ');
      for(const width of [320,390,768,1280]){await page.setViewportSize({width,height:900});assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true);}
      await page.screenshot({path:resolve(output,name+'-light.png'),fullPage:true});await page.locator('#theme-toggle').click();assert.equal(await page.locator('html').getAttribute('data-theme'),'dark');await page.screenshot({path:resolve(output,name+'-dark.png'),fullPage:true});row.checks.push('320–1280px layout and light/dark themes');
      await page.waitForFunction(()=>!document.querySelector('#offline-save').disabled);await page.locator('#offline-save').click();await until(page,'#offline-status','Ready offline');
      unavailable=true;await page.reload();await lookup(page,'ἀρετή');await until(page,'#results','goodness');await page.locator('#language').selectOption('lat');await lookup(page,'amo');await until(page,'#results','to love');await page.locator('#language').selectOption('grc');await page.goto(origin+'/about/');assert.equal(await page.locator('h1').textContent(),'About Morph');await page.goto(origin+'/licenses/');assert.equal(await page.locator('h1').textContent(),'Licenses and credits');row.checks.push('Cold offline reload, real lookup and document navigation');unavailable=false;
      await page.goto(origin);assert.equal(await page.locator('html').getAttribute('data-theme'),'dark');row.checks.push('Shared persistent theme on application and information pages');
      if(name==='chromium'){
        const sw=context.serviceWorkers()[0];
        await sw.evaluate(()=>{self.originalCachePut=Cache.prototype.put;Cache.prototype.put=function(){return Promise.reject(new DOMException('Quota simulation','QuotaExceededError'));};});
        await page.locator('#offline-save').click();await until(page,'#offline-status','failed');
        await sw.evaluate(()=>{Cache.prototype.put=self.originalCachePut;delete self.originalCachePut;});
        unavailable=true;await page.reload();await lookup(page,'λόγος');unavailable=false;row.checks.push('Storage quota failure preserves the previous complete offline edition');
        await until(page,'#offline-status','Ready offline');failData=true;await page.locator('#offline-save').click();await until(page,'#offline-status','failed');failData=false;unavailable=true;await page.reload();await lookup(page,'ἄνθρωπος');unavailable=false;row.checks.push('Failed update retains complete working offline edition');
        await page.reload();await page.waitForFunction(()=>!document.querySelector('#offline-save').disabled);holdData=true;await page.locator('#offline-save').click();await page.waitForTimeout(500);await page.locator('#offline-cancel').click();await until(page,'#offline-status','failed');holdData=false;releaseHold?.();row.checks.push('Cancelled download restores controls and preserves previous edition');
        await page.evaluate(async()=>{const request=indexedDB.open('morph-offline');const db=await new Promise(r=>request.onsuccess=()=>r(request.result));const active=await new Promise(r=>{const q=db.transaction('state').objectStore('state').get('active');q.onsuccess=()=>r(q.result)});db.close();const cache=await caches.open(active.cache);const key=(await cache.keys()).find(x=>x.url.endsWith('/morpheus.data'));await cache.put(key,new Response('damaged'));});
        await page.reload();await until(page,'#offline-status','No complete');await page.locator('#offline-save').click();await until(page,'#offline-status','Ready offline');unavailable=true;await page.reload();await lookup(page,'λόγος');unavailable=false;row.checks.push('Corrupt offline data is detected, repaired and used by a real offline lookup');
        const broken=await browser.newContext({serviceWorkers:'block'}),bp=await broken.newPage();
        await bp.route('**/dictionaries/*/*.json.gz',route=>route.fulfill({status:200,body:'damaged',contentType:'application/gzip'}));
        await bp.goto(origin);await lookup(bp,'λόγος');await until(bp,'#results','integrity check failed');assert.match(await bp.locator('#results').innerText(),/masc nom sg/);
        await bp.unroute('**/dictionaries/*/*.json.gz');await bp.getByRole('button',{name:'Retry dictionary'}).click();await until(bp,'#results','computation, reckoning');
        await bp.locator('#language').selectOption('lat');await lookup(bp,'gallus');await until(bp,'#results','a cock');assert.equal(await bp.locator('.dictionary-entry').count(),1);
        await broken.close();row.checks.push('Dictionary corruption is distinct from no match; retry restores meanings without changing morphology');
        const isolated=await browser.newContext(),p=await isolated.newPage();await p.addInitScript(()=>Object.defineProperty(window,'indexedDB',{get(){throw Error('Denied for test')}}));await p.goto(origin);await lookup(p,'λόγος');await p.locator('#theme-toggle').click();assert.equal(await p.locator('html').getAttribute('data-theme'),'dark');await isolated.close();row.checks.push('Denied preferences do not block analysis or theme');
        const stopped=await browser.newContext(),q=await stopped.newPage();await q.goto(origin);holdData=true;await q.locator('#input').fill('λόγος');await q.locator('#analyze').click();await q.waitForFunction(()=>!document.querySelector('#cancel').hidden);await q.locator('#cancel').click();await until(q,'#status','stopped');holdData=false;releaseHold?.();await lookup(q,'ἄνθρωπος');await stopped.close();row.checks.push('Cancellation during engine loading terminates the worker and a fresh query succeeds');
        assert.equal(await page.evaluate(async()=>{let n=0;for(const key of await caches.keys()){const r=await(await caches.open(key)).match('/_morph/manifest');if(r)n++;}return n;}),1);row.checks.push('Repeated downloads do not retain duplicate complete copies of the same release');
      }
      assert.deepEqual(errors,[]);row.checks.push('No uncaught page errors');console.log(name,'passed',row.checks.length);
    }catch(error){row.failure=String(error);console.error(name,row.failure);process.exitCode=1;}
    finally{unavailable=false;failData=false;holdData=false;releaseHold?.();await context.close();await browser.close();writeFileSync(resolve(output,'receipt.json'),JSON.stringify(report,null,2)+'\n');}
    const profile=mkdtempSync(resolve(root,'build/browser-profile-'));let persistent;
    try{
      persistent=await matrix[name].launchPersistentContext(profile,{headless:true});
      let page=await persistent.newPage();await page.goto(origin);await page.waitForFunction(()=>!document.querySelector('#offline-save').disabled);await page.locator('#offline-save').click();await until(page,'#offline-status','Ready offline');
      await persistent.close();persistent=undefined;unavailable=true;
      persistent=await matrix[name].launchPersistentContext(profile,{headless:true});page=await persistent.newPage();await page.goto(origin);await lookup(page,'ἀρετή');await until(page,'#results','goodness');await page.locator('#language').selectOption('lat');await lookup(page,'amo');await until(page,'#results','to love');
      row.checks.push('Browser process terminated and relaunched with no reachable origin; real lookup succeeds');console.log(name,'persistent cold restart passed');
    }catch(error){row.restartFailure=String(error);console.error(name,row.restartFailure);process.exitCode=1;}
    finally{unavailable=false;await persistent?.close();rmSync(profile,{recursive:true,force:true});writeFileSync(resolve(output,'receipt.json'),JSON.stringify(report,null,2)+'\n');}
  }
}finally{server.closeAllConnections();await new Promise(r=>server.close(r));}
