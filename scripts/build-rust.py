#!/usr/bin/env python3
"""Build the preservation candidate; compilation does not imply qualification."""
import argparse,hashlib,json,os,shutil,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FROZEN_C_WASM='c91ddc0e424197e3d3cd4e4d396979367f0d37101f608bdd6e9205b841914273'
FROZEN_DATA='9309f9ae824ff8e2f0db3636e4896a959381c4b5540d314281d688e4451b226f'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--toolchain',type=Path,required=True);p.add_argument('--emsdk',type=Path,required=True);p.add_argument('--reference',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--trace',action='store_true');a=p.parse_args()
 out=a.output.resolve();tool=a.toolchain.resolve();sdk=a.emsdk.resolve();ref=a.reference.resolve()
 if out.exists():p.error('Retain prior evidence and use a fresh output directory.')
 manifest=json.loads((ref/'build-manifest.json').read_text());lock=json.loads((ROOT/'source.lock.json').read_text());assert manifest['source']['revision']==lock['revision']
 # The retained-byte model targets this compiled reference, not arbitrary C builds.
 if sha(ref/'artifact/morpheus.wasm')!=FROZEN_C_WASM or manifest['runtimeDataIdentity']!=FROZEN_DATA:raise RuntimeError('Reference executable/data differs from the causally qualified preservation baseline.')
 for item in manifest['runtimeFiles']:assert sha(ref/'runtime'/item['path'])==item['sha256'],item['path']
 out.mkdir(parents=True);(out/'artifact').mkdir();(out/'native').mkdir();(out/'logs').mkdir();commands=[]
 env=dict(os.environ,CARGO_ENCODED_RUSTFLAGS='--remap-path-prefix='+str(ROOT)+'=morpheus',RUSTFLAGS='',RUSTC=str(tool/'bin/rustc'),CARGO_HOME=str(out/'cargo-home'),CARGO_TARGET_DIR=str(out/'target'),CARGO_BUILD_JOBS='4',EMSDK_PYTHON=sys.executable,LC_ALL='C',TZ='UTC',PATH=str(tool/'bin')+os.pathsep+os.environ.get('PATH',''))
 rust=subprocess.check_output([tool/'bin/rustc','--version'],env=env,text=True).strip();emcc=sdk/'upstream/emscripten/emcc';em=subprocess.check_output([emcc,'--version'],env=env,text=True).strip()
 if not rust.startswith('rustc 1.98.1 ') or '6.0.6'not in em:raise RuntimeError('This candidate is pinned to Rust 1.98.1 and Emscripten 6.0.6.')
 def run(cmd,cwd,name):
  cmd=list(map(str,cmd));commands.append({'argv':cmd,'cwd':str(cwd),'log':name})
  with(out/'logs'/name).open('wb')as f:r=subprocess.run(cmd,cwd=cwd,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=600)
  if r.returncode:raise RuntimeError(f'Build failed; inspect {out/"logs"/name}')
 features=(["trace"]if a.trace else [])
 cargo=[tool/'bin/cargo','build','--offline','--frozen','--release',*(['--features',','.join(features)]if features else [])];crate=ROOT/'rust/engine'
 run([*cargo,'--bin','cruncher'],crate,'native.log');shutil.copy2(out/'target/release/cruncher',out/'native/cruncher')
 run([*cargo,'--lib','--target','wasm32-unknown-emscripten'],crate,'wasm.log')
 library=out/'target/wasm32-unknown-emscripten/release/libmorpheus_preservation.a'
 run([emcc,library,'-O2','-o','morpheus.mjs','-sMODULARIZE=1','-sEXPORT_ES6=1','-sINVOKE_RUN=0','-sALLOW_MEMORY_GROWTH=1','-sEXPORTED_RUNTIME_METHODS=callMain,FS,ENV','-sENVIRONMENT=web,worker,node','--preload-file',str(ref/'runtime')+'@/morphlib'],out/'artifact','link.log')
 # Package the same protocol wrapper with an explicit candidate identity.
 identity_source=(ROOT/'engine/identity.mjs').read_text();identity=json.loads(identity_source.split('Object.freeze(',1)[1].rsplit(');',1)[0]);identity['implementation']='perseus-b1b33c5-rust-candidate.2'
 identity['assets']={f.name:{'bytes':f.stat().st_size,'sha256':sha(f)}for f in sorted((out/'artifact').iterdir())if f.suffix in ('.mjs','.wasm','.data')}
 (out/'artifact/identity.mjs').write_text('export const identity = Object.freeze('+json.dumps(identity,indent=2)+');\n')
 shutil.copyfile(ROOT/'engine/runtime.mjs',out/'artifact/runtime.mjs')
 cli=(ROOT/'scripts/cli.mjs').read_text().replace('../engine/','./');(out/'artifact/cli.mjs').write_text(cli)
 files=[{'path':str(f.relative_to(out)),'bytes':f.stat().st_size,'sha256':sha(f)}for f in sorted((out/'artifact').iterdir())]+[{'path':'native/cruncher','bytes':(out/'native/cruncher').stat().st_size,'sha256':sha(out/'native/cruncher')}]
 result={'schema':1,'qualification':'candidate; run differential gates before use','source':manifest['source'],'data':manifest['runtimeDataIdentity'],'features':features,'rustc':rust,'emscripten':em,'cargoLockSha256':sha(crate/'Cargo.lock'),'sourceFiles':[{'path':str(f.relative_to(crate)),'sha256':sha(f)}for f in sorted(crate.rglob('*.rs'))if 'target'not in f.parts],'commands':commands,'artifacts':files,'engineLinkage':'Rust static library only; Emscripten platform runtime; no original morphology C objects','nativeRuntime':'Pinned musl qsort, compiled separately from vendor/runtime; platform libc'}
 (out/'build-manifest.json').write_text(json.dumps(result,indent=2)+'\n');print(out/'build-manifest.json')
if __name__=='__main__':main()
