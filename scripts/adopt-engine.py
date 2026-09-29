#!/usr/bin/env python3
"""Import a verified local build and sanitize host paths in distributable evidence."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('build', type=Path)
parser.add_argument('--emsdk', type=Path, required=True)
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
build = args.build.resolve()
manifest = json.loads((build / 'build-manifest.json').read_text())
assert '6.0.6' in manifest['tools']['emscripten'].splitlines()[0], 'This implementation identity requires Emscripten 6.0.6'
lock = json.loads((root / 'source.lock.json').read_text())
assert manifest['source']['archiveSha256'] == lock['localSnapshot']['archiveSha256']
assert manifest['source']['revision'] == lock['revision']
for entry in manifest['artifacts']:
    source = build / 'artifact' / entry['file']
    assert source.stat().st_size == entry['bytes']
    assert hashlib.sha256(source.read_bytes()).hexdigest() == entry['sha256']
    shutil.copyfile(source, root / 'engine' / entry['file'])
identity = {'profile': 'legacy-c-engineering-0.0.1',
            'implementation': 'perseus-b1b33c5-emscripten-6.0.6',
            'data': manifest['runtimeDataIdentity'],
            'assets': {e['file']: {'bytes': e['bytes'], 'sha256': e['sha256']} for e in manifest['artifacts']},
            'source': manifest['source']}
(root / 'engine/identity.mjs').write_text('export const identity = Object.freeze(' + json.dumps(identity, indent=2) + ');\n')
replacements = [(str(build), '$BUILD'), (str(args.emsdk.resolve()), '$EMSDK'), (str(root), '$PROJECT')]
def sanitize(text):
    for old, new in replacements:
        text = text.replace(old, new)
    return text
for entry in manifest['patches']:
    data = sanitize((build / entry['patch']).read_text()).encode()
    (root / 'patches' / entry['patch']).write_bytes(data)
    entry['sha256'] = hashlib.sha256(data).hexdigest()
manifest['pathConvention'] = 'Absolute build/SDK/project paths are replaced by $BUILD/$EMSDK/$PROJECT; patch hashes refer to these path-neutral copies.'
(root / 'evidence/build.json').write_text(sanitize(json.dumps(manifest, indent=2)) + '\n')
print('Imported verified engine, identity, adaptations and path-neutral build evidence.')
