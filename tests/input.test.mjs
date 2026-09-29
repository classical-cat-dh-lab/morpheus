import test from 'node:test';
import assert from 'node:assert/strict';
import {toBeta,fromBeta,prepareInput} from '../browser/input.mjs';

test('Greek conversion preserves letters, breathings, accents and capitalization',()=>{
  for(const [greek,beta]of [['λόγος','lo/gos'],['ἄνθρωπος','a)/nqrwpos'],['Ἥρα','*(/hra'],['τῷ','tw=|'],['οἶδα','oi)=da'],['ἀλλ᾽',"a)ll'"],['αβγδεζηθικλμνξοπρστυφχψω','abgdezhqiklmncoprstufxyw']])assert.equal(toBeta(greek),beta);
  assert.equal(toBeta('ἄνθρωπος'.normalize('NFD')),'a)/nqrwpos');
  assert.equal(fromBeta('lo/gos'),'λόγος');assert.equal(fromBeta('*(/hra'),'Ἥρα');
  assert.throws(()=>toBeta('ϡ'),/Unsupported/);assert.throws(()=>toBeta('amor'),/Unsupported/);
  assert.throws(()=>toBeta('\u0301α'),/Unsupported|combining/);
});
test('conversion records retain original text and exact delivered bytes',()=>{
  const p=prepareInput('λόγος, ἄνθρωπος.','grc');
  assert.equal(p.original,'λόγος, ἄνθρωπος.');assert.equal(p.delivered,'lo/gos\na)/nqrwpos\n');
  assert.equal(p.records[1].start,7);assert.equal(p.original.slice(p.records[1].start,p.records[1].end),'ἄνθρωπος');
  assert.equal(prepareInput('Arma virumque canō','lat').delivered,'Arma\nvirumque\ncano\n');
  assert.equal(prepareInput('lo/gos\r\n','grc','original').delivered,'lo/gos\r\n');
  assert.throws(()=>prepareInput('λόγος','grc','original'),/ASCII/);
  assert.throws(()=>prepareInput('bonjouré','lat'),/Unsupported/);
});
