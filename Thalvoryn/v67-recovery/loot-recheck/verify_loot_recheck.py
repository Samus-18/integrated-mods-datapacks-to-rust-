"""Independent comparison (not part of build): verify no geometry/state/entity changes, only conversion from prefilled Items to loot tables."""
from pathlib import Path
import sys,json,zipfile,collections,hashlib
sys.path.insert(0,'/mnt/data/thalvoryn_v67_terrain_loot')
from nbt_codec_v67 import read_nbt
ROOT=Path('/mnt/data/Thalvoryn_V67_AUDIT');BASE=Path('/mnt/data/Thalvoryn_V67_WORLD_HOTFIX');A,B=map(int,sys.argv[1:3])
source=zipfile.ZipFile(BASE/'Thalvoryn-RUST-Nether1024-V67-WORLD-ROOTFIX-TEST.zip')
new=zipfile.ZipFile(ROOT/'Thalvoryn-RUST-Nether1024-V67-LOOT-RECHECK-TEST.zip')
changed=sorted(n for n in source.namelist() if source.getinfo(n).CRC!=new.getinfo(n).CRC)
assert len(changed)==502 and all(n.endswith('.nbt') for n in changed),len(changed)
counts=collections.Counter();modified=[]
for name in changed[A:B]:
 _,a=read_nbt(source.read(name));_,b=read_nbt(new.read(name))
 assert a.keys()==b.keys() and a['size']==b['size'] and a['palette']==b['palette'] and a['entities']==b['entities'],name
 assert all(a[k]==b[k] for k in a if k!='blocks'),name
 ca=a['blocks'].value[1];cb=b['blocks'].value[1]
 assert len(ca)==len(cb),name
 for aa,bb in zip(ca,cb):
  assert aa['pos']==bb['pos'] and aa['state']==bb['state'] and aa.keys()==bb.keys(),name
  if aa==bb:continue
  ar=aa['nbt'].value;br=bb['nbt'].value
  assert ar['id'].value in ('minecraft:barrel','minecraft:chest','minecraft:trapped_chest'),name
  assert 'Items' in ar and len(ar['Items'].value[1])>0 and 'LootTable' not in ar,name
  assert 'Items' not in br and 'LootTable' in br and br['LootTable'].typ==8,name
  assert {k:v for k,v in ar.items() if k not in ('Items','LootTableSeed')} == {k:v for k,v in br.items() if k!='LootTable'},name
  loot='data/'+br['LootTable'].value.replace(':','/loot_table/')+'.json'
  assert loot in new.namelist(),(name,loot)
  counts[ar['id'].value]+=1
 modified.append(name)
report={'status':'INDEPENDENT STATIC QA ONLY','range':[A,B],'templates_checked':len(modified),'converted_containers':sum(counts.values()),'block_entity_counts':counts,'all_other_nbt_fields_unchanged':True}
(ROOT/f'verify_{A}_{B}.json').write_text(json.dumps(report,indent=2,default=int)+'\n')
print('PASS',A,B,'checked',len(modified),'converted',report['converted_containers'],'by ID',dict(counts))