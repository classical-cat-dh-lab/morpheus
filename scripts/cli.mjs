#!/usr/bin/env node
import {readFileSync, writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {runEngine, identity, protocol} from '../engine/runtime.mjs';

const arguments_ = process.argv.slice(2), file = arguments_[0] === '--file';
if (file) arguments_.shift();
if (arguments_.includes('--help')) {
  console.log('Usage: node scripts/cli.mjs [--file] [original flags]\nReads original ASCII input from stdin. Pass -T to disable timing.\n--file runs the original file workflow and writes input.morph/.failed/.stats.');
  process.exit(0);
}
const request = {protocol, id:'cli', profile:identity.profile, implementation:identity.implementation,
  data:identity.data, operation:file?'cruncher-file':'cruncher', argv:arguments_, stdin:new Uint8Array(readFileSync(0))};
try {
  const result = await runEngine(request,{locateFile:name=>fileURLToPath(new URL('../engine/'+name,import.meta.url))});
  process.stdout.write(result.stdout); process.stderr.write(result.stderr);
  if (file) for (const [name,bytes] of Object.entries(result.files)) writeFileSync(name,bytes,{flag:'wx'});
  if (result.termination.kind !== 'exit') { console.error(result.termination.message); process.exitCode=1; }
  else process.exitCode=result.termination.code;
} catch(error) { console.error(String(error.message??error)); process.exitCode=1; }
