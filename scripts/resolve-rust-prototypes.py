from pathlib import Path
import argparse,json,shutil,re
root=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser();p.add_argument('--input',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args();original=a.input.resolve();out=a.output.resolve()
if out.exists():raise SystemExit('Use a fresh input directory.')
shutil.copytree(original/'perseus',out/'perseus');trees={};defs={}
for p in (original/'ast').glob('*.json'):
 tree=json.loads(p.read_text());trees[p.stem]=tree
 for n in tree.get('inner',[]):
  if n.get('kind')=='FunctionDecl' and any(c['kind']=='CompoundStmt' for c in n.get('inner',[])) and 'includedFrom'not in n.get('loc',{}) and n.get('storageClass')!='static':defs[n['name']]=n
changes=[]
for unit in json.loads((root/'rust/translation-units.json').read_text()):
 relative=unit['path'];key=relative.replace('/','-')[:-2];tree=trees[key];path=out/'perseus/src'/relative;source=path.read_text(encoding="latin1");calls=set();functions=[]
 def walk(n):
  if n.get('kind')=='DeclRefExpr'and n.get('referencedDecl',{}).get('kind')=='FunctionDecl':calls.add(n['referencedDecl']['name'])
  for c in n.get('inner',[]):walk(c)
 for n in tree.get('inner',[]):
  if n.get('kind')=='FunctionDecl'and any(c['kind']=='CompoundStmt'for c in n.get('inner',[]))and 'includedFrom'not in n.get('loc',{}):functions.append(n);walk(n)
 declared=[]
 for name in sorted(calls):
  if name not in defs:continue
  t=defs[name]['type']['qualType'];position=t.index('(');prototype=t[:position]+name+t[position:]
  if prototype.endswith('()'):prototype=prototype[:-2]+'(void)'
  declared.append(prototype+';')
 # These two retained-library helpers are outside the cruncher call closure.
 # Their original missing arguments have no defined portable value. Never invent one.
 unsupported={'gkends/mkend.c':('do_dissim(gkstring_of(Have))','morph_port_unavailable("gkends/mkend.c:94:do_dissim")'),'morphlib/gkstring.c':('AddDialect(di,dialbuf)','morph_port_unavailable("morphlib/gkstring.c:527:AddDialect")')}
 if relative in unsupported:
  old,new=unsupported[relative];assert source.count(old)==1;source=source.replace(old,new);declared.append('int morph_port_unavailable(const char *);')
 if declared:
  first=min(n['range']['begin']['offset'] for n in functions)
  source=source[:first]+'\n/* Explicit cross-unit signatures for source-faithful Rust translation. */\n'+'\n'.join(declared)+'\n'+source[first:]
 path.write_text(source,encoding="latin1");changes.append({'path':relative,'explicit_signatures':declared,'unsupported_call':unsupported.get(relative)})
# Redeclarations inside function blocks must carry the same explicit signature.
for path in (out/'perseus/src').rglob('*'):
 if path.suffix not in ('.c','.h'):continue
 text=path.read_text(encoding='latin1')
 for name,n in defs.items():
  t=n['type']['qualType'];params=t[t.index('(')+1:-1]
  if not params or params=='void':continue
  text=re.sub(r'\b'+re.escape(name)+r'\(\s*\)(?=\s*[,;])',name+'('+params+')',text)
 path.write_text(text,encoding='latin1')
commands=json.loads((original/'compile_commands.json').read_text())
for command in commands:
 command['directory']=command['directory'].replace(str(original.resolve()),str(out.resolve()));command['file']=command['file'].replace(str(original.resolve()),str(out.resolve()));command['arguments']=[x.replace(str(original.resolve()),str(out.resolve())) for x in command['arguments']]
(out/'compile_commands.json').write_text(json.dumps(commands,indent=2)+'\n');(out/'type-resolution.json').write_text(json.dumps(changes,indent=2)+'\n')
print('Resolved signatures for',len(changes),'units')
