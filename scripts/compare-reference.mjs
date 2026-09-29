/** Compare raw streams and termination for bounded Greek/Latin fixtures.
 * Each run receives a new native process or a fresh Wasm instance in a new host
 * process. This is construction evidence, not complete compatibility acceptance.
 */
import {readFileSync, writeFileSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL, fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';

const self = fileURLToPath(import.meta.url);

async function wasmJob(build, encodedInput, encodedArguments) {
  const input = Buffer.from(encodedInput, 'base64');
  const argv = JSON.parse(Buffer.from(encodedArguments, 'base64').toString('utf8'));
  const stdout = [], stderr = [];
  let position = 0;
  const factory = (await import(pathToFileURL(join(build, 'artifact/morpheus.mjs')).href)).default;
  const module = await factory({
    noInitialRun: true,
    locateFile: name => join(build, 'artifact', name),
    stdin: () => position < input.length ? input[position++] : null,
    stdout: byte => { if (byte !== null) stdout.push(byte); },
    stderr: byte => { if (byte !== null) stderr.push(byte); },
    preRun: [module => { module.ENV.MORPHLIB = '/morphlib'; module.ENV.LC_ALL = 'C'; }],
  });
  let termination;
  try {
    termination = {kind: 'exit', code: module.callMain([...argv])};
  } catch (error) {
    termination = {kind: 'runtime-error', message: String(error)};
  }
  process.stdout.write(JSON.stringify({stdout: Buffer.from(stdout).toString('base64'),
    stderr: Buffer.from(stderr).toString('base64'), termination}));
}

function commandResult(command, args, options) {
  return spawnSync(command, args, {...options, timeout: 15000, maxBuffer: 8 * 1024 * 1024});
}

function compare(build, output) {
  const manifest = JSON.parse(readFileSync(join(build, 'build-manifest.json')));
  const fixtures = [];
  const words = ['lo/gos', 'lu/w', 'a)/nqrwpos', 'a)nqrw/pou', 'e)/lusa', 'e)sti/',
    'ei)mi/', 'oi)=da', 'pa=s', 'tis', '*swkra/ths', 'a)gaqo/s', 'mh=nin', 'zzzz'];
  for (const flags of [[], ...['n','l','p','d','a','m','b','c','k','i','s','x','S','V','P'].map(f=>['-'+f])]) {
    for (const word of words) fixtures.push({id: `${flags.join('') || 'default'}:${word}`,
      argv: ['-T', ...flags], stdin: Buffer.from(word + '\n')});
  }
  for (const flags of [[], ...['n','l','p','d','a','m','b','c','k','i','s','x','S','V','P'].map(f=>['-'+f])]) {
    for (const word of ['amo','arma','virumque','cano','rosa','sum','esse','fui','puellae','viri','cornu','Roma','qui','zzzz']) fixtures.push({id:`latin:${flags.join('')||'default'}:${word}`,argv:['-T','-L',...flags],stdin:Buffer.from(word+'\n')});
  }
  for (const [id, stdin] of [
    ['empty', ''], ['blank', '\n'], ['comment', '# construction probe\n'],
    ['no-final-newline', 'lo/gos'], ['crlf', 'lo/gos\r\n'],
    ['leading-space', '  lo/gos\n'], ['trailing-fields', 'lo/gos ignored\n'],
    ['batch-repeat', 'lu/w\nlo/gos\nlu/w\nzzzz\n'],
  ]) fixtures.push({id, argv: ['-T'], stdin: Buffer.from(stdin)});
  const results = fixtures.map(fixture => {
    const native = commandResult(join(build, 'native/src/anal/cruncher'), fixture.argv,
      {input: fixture.stdin, cwd: join(build, 'native/stemlib/Greek'),
       env: {...process.env, MORPHLIB: '..', LC_ALL: 'C', TZ: 'UTC'}});
    if (native.error) throw native.error;
    const nativeResult = {stdout: native.stdout.toString('base64'), stderr: native.stderr.toString('base64'),
      termination: native.signal ? {kind: 'signal', signal: native.signal} : {kind: 'exit', code: native.status}};
    const child = commandResult(process.execPath, [self, '--wasm-job', build, fixture.stdin.toString('base64'),
      Buffer.from(JSON.stringify(fixture.argv)).toString('base64')], {});
    if (child.error) throw child.error;
    if (child.status !== 0 || child.signal) throw new Error(`Wasm host failed for ${fixture.id}: ${child.stderr}`);
    const wasmResult = JSON.parse(child.stdout.toString('utf8'));
    const equal = JSON.stringify(nativeResult) === JSON.stringify(wasmResult);
    return {id: fixture.id, argv: fixture.argv, stdin: fixture.stdin.toString('base64'),
      equal, native: nativeResult, wasm: wasmResult};
  });
  const evidence = {schemaVersion: 1, recordedDate: new Date().toISOString(),
    scope: 'Selected-original Greek/Latin portable-runtime probe; stdin/stdout/stderr/exit; all requests explicitly disable timing with -T',
    source: manifest.source, runtimeDataIdentity: manifest.runtimeDataIdentity,
    buildManifestSha256: createHash('sha256').update(readFileSync(join(build, 'build-manifest.json'))).digest('hex'),
    node: process.version, fixtures: results.length, matches: results.filter(r => r.equal).length,
    mismatches: results.filter(r => !r.equal).map(r => r.id), results};
  writeFileSync(output, JSON.stringify(evidence, null, 2) + '\n');
  console.log(JSON.stringify({fixtures: evidence.fixtures, matches: evidence.matches, mismatches: evidence.mismatches, output}));
  if (evidence.mismatches.length) process.exitCode = 1;
}

if (process.argv[2] === '--wasm-job') {
  await wasmJob(...process.argv.slice(3));
} else {
  const [build, output] = process.argv.slice(2);
  if (!build || !output) throw new Error('Usage: node compare-reference.mjs BUILD_DIRECTORY OUTPUT_JSON');
  compare(resolve(build), resolve(output));
}
