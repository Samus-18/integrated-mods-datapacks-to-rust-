#!/usr/bin/env python3
"""Static full reachable NBT jigsaw / template-pool and loot reference audit.
Each NBT is individually visited once, with constant memory per file.
"""
import sys,zipfile,json,gzip,struct,collections,time,hashlib
from pathlib import Path
SOURCE=Path(sys.argv[1]) if len(sys.argv)>1 else next(Path(__file__).parent.glob('*RUST*1024*.zip'))
REPORT=Path(sys.argv[2]) if len(sys.argv)>2 else Path(__file__).parent/'jigsaw_graph_report.json'
start=time.time()
z=zipfile.ZipFile(SOURCE)
names=set(z.namelist())
js={}
for n in names:
 if n.endswith('.json') and ('/worldgen/template_pool/' in n or '/worldgen/structure_set/' in n or '/worldgen/structure/' in n or '/worldgen/placed_feature/' in n):
  js[n]=json.loads(z.read(n))

def pth(kind,ref,ext='.json'):
 if not isinstance(ref,str) or not ref:return ''
 if ref.startswith('#'):ref=ref[1:]
 ns,r=(ref.split(':',1) if ':' in ref else ('minecraft',ref))
 return f'data/{ns}/worldgen/{kind}/{r}{ext}'

def nbtpath(ref):
 ns,r=(ref.split(':',1) if ':' in ref else ('minecraft',ref))
 return f'data/{ns}/structure/{r}.nbt'

def findstrings(raw,tag):
 mark=b'\x08'+struct.pack('>H',len(tag))+tag.encode()
 start=0
 while 1:
  i=raw.find(mark,start)
  if i<0:break
  off=i+len(mark)
  if off+2>len(raw):break
  num=struct.unpack_from('>H',raw,off)[0]
  if num<256 and off+2+num<=len(raw):
   s=raw[off+2:off+2+num].decode('utf8',errors='replace')
   if s and all(c.isprintable() for c in s):yield s
  start=off+2

def locs(v):
 if not isinstance(v,dict):return
 t=v.get('element_type','')
 if t in ('minecraft:single_pool_element','minecraft:legacy_single_pool_element'):
  yield v.get('location')
 elif t=='minecraft:list_pool_element':
  for a in v.get('elements',[]):yield from locs(a)

def reftype(s):return isinstance(s,str) and ':' in s and not s.startswith('minecraft:')
ref_sets={}
for file,val in js.items():
 if '/worldgen/structure_set/' in file:
  for e in val.get('structures',[]):
   x=e.get('structure')
   if x:ref_sets.setdefault(x,[]).append(file)
structures={}
for file,val in js.items():
 if '/worldgen/structure/' in file and '/tags/' not in file:
  structures[file.split('/')[1]+':'+file.split('/worldgen/structure/')[1][:-5]]=val
starts=set();unresolved_starts=[]
for name in ref_sets:
 s=structures.get(name)
 if s and s.get('type')=='minecraft:jigsaw':
  id=s.get('start_pool')
  if id:starts.add(id)
  if id and pth('template_pool',id) not in names and reftype(id):unresolved_starts.append((name,id))
queue=collections.deque(starts)
seenp=set();visitedn=set();missing_templates=set();missing_pools=set();traversed_edges=0
missing_loot=set();jigsaw_count=0;nbt_visited=0;max_graph_degree=collections.Counter()
while queue:
 pid=queue.popleft()
 if pid in seenp or pid=='minecraft:empty':continue
 seenp.add(pid)
 file=pth('template_pool',pid)
 if file not in names:
  if reftype(pid):missing_pools.add(pid)
  continue
 obj=js[file]
 cand=[obj.get('fallback')]
 if isinstance(obj.get('elements'),list):
  for el in obj['elements']:
   for template_id in locs(el.get('element',{})):
    if not template_id or template_id=='minecraft:empty':continue
    n=nbtpath(template_id)
    if n not in names:
     if reftype(template_id):missing_templates.add((pid,template_id))
     continue
    if n in visitedn:continue
    visitedn.add(n)
    raw=z.read(n)
    if raw[:2]==b'\x1f\x8b':raw=gzip.decompress(raw)
    arr=set(findstrings(raw,'pool'))
    for x in arr:
     if (':' in x or x=='empty') and x not in seenp:
      cand.append(x)
    # Untyped strings through NBT: only test presence of custom loot ids
    for lid in set(findstrings(raw,'LootTable')):
     if reftype(lid):
      lp='data/'+lid.split(':',1)[0]+'/loot_table/'+lid.split(':',1)[1]+'.json'
      if lp not in names:missing_loot.add((n,lid))
    jigsaw_count+=len(arr)
    nbt_visited+=1
    if nbt_visited%500==0:print(f'Visited {nbt_visited} NBT, {len(seenp)} pools in {time.time()-start:.1f}s',flush=True)
 for x in cand:
  if x and x!='minecraft:empty' and isinstance(x,str):
   traversed_edges+=1; queue.append(x)
 if len(seenp)>5000:raise RuntimeError('Unexpected big pool graph')
report={'source':str(SOURCE),'status':'STATIC ONLY','elapsed_s':round(time.time()-start,2),'active_structure_names':len(ref_sets),'start_pool_count':len(starts),'reachable_pools':len(seenp),'reachable_nbt':nbt_visited,'jigsaw_connector_pool_observations':jigsaw_count,'graph_edges':traversed_edges,'unresolved_start_pools':unresolved_starts,'missing_reachable_custom_pools':sorted(missing_pools),'missing_reachable_custom_templates':sorted(missing_templates),'missing_reachable_custom_loot':sorted(missing_loot),'total_nbt_in_archive':sum(n.endswith('.nbt') for n in names)}
REPORT.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')
print(json.dumps({k:(v[:8] if isinstance(v,list) else v) for k,v in report.items()},indent=2,ensure_ascii=False))