import createEngine from './morpheus.mjs';
import {identity} from './identity.mjs';

export {identity};
export const protocol = 'morph-engine/1';
export const limits = Object.freeze({inputBytes: 65536, lines: 1000, tokenBytes: 48, outputBytes: 16777216});
const flags = new Set(['-T', '-L', '-a', '-l', '-m', '-b', '-c', '-k', '-i', '-d', '-s', '-n', '-x', '-S', '-V', '-p', '-P']);

export function validateRequest(request) {
  if (!request || request.protocol !== protocol) throw new Error('Unsupported engine protocol.');
  for (const key of ['profile', 'implementation', 'data']) {
    if (request[key] !== identity[key]) throw new Error(`Unavailable ${key}: select an installed engine explicitly.`);
  }
  if (!['cruncher', 'cruncher-file'].includes(request.operation)) throw new Error('Unsupported engine operation.');
  if (!Array.isArray(request.argv) || request.argv.some(arg => !flags.has(arg))) throw new Error('Unsupported engine option.');
  if (!(request.stdin instanceof Uint8Array)) throw new Error('stdin must be a byte array.');
  if (request.stdin.length > limits.inputBytes) throw new Error('This engineering build accepts at most 64 KiB per run.');
  let line = 0, lines = 1;
  for (const byte of request.stdin) {
    if (byte > 127 || byte === 0 || (byte < 32 && ![9, 10, 13].includes(byte))) throw new Error('Original input must contain ASCII text without control bytes.');
    if (byte === 10) { lines++; line = 0; }
    else if (++line > limits.tokenBytes) throw new Error('This engineering build accepts lines of at most 48 bytes.');
  }
  if (request.stdin.at(-1) === 10 || request.stdin.length === 0) lines--;
  if (lines > limits.lines) throw new Error('This engineering build accepts at most 1,000 lines per run.');
  if (request.operation === 'cruncher-file' && !request.argv.includes('-T')) throw new Error('File mode currently requires the explicit -T option.');
}

/** One invocation owns one complete legacy process lifetime. */
export async function runEngine(request, options = {}) {
  validateRequest(request);
  const stdout = [], stderr = [];
  let offset = 0, count = 0;
  const capture = channel => byte => {
    if (byte === null) return;
    if (++count > limits.outputBytes) throw new Error('Engine output exceeds the declared 16 MiB limit.');
    channel.push(byte);
  };
  const module = await createEngine({
    noInitialRun: true,
    locateFile: name => options.locateFile ? options.locateFile(name) : new URL(name, import.meta.url).href,
    ...(options.wasmBinary ? {wasmBinary: options.wasmBinary} : {}),
    ...(options.data ? {getPreloadedPackage: () => options.data} : {}),
    stdin: () => offset < request.stdin.length ? request.stdin[offset++] : null,
    stdout: capture(stdout), stderr: capture(stderr),
    preRun: [module => {
      module.ENV.MORPHLIB = '/morphlib'; module.ENV.LC_ALL = 'C'; module.ENV.TZ = 'UTC';
      module.FS.mkdir('/job'); module.FS.chdir('/job');
      if (request.operation === 'cruncher-file') module.FS.writeFile('input.words', request.stdin);
    }],
  });
  let termination;
  try {
    const argv = request.operation === 'cruncher-file' ? [...request.argv, 'input'] : [...request.argv];
    termination = {kind: 'exit', code: module.callMain(argv)};
  } catch (error) {
    termination = {kind: 'trap', message: String(error.message ?? error)};
  }
  const files = {};
  for (const name of ['input.morph', 'input.failed', 'input.stats']) {
    if (module.FS.analyzePath(name).exists) files[name] = module.FS.readFile(name).slice();
  }
  return {protocol, id: request.id, ...identity, operation: request.operation,
    stdout: Uint8Array.from(stdout), stderr: Uint8Array.from(stderr), files, termination};
}
