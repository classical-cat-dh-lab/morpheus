from pathlib import Path
import argparse,concurrent.futures,hashlib,json,subprocess
p=argparse.ArgumentParser();p.add_argument('--input',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
build=a.input.resolve();units=json.loads((build/'compile_commands.json').read_text());astdir=build/'ast';astdir.mkdir(exist_ok=True)
def one(unit):
 f=Path(unit['file']);command=unit['arguments'][:-4]+['-Wno-comment','-Xclang','-ast-dump=json','-fsyntax-only',str(f)]
 result=subprocess.run(command,capture_output=True)
 name=f.parent.name+'/'+f.name
 (astdir/(f.parent.name+'-'+f.stem+'.diagnostics.txt')).write_bytes(result.stderr)
 if result.returncode:return {'path':name,'parsed':False,'errors':result.stderr.decode(errors='replace')[-1800:]}
 dest=astdir/(f.parent.name+'-'+f.stem+'.json');dest.write_bytes(result.stdout);tree=json.loads(result.stdout);source=f.read_bytes();functions=[]
 for node in tree.get('inner',[]):
  if node.get('kind')!='FunctionDecl':continue
  body=next((n for n in node.get('inner',[]) if n.get('kind')=='CompoundStmt'),None)
  if not body or 'includedFrom' in node.get('loc',{}):continue
  begin=node['range']['begin'];end=node['range']['end']
  if 'offset' not in begin or 'offset' not in end:continue
  start=begin['offset'];stop=end['offset']+end['tokLen'];calls=set()
  def walk(n):
   if n.get('kind')=='DeclRefExpr' and n.get('referencedDecl',{}).get('kind')=='FunctionDecl':calls.add(n['referencedDecl']['name'])
   for c in n.get('inner',[]):walk(c)
  walk(body)
  functions.append({'name':node['name'],'line':source[:start].count(b'\n')+1,'last_line':source[:stop].count(b'\n')+1,'c_type':node['type']['qualType'],'storage':node.get('storageClass','external'),'source_sha256':hashlib.sha256(source[start:stop]).hexdigest(),'calls':sorted(calls)})
 return {'path':name,'parsed':True,'functions':functions}
with concurrent.futures.ThreadPoolExecutor(max_workers=4)as pool:results=list(pool.map(one,units))
report={'source_revision':'b1b33c56ef2338fe0dcd1893628ed638f00c0986','units':len(results),'functions':sum(len(r.get('functions',[]))for r in results),'files':results}
a.output.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'units':report['units'],'functions':report['functions'],'failures':[r for r in results if not r['parsed']]}))
