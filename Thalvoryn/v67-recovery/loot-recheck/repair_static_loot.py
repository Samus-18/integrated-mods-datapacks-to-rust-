"""V67 conservative loot repair: only non-empty, static chest/barrel inventories in custom NBT structures.
Does not alter mechanisms, empty/decorative containers, vanilla, or structures geometry. No runtime claim.
"""
from pathlib import Path
import sys,json,zipfile,gzip,hashlib,collections,copy,time
sys.path.insert(0,'/mnt/data/thalvoryn_v67_terrain_loot')
from nbt_codec_v67 import read_nbt,write_nbt,t
BASE=Path('/mnt/data/Thalvoryn_V67_WORLD_HOTFIX/Thalvoryn-RUST-Nether1024-V67-WORLD-ROOTFIX-TEST.zip')
OUT=Path('/mnt/data/Thalvoryn_V67_AUDIT')
A,B=map(int,sys.argv[1:3]); t0=time.monotonic()
with zipfile.ZipFile(BASE) as source:
 nbt=sorted(n for n in source.namelist() if n.endswith('.nbt'))
 loot={n.split('/loot_table/',1)[1][:-5].join([]) for n in []} # populated below
 loot=set(n for n in source.namelist() if '/loot_table/' in n and n.endswith('.json'))
 result=collections.Counter();modified={};changes=[]
 def pick(path,entity_id,pos):
  x=path.lower();kind='barrel' if entity_id.endswith(':barrel') else 'chest';seed=hashlib.sha256((path+str(pos)).encode()).digest()[0]
  if '/the_end/' in x or '/end/' in x or '/end_' in x:
   name='storage_rare_midlands' if kind=='barrel' else ('ruin_rare_midlands' if seed%3 else 'generic_rare_midlands');dim='end'
  elif '/nether/' in x or '/nether_' in x:
   name=('storage_uncommon_crimson' if seed%3==0 else 'storage_uncommon_basalt' if seed%3==1 else 'storage_uncommon_soul') if kind=='barrel' else ('fortress_uncommon_crimson' if any(k in x for k in ('fortress','bastion','castle')) else 'generic_uncommon_wastes');dim='nether'
  else:
   dim='overworld'
   if 'deep_dark' in x or 'ancient_city' in x: name='storage_common_deep_dark' if kind=='barrel' else 'cave_common_deep_dark'
   elif any(k in x for k in ('cave','underground','monster_room','dungeon','mineshaft','dripstone')):name='storage_common_lush' if kind=='barrel' else 'cave_common_dripstone'
   elif any(k in x for k in ('ship','vessel','port','ocean','cove','sunken')): name='storage_common_ocean' if kind=='barrel' else 'ship_uncommon_ocean'
   elif any(k in x for k in ('castle','fort','fortress','mansion','stronghold')):name='storage_common_plains' if kind=='barrel' else ('fortress_uncommon_plains' if seed%5==0 else 'fortress_common_plains')
   elif any(k in x for k in ('village','farm','cottage','house','manor','trade','cabin')):name='storage_common_plains' if kind=='barrel' else 'settlement_common_plains'
   elif 'jungle' in x:name='storage_common_jungle' if kind=='barrel' else 'generic_common_plains'
   else:name='storage_common_plains' if kind=='barrel' else 'generic_common_plains'
  table=f'thalvoryn:chests/rc37/{dim}/{name}'
  check=f'data/thalvoryn/loot_table/chests/rc37/{dim}/{name}.json'
  assert check in loot,(path,check)
  return table
 with zipfile.ZipFile(OUT/f'loot_patch_{A}_{B}.zip','w',compression=zipfile.ZIP_DEFLATED,compresslevel=6) as patches:
  for name in nbt[A:B]:
   if name.startswith('data/minecraft/structure/'):continue
   blob=source.read(name);raw=gzip.decompress(blob) if blob[:2]==b'\x1f\x8b' else blob
   if b'Items' not in raw:continue
   nm,root=read_nbt(blob);q=0
   for block in root['blocks'].value[1]:
    be=block.get('nbt')
    if not be or be.typ!=10:continue
    d=be.value;id=d.get('id')
    if id is None or id.value not in ('minecraft:chest','minecraft:trapped_chest','minecraft:barrel'):continue
    if 'LootTable' in d:continue
    items=d.get('Items');
    if items is None or items.typ!=9 or len(items.value[1])==0:continue
    lt=pick(name,id.value,block['pos'].value[1]);del d['Items'];d.pop('LootTableSeed',None);d['LootTable']=t(8,lt)
    result[id.value]+=1;q+=1
   if q:
    new=write_nbt(nm,root,compressed=blob[:2]==b'\x1f\x8b')
    assert new!=blob
    _,check=read_nbt(new)
    assert check==root
    patches.writestr(name,new)
    changes.append({'template':name,'containers':q,'original_sha256':hashlib.sha256(blob).hexdigest(),'new_sha256':hashlib.sha256(new).hexdigest()})
 report={'status':'PRIVATE STATIC REPAIR, NOT GAME TESTED','from':BASE.name,'range':[A,B],'patch_files':len(changes),'fixed_containers':sum(result.values()),'by_block_entity_id':dict(result),'changes':changes,'elapsed_seconds':round(time.monotonic()-t0,2)}
 (OUT/f'loot_patch_{A}_{B}.json').write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')
 print('PATCH',A,B,'templates',len(changes),'containers',report['fixed_containers'],'seconds',report['elapsed_seconds'])