#!/usr/bin/env python3
"""Prepare the exact portable C translation input without changing the reference."""
import argparse,hashlib,json,re,tarfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--output',type=Path,required=True)
p.add_argument('--emsdk',type=Path,required=True)
p.add_argument('--clang',type=Path,required=True)
a=p.parse_args();out=a.output.resolve();sdk=a.emsdk.resolve();clang=a.clang.resolve()
if out.exists():p.error('Use a fresh output directory.')
lock=json.loads((ROOT/'source.lock.json').read_text())
archive=ROOT/lock['localSnapshot']['archive']
assert hashlib.sha256(archive.read_bytes()).hexdigest()==lock['localSnapshot']['archiveSha256']
out.mkdir(parents=True)
with tarfile.open(archive) as bundle:bundle.extractall(out,filter='data')
src=out/'perseus/src'
adaptations=[('gkdict/derivio.c','strcpy(derivsuff,s);','memmove(derivsuff,s,strlen(s)+1);'),('anal/prvb.c','strcpy(stem_of(&WorkGkword),stem_of(&WorkGkword)+1);','memmove(stem_of(&WorkGkword),stem_of(&WorkGkword)+1,strlen(stem_of(&WorkGkword)+1)+1);')]
for name,old,new in adaptations:
 f=src/name;content=f.read_text();assert content.count(old)==1;f.write_text(content.replace(old,new))
f=src/'morphlib/fixacc.c';content,n=re.subn(r'\b(getsyll2?)\(([^,\n]+),([^,\n]+),(?:0|is_ending)\)',r'\1(\2,\3)',f.read_text());assert n==16;f.write_text(content)
units=json.loads((ROOT/'rust/translation-units.json').read_text());commands=[]
flags=['-target','wasm32-unknown-emscripten','--sysroot='+str(sdk/'upstream/emscripten/cache/sysroot'),'-std=gnu89','-I'+str(src/'includes'),'-I'+str(sdk/'upstream/emscripten/cache/sysroot/include'),'-Wno-implicit-function-declaration','-Wno-implicit-int','-Wno-int-conversion','-Wno-incompatible-function-pointer-types','-Wno-return-type','-fno-common']
for unit in units:
 f=src/unit['path'];assert hashlib.sha256(f.read_bytes()).hexdigest()==unit['sha256'],unit['path']
 commands.append({'directory':str(f.parent),'file':str(f),'arguments':[str(clang),*flags,'-c',str(f),'-o',str(f.with_suffix('.o'))]})
(out/'compile_commands.json').write_text(json.dumps(commands,indent=2)+'\n')
print(json.dumps({'units':len(units),'source':lock['revision'],'compilation_database':str(out/'compile_commands.json')}))
