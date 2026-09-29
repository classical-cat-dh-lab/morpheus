#!/usr/bin/env python3
"""Compare the native Rust candidate with recorded frozen C/Wasm byte hashes."""
import argparse,hashlib,json,os,subprocess,tempfile
from pathlib import Path
p=argparse.ArgumentParser(description=__doc__);p.add_argument('--binary',type=Path,required=True);p.add_argument('--data',type=Path,required=True);p.add_argument('--inputs',type=Path,required=True);p.add_argument('--wasm-receipt',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
def sha(b):return hashlib.sha256(b).hexdigest()
corpus=json.loads(a.inputs.read_text());oracle=json.loads(a.wasm_receipt.read_text());rows=[];failures=[]
for index,g in enumerate(oracle['groups']):
 stdin=('\n'.join(corpus[g['language']][g['first']:g['first']+g['count']])+'\n').encode();assert sha(stdin)==g['inputSha256']
 with tempfile.TemporaryDirectory(prefix='morph-rust-native-')as d:
  argv=['-T']+(['-L']if g['language']=='lat'else[])
  if g['operation']=='cruncher-file':Path(d,'input.words').write_bytes(stdin);argv+=['input']
  r=subprocess.run([a.binary.resolve(),*argv],cwd=d,env=dict(os.environ,MORPHLIB=str(a.data.resolve()),LC_ALL='C',TZ='UTC'),input=stdin,capture_output=True,timeout=20)
  actual={'stdout':sha(r.stdout),'stderr':sha(r.stderr),'termination':{'kind':'exit','code':r.returncode}if r.returncode>=0 else{'kind':'signal','signal':-r.returncode},'files':{n:sha(Path(d,n).read_bytes())for n in ['input.morph','input.failed','input.stats']if Path(d,n).exists()}}
  equal=actual==g['c'];rows.append({'group':index,'equal':equal,'actual':actual})
  if not equal:failures.append(index)
 if index%400==0:print(index,'groups,',len(failures),'failures',flush=True)
report={'binarySha256':sha(a.binary.read_bytes()),'referenceReceiptSha256':sha(a.wasm_receipt.read_bytes()),'inputSha256':sha(a.inputs.read_bytes()),'groups':rows,'failures':failures,'passed':not failures};a.output.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'groups':len(rows),'failures':failures}));raise SystemExit(bool(failures))
