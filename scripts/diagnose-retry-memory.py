#!/usr/bin/env python3
"""Observe the pinned original executable without recompiling its C source."""
import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FROZEN_SHA256 = 'c91ddc0e424197e3d3cd4e4d396979367f0d37101f608bdd6e9205b841914273'
FEATURES = ['--enable-bulk-memory', '--enable-nontrapping-float-to-int',
            '--enable-sign-ext', '--enable-mutable-globals']


def instrument(text):
    imports = (' (import "causality" "snapshot" (func $causal_snapshot (param i32 i32 i32 i32)))\n'
               ' (import "causality" "read" (func $causal_read (param i32 i32 i32) (result i32)))\n')
    text = text.replace(' (global $global$0', imports + ' (global $global$0', 1)
    parts = re.split(r'(?=^ \(func )', text, flags=re.M)
    for number in (20, 21):
        index = next(i for i, f in enumerate(parts) if f.startswith(f' (func ${number} '))
        function = parts[index]
        start = function.index('  (global.set $global$0')
        depth = 0
        for end in range(start, len(function)):
            if function[end] == '(':
                depth += 1
            elif function[end] == ')':
                depth -= 1
                if depth == 0:
                    break
        end += 1
        hook = f'\n  (call $causal_snapshot (i32.const {number}) (local.get $6) (local.get $0) (global.get $global$0))'
        function = function[:end] + hook + function[end:]
        if number == 20:
            original = '(i32.load8_u offset=2\n         (local.get $1)\n        )'
            assert function.count(original) == 1
            function = function.replace(original, '(i32.load8_u offset=2\n         (call $causal_read (local.get $1) (local.get $6) (local.get $0))\n        )')
        parts[index] = function
    index = next(i for i, f in enumerate(parts) if f.startswith(' (func $206 '))
    function = parts[index]
    end = list(re.finditer(r'^  \(local [^\n]*\n', function, re.M))[-1].end()
    parts[index] = function[:end] + '  (call $causal_snapshot (i32.const 206) (local.get $0) (local.get $1) (global.get $global$0))\n' + function[end:]
    return ''.join(parts)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference', type=Path, required=True, help='Frozen C artifact directory')
    parser.add_argument('--emsdk', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    reference, sdk, output = args.reference.resolve(), args.emsdk.resolve(), args.output.resolve()
    if output.exists():
        parser.error('Preserve prior evidence; use a fresh output directory.')
    assert hashlib.sha256((reference / 'morpheus.wasm').read_bytes()).hexdigest() == FROZEN_SHA256
    output.mkdir(parents=True)
    binaryen = sdk / 'upstream/bin'

    def run(*command):
        subprocess.run(list(map(str, command)), check=True)

    run(binaryen / 'wasm-dis', reference / 'morpheus.wasm', '-o', output / 'original.wat')
    original_wat = (output / 'original.wat').read_text()
    functions = re.split(r'(?=^ \(func )', original_wat, flags=re.M)
    for number in (16, 17, 20, 21, 28):
        function = next(f for f in functions if f.startswith(f' (func ${number} '))
        (output / f'function-{number}.wat').write_text(function)
    (output / 'observed.wat').write_text(instrument(original_wat))
    run(binaryen / 'wasm-as', output / 'observed.wat', '-g', *FEATURES, '-o', output / 'observed.wasm')
    run(binaryen / 'wasm-opt', output / 'observed.wasm', '--instrument-memory', '-g', *FEATURES, '-o', output / 'memory.wasm')
    run(binaryen / 'wasm-dis', output / 'memory.wasm', '-o', output / 'memory.wat')
    observer = ROOT / 'scripts/observe-frozen-memory.mjs'
    for name, binary, watch, byte in [
        ('original', 'original', '', ''), ('observed', output / 'observed.wasm', '', ''),
        ('memory', output / 'memory.wasm', '93016', ''),
        ('single-byte-j', output / 'memory.wasm', '93016', '106'),
        ('single-byte-zero', output / 'memory.wasm', '93016', '0'),
    ]:
        run('node', observer, reference, binary, output / (name + '.json'), watch, byte)
    reports = {name: json.loads((output / (name + '.json')).read_text())
               for name in ('original', 'observed', 'memory', 'single-byte-j', 'single-byte-zero')}
    original = reports['original']
    assert all(r['fingerprint'] == original['fingerprint'] for r in reports.values())
    assert all(reports[n]['memorySha256'] == original['memorySha256'] for n in ('observed', 'memory'))
    assert reports['single-byte-j']['injections'] == [{'address': 93016, 'before': 0, 'after': 106}]
    assert [e['word'] for e in reports['single-byte-j']['events'] if e['kind'] == 'entry20'] == [
        'dictatoriis', 'dictatorjis', 'obiectivis', 'obiecti', 'objecti',
        'objectj', 'obiectj', 'objectj', 'objectivis']
    print('PASS: observer preserves I/O and complete final linear memory; one-byte intervention changes the retry path.')


if __name__ == '__main__':
    main()
