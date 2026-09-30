#!/usr/bin/env python3
"""Build C-only counterfactual controls, never replacement references."""
import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FROZEN = 'c91ddc0e424197e3d3cd4e4d396979367f0d37101f608bdd6e9205b841914273'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference', type=Path, required=True)
    parser.add_argument('--emsdk', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    ref, sdk, output = args.reference.resolve(), args.emsdk.resolve(), args.output.resolve()
    if output.exists():
        parser.error('Preserve prior evidence; use a fresh output directory.')
    assert hashlib.sha256((ref / 'artifact/morpheus.wasm').read_bytes()).hexdigest() == FROZEN
    output.mkdir(parents=True)
    emcc = sdk / 'upstream/emscripten/emcc'
    env = dict(os.environ, EMSDK_PYTHON=sys.executable, LC_ALL='C', TZ='UTC')
    flags = ['-O2', '-std=gnu89', '-I' + str(ref / 'wasm/src/includes'),
             '-I' + str(ref / 'wasm/src/anal'), '-Wno-return-type',
             '-Wno-implicit-function-declaration', '-Wno-int-conversion',
             '-Wno-incompatible-function-pointer-types', '-fno-common']
    source = (ref / 'wasm/src/anal/checkstring.c').read_text(encoding='latin1')
    old = "*a == 'i' && *(a+2) && strchr"
    assert source.count(old) == 1
    variants = {
        'guard': (source.replace(old, "*a == 'i' && *(a+1) && *(a+2) && strchr"), []),
        'zero': (source, ['-ftrivial-auto-var-init=zero']),
        'pattern': (source, ['-ftrivial-auto-var-init=pattern']),
    }
    for name, (text, extra) in variants.items():
        dest = output / name
        dest.mkdir()
        cfile, obj = dest / 'checkstring.c', dest / 'checkstring.o'
        cfile.write_text(text, encoding='latin1')
        commands = [
            [emcc, *flags, *extra, '-c', cfile, '-o', obj],
            [emcc, *flags, ref / 'wasm/src/anal/stdiomorph.c', obj,
             ref / 'wasm/src/gener/genwd.o',
             *[ref / 'wasm/src' / d / (d + '.a') for d in
               ['anal', 'gener', 'gkends', 'gkdict', 'morphlib', 'greeklib']],
             '-o', dest / 'morpheus.mjs', '-sMODULARIZE=1', '-sEXPORT_ES6=1',
             '-sINVOKE_RUN=0', '-sALLOW_MEMORY_GROWTH=1',
             '-sEXPORTED_RUNTIME_METHODS=callMain,FS,ENV', '-sENVIRONMENT=web,worker,node',
             '--preload-file', str(ref / 'runtime') + '@/morphlib'],
        ]
        for i, command in enumerate(commands):
            result = subprocess.run(list(map(str, command)), env=env, capture_output=True)
            (dest / f'{i}.log').write_bytes(result.stdout + result.stderr)
            result.check_returncode()
        print(name, flush=True)
    # Use precisely the same frozen byte runner and retain outputs, not analyses
    # normalized by a dictionary adapter or presentation layer.
    runner = (ROOT / 'scripts/verify-rust.mjs').as_uri()
    program = '''
import {runner,fingerprint} from RUNNER;
import {writeFileSync} from 'node:fs';
const [reference,output]=process.argv.slice(2),rows=[];
for(const [name,path] of [['frozen',reference+'/artifact'],...['guard','zero','pattern'].map(n=>[n,output+'/'+n])]){
 const run=await runner(path);
 for(const input of ['dictatoriis\\nobiectivis\\n','subiectus\\nexto\\n','exto\\n']){
  const r=await run({stdin:Buffer.from(input),argv:['-T','-L'],operation:'cruncher-file'});
  rows.push({name,input,fingerprint:fingerprint(r),analyses:r.files['input.morph']?.toString()});
 }
}
writeFileSync(output+'/controls.json',JSON.stringify(rows,null,2)+'\\n');
if(JSON.stringify(rows[1].fingerprint)===JSON.stringify(rows[7].fingerprint))throw Error('Expected C-only zero-initialization control to change the sequence witness');
console.log('PASS: C-only initialization changes the sequence witness.');
'''.replace('RUNNER', json.dumps(runner))
    script = output / 'compare.mjs'
    script.write_text(program)
    subprocess.run(['node', str(script), str(ref), str(output)], check=True)


if __name__ == '__main__':
    main()
