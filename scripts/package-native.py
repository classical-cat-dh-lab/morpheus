#!/usr/bin/env python3
"""Package the unchanged qualified Apple Silicon executable and morphology data."""
import argparse, gzip, hashlib, io, json, tarfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--reference', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    version = json.loads((ROOT / 'package.json').read_text())['version']
    qualified = json.loads((ROOT / 'rust/evidence/qualification.json').read_text())
    binary = (args.candidate / 'native/cruncher').read_bytes()
    expected = next(f for f in qualified['artifacts'] if f['path'] == 'native/cruncher')
    assert hashlib.sha256(binary).hexdigest() == expected['sha256']
    reference = json.loads((args.reference / 'build-manifest.json').read_text())
    assert reference['runtimeDataIdentity'] == qualified['data']
    files = {'cruncher': binary}
    for item in reference['runtimeFiles']:
        data = (args.reference / 'runtime' / item['path']).read_bytes()
        assert hashlib.sha256(data).hexdigest() == item['sha256']
        files['runtime/' + item['path']] = data
    for name in ['LICENSE', 'CITATION.cff', 'rust/ACCEPTANCE.md', 'rust/preservation-baseline.json', 'rust/evidence/qualification.json']:
        files[name] = (ROOT / name).read_bytes()
    for path in (ROOT / 'licenses').iterdir():
        if path.is_file(): files['licenses/' + path.name] = path.read_bytes()
    files['README.md'] = f'''# Morph {version} native companion

Qualified platform: macOS on Apple Silicon (arm64). The unchanged Rust executable
is supplied with its complete morphology data. This is a command-line analyzer,
without the browser UI or mounted dictionary reader. Other platforms are not
advertised as tested binaries; all source and pinned build instructions remain
in the matching source archive. No compiler or Node.js is needed for this binary.

From this extracted directory:

```sh
export MORPHLIB="$PWD/runtime"
printf 'lo/gos\\n' | ./cruncher -T
printf 'amo\\n' | ./cruncher -T -L
```

Use ASCII Beta Code for Greek. One word per line; keep words at most 48 bytes.
The original program is preserved, including its historical limitations. Start a
new process for each job. This binary exposes the original command line directly;
the browser/Node protocol wrapper supplies additional explicit input bounds.
See rust/ACCEPTANCE.md for the native/Wasm qualification boundary.

Core and morphology data: CC BY-SA 3.0 US; retain LICENSE and component notices.
This executable is not Apple-notarized. Verify SHA256SUMS.txt from the signed
matching release before use. CLI interface refinements are deferred until 1.0.
'''.encode()
    files['SHA256SUMS.txt'] = ''.join(hashlib.sha256(data).hexdigest()+'  '+name+'\n' for name,data in sorted(files.items())).encode()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open('wb') as raw, gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0, compresslevel=9) as gz, tarfile.open(fileobj=gz, mode='w', format=tarfile.PAX_FORMAT) as archive:
        for name,data in sorted(files.items()):
            info = tarfile.TarInfo(name); info.size = len(data); info.mode = 0o755 if name == 'cruncher' else 0o644
            archive.addfile(info, io.BytesIO(data))
    print(json.dumps({'file': args.output.name, 'bytes': args.output.stat().st_size, 'sha256': hashlib.sha256(args.output.read_bytes()).hexdigest(), 'files':len(files)}))

if __name__ == '__main__': main()
