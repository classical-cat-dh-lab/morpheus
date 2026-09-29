#!/usr/bin/env python3
"""Build separate C checkpoint witnesses from a completed frozen reference build."""
import argparse,json,os,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser(description=__doc__);p.add_argument('--reference',type=Path,required=True);p.add_argument('--emsdk',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args();ref=a.reference.resolve();out=a.output.resolve();sdk=a.emsdk.resolve()
if out.exists():p.error('Use a fresh directory.')
out.mkdir(parents=True);(out/'logs').mkdir();hooks=json.loads((ROOT/'rust/probes/hooks.json').read_text());env=dict(os.environ,EMSDK_PYTHON=sys.executable,LC_ALL='C',TZ='UTC');commands=[]
def run(cmd,name):
 cmd=list(map(str,cmd));commands.append(cmd)
 with(out/'logs'/name).open('wb')as log:r=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=300)
 if r.returncode:raise RuntimeError('Inspect '+str(out/'logs'/name))
for target,cc in [('native','clang'),('wasm',sdk/'upstream/emscripten/emcc')]:
 tree=ref/target;dest=out/target;dest.mkdir();objects=[]
 flags=['-O2','-std=gnu89','-I'+str(tree/'src/includes'),'-I'+str(tree/'src/anal'),'-Wno-return-type','-Wno-implicit-function-declaration','-Wno-int-conversion','-Wno-incompatible-function-pointer-types','-fno-common']
 for unit in sorted(set(h['unit']for h in hooks)):
  text=(tree/'src'/(unit+'.c')).read_text(encoding='latin1')
  for name in sorted(set(h['function']for h in hooks if h['unit']==unit)):
   m=re.search(r'(?m)^'+re.escape(name)+r'\([^;{}]*\)\s*\{',text);assert m,name
   insert='\n'+''.join('morph_trace_word("'+h['tag']+'",'+h['parameter']+');\n'for h in hooks if h['unit']==unit and h['function']==name);text=text[:m.end()]+insert+text[m.end():]
  if unit=='anal/prntanal':
   old='SortAnals(analysis_of(Gkword),nanals);';assert text.count(old)==1;text=text.replace(old,old+'\nmorph_trace_word("PrntAnalyses:sorted",Gkword);')
  text,n=re.subn(r'#include\s*<gkstring.h>', '#include <gkstring.h>\nvoid morph_trace_word(const char*,const gk_word*);',text);assert n==1
  source=dest/(Path(unit).name+'.c');source.write_text(text,encoding='latin1');obj=source.with_suffix('.o');run([cc,*flags,'-c',source,'-o',obj],target+'-'+source.stem+'.log');objects.append(obj)
 cmd=[cc,*flags,tree/'src/anal/stdiomorph.c',*([ROOT/'rust/probes/trace.c']if target=='native'else[]),*objects,tree/'src/gener/genwd.o',*[tree/'src'/d/(d+'.a')for d in ['anal','gener','gkends','gkdict','morphlib','greeklib']]]
 if target=='native':cmd += [ref/'runtime-sort.c','-o',dest/'cruncher']
 else:cmd += ['--js-library',ROOT/'rust/probes/trace-library.js','-o',dest/'morpheus.mjs','-sMODULARIZE=1','-sEXPORT_ES6=1','-sINVOKE_RUN=0','-sALLOW_MEMORY_GROWTH=1','-sEXPORTED_RUNTIME_METHODS=callMain,FS,ENV','-sENVIRONMENT=web,worker,node','--preload-file',str(ref/'runtime')+'@/morphlib']
 run(cmd,target+'-link.log')
(out/'commands.json').write_text(json.dumps(commands,indent=2)+'\n');print(out)
