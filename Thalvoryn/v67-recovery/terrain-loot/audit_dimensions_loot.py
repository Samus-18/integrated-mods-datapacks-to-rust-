import zipfile,glob,json,collections,gzip,struct,pathlib
p=glob.glob('/mnt/data/thalvoryn_v67_terrain_loot/PRIVATE_TEST/*RUST*1024*.zip')[0]
with zipfile.ZipFile(p) as z:
 names=set(z.namelist());present=collections.defaultdict(set);fail=[];warn=[]
 for name in sorted(n for n in names if '/worldgen/structure_set/' in n and n.endswith('.json')):
  s=json.loads(z.read(name));part=name.split('structure_set/')[1]
  kind=('nether' if 'nether' in part else 'end' if 'end' in part else 'overworld')
  if 'spacing' in s['placement']:
   pl=s['placement'];assert pl['spacing']>pl['separation']>=0
  for st in s.get('structures',[]):
   ns,key=st['structure'].split(':',1);fn=f'data/{ns}/worldgen/structure/{key}.json'
   if fn not in names:
    if ns=='minecraft':warn.append(('vanilla_registry',st['structure']))
    else:fail.append(('missing_structure_def',st['structure']))
    continue
   present[kind].add(st['structure'])
   d=json.loads(z.read(fn))
   if d.get('type')=='minecraft:jigsaw':
    po=d['start_pool'];ns2,k2=po.split(':',1);pf=f'data/{ns2}/worldgen/template_pool/{k2}.json'
    if pf not in names and ns2!='minecraft':fail.append(('missing_start_pool',st['structure'],po))
    if kind in ('end','nether') and any(a in st['structure'] for a in ('ship','pirate','galleon','frigate')):
     fail.append(('nonvanilla_ship_active_wrong_dimension',kind,st['structure']))
 print('DIM COUNTS',dict((k,len(v)) for k,v in present.items()),'fail',len(fail),'warnings',len(warn));print('fail samples',fail[:20])
 # binary scan for actual LootTable IDs in all NBT, not just display tables
 import collections
 lootrefs=collections.Counter();invalid=[]
 pref=b'\x08\x00\x09LootTable'
 for n in names:
  if not n.endswith('.nbt'):continue
  raw=z.read(n); raw=gzip.decompress(raw) if raw[:2]==b'\x1f\x8b' else raw
  k=0
  while (idx:=raw.find(pref,k))!=-1:
   pos=idx+len(pref);size=struct.unpack_from('>H',raw,pos)[0]
   if size<350:
    val=raw[pos+2:pos+2+size].decode('utf8','replace');lootrefs[val]+=1
   k=pos+2+size
 for name,count in lootrefs.items():
  if ':' not in name:invalid.append(('unqualified',name,count));continue
  ns,key=name.split(':',1)
  if ns=='minecraft':continue
  if f'data/{ns}/loot_table/{key}.json' not in names:
   invalid.append(('missing',name,count))
 print('LOOT UNIQUE',len(lootrefs),'refs',sum(lootrefs.values()),'BAD',len(invalid),invalid[:35]);
 data={'dimension_active':dict((k,len(v)) for k,v in present.items()),'structure_reference_errors':fail,'vanilla_reference_warnings':warn,'loot_table_reference_count':sum(lootrefs.values()),'loot_table_unique':len(lootrefs),'missing_own_loot_refs':invalid}
 pathlib.Path('/mnt/data/thalvoryn_v67_terrain_loot/PRIVATE_TEST/END_NETHER_LOOT_AUDIT.json').write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n')
 assert not fail
 assert not invalid