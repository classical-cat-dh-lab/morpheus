import {readFileSync, readdirSync} from 'node:fs';
import {resolve, dirname, relative} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const args = process.argv.slice(2);
if (args.length && (args.length !== 2 || args[0] !== '--artifacts')) {
  throw new Error('Usage: node scripts/check-rust-baseline.mjs [--artifacts BUILD_DIRECTORY]');
}
const read = path => JSON.parse(readFileSync(resolve(root, path), 'utf8'));
const hash = path => createHash('sha256').update(readFileSync(path)).digest('hex');
const baseline = read('rust/preservation-baseline.json');
if (baseline.schema !== 1 || baseline.status !== 'frozen') throw new Error('Unsupported baseline');
const failures = [];
function check(path, expected) {
  try {
    if (hash(path) !== expected) failures.push(`${relative(root, path)}: checksum changed`);
  } catch {
    failures.push(`${relative(root, path)}: missing or unreadable`);
  }
}
for (const [path, expected] of Object.entries(baseline.protectedFiles)) {
  check(resolve(root, path), expected);
}
const qualification = read(baseline.qualification);
for (const [path, expected] of Object.entries(qualification.receipts)) {
  check(resolve(root, 'rust/evidence', path), expected);
}
const build = read('rust/evidence/candidate-2/build.json');
const sourceFiles = [];
function walk(directory) {
  for (const entry of readdirSync(directory, {withFileTypes: true})) {
    if (entry.name === 'target') continue;
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) walk(path);
    else if (entry.name.endsWith('.rs')) sourceFiles.push(relative(resolve(root, 'rust/engine'), path));
  }
}
walk(resolve(root, 'rust/engine'));
if (JSON.stringify(sourceFiles.sort()) !== JSON.stringify(build.sourceFiles.map(f => f.path).sort())) {
  failures.push('Rust source inventory changed');
}
for (const file of build.sourceFiles) check(resolve(root, 'rust/engine', file.path), file.sha256);
if (args.length) {
  const artifacts = resolve(args[1]);
  for (const file of qualification.artifacts) check(resolve(artifacts, file.path), file.sha256);
}
if (failures.length) {
  throw new Error(`Frozen Rust baseline differs:\n${failures.join('\n')}\nRetain this manifest; qualify a successor instead of refreshing hashes.`);
}
console.log(`PASS: frozen Rust core; ${Object.keys(baseline.protectedFiles).length} protected files, ${Object.keys(qualification.receipts).length} receipts${args.length ? ', 7 delivery artifacts' : ''}`);
