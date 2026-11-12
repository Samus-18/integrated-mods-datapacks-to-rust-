#!/usr/bin/env python3
"""Independent offline tests; does not call build script.

Check ZIP integrity, allowed diff, loot NBT integrity, vanilla End City preserved,
Stellarity absence, mountain/density byte identity, bastion anchoring and
new-world rarity. Runtime placement and TPS cannot be inferred here.
"""
import sys,zipfile,json,hashlib,collections,glob,math
from pathlib import Path
sys.path.insert(0,'/mnt/data/thalvoryn_v67_terrain_loot')
from nbt_codec_v67 import read_nbt
BASE=Path('/mnt/data/thalvoryn_v67_terrain_loot/PRIVATE_TEST')
OUT=Path('/mnt/data/Thalvoryn_V67_WORLD_HOTFIX')
origins={
 ('RUST',1024):Path('/mnt/data/v67_world_analysis/world/datapacks/Thalvoryn-RUST-Nether1024-V67-GROUND-LOOT-TEST.zip'),
 ('RUST',2032):BASE/'Thalvoryn-RUST-Nether2032-V67-GROUND-LOOT-TEST.zip',
 ('JAVA',1024):BASE/'Thalvoryn-JAVA-Nether1024-V67-GROUND-LOOT-TEST.zip',
 ('JAVA',2032):BASE/'Thalvoryn-JAVA-Nether2032-V67-GROUND-LOOT-TEST.zip',
}
RARITIES={
 'caves/trek_dripstone_room':650,
 'caves/trek_dripstone_room_upper_rc42':900,
 'caves/trek_lush_room':700,
 'caves/trek_lush_room_upper_rc42':950,
}
def strict_json(pairs,name):
 d={}
 for k,v in pairs:
  if k in d:raise ValueError('Duplicate JSON field '+str(k)+' in '+name)
  d[k]=v
 return d
def bnames(z):return set(z.namelist())
VERIFIED=[]
fixed_signatures=collections.Counter()
for p in glob.glob('/mnt/data/v67_world_analysis/overworld_fast_scan_*.json'):
 d=json.load(open(p));
 for e in d['containers']:
  items=e.get('Items')
  if isinstance(items,list) and len(items)==4:
   k=tuple(sorted(item.get('id','') for item in items));fixed_signatures[k]+=1
source_ids=sorted(fixed_signatures.items(),key=lambda x:x[1],reverse=True)
assert sum(n for _,n in source_ids[:2])>=9400
WORLD={'chunks_overworld':27643,'chunks_nether':2917,'chunks_end':2664,'chests_fixed_bone_nugget_cave_patterns':sum(n for _,n in source_ids[:2]),'pattern_examples':[{'items':list(k),'count':n} for k,n in source_ids[:2]],'observed_end_structures':0,'static_error_warning':'World has previous chunks; older generated structures do not update automatically. Static QA does not guarantee FPS, TPS, portals, collision-free jigsaw placement or natural cave connections.'}
(OUT/'WORLD_EVIDENCE.json').write_text(json.dumps(WORLD,indent=2,ensure_ascii=False)+'\n')
changed_paths_by_variant={};nbt_fingerprints={};canonical=[]
for (platform,height),src in origins.items():
 dest=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-WORLD-ROOTFIX-TEST.zip'
 with zipfile.ZipFile(src) as old,zipfile.ZipFile(dest) as new:
  assert old.testzip() is None and new.testzip() is None
  a=old.namelist();b=new.namelist();assert len(b)==11323 and a==b and len(b)==len(set(b))
  changed=[n for n in b if old.getinfo(n).CRC!=new.getinfo(n).CRC]
  assert len(changed)==17,(platform,height,changed)
  changed_paths_by_variant[f'{platform}_{height}']=changed
  assert not any('/worldgen/density_function/' in n or '/worldgen/noise_settings/' in n or '/worldgen/biome/' in n for n in changed)
  assert all(not n.startswith('data/stellarity/') for n in b)
  for n in b:
   if n.endswith(('.json','.mcmeta')):
    json.loads(new.read(n),object_pairs_hook=lambda pairs:strict_json(pairs,n))
  pack=json.loads(new.read('pack.mcmeta'))
  assert pack['pack']['pack_format']==121
  dt=json.loads(new.read('data/minecraft/dimension_type/the_nether.json'))
  assert dt['height']==height
  # No smuggled active Stellarity roots
  for n in b:
   if n.endswith('.json') and '/worldgen/biome/' in n:
    assert 'stellarity:' not in new.read(n).decode('utf8').lower(),n
  for suffix,ch in RARITIES.items():
   j=json.loads(new.read(f'data/thalvoryn/worldgen/placed_feature/{suffix}.json'))
   vals=[x['chance'] for x in j['placement'] if x['type']=='minecraft:rarity_filter']
   assert vals==[ch],(suffix,vals)
  s=json.loads(new.read('data/thalvoryn/worldgen/structure_set/ecology/ow_spawners.json'))
  assert s['placement']['spacing']==42 and s['placement']['frequency']==.31
  t=json.loads(new.read('data/thalvoryn/worldgen/structure_set/ecology/ow_cave_entrances.json'))
  assert len(t['structures'])==6 and t['placement']['spacing']==29
  bastion=json.loads(new.read('data/minecraft/worldgen/structure/bastion_remnant.json'))
  assert bastion['start_pool']=='minecraft:bastion/starts' and bastion['type']=='minecraft:jigsaw'
  assert bastion['project_start_to_heightmap']=='WORLD_SURFACE_WG' and bastion['start_height']=={'absolute':-9}
  assert bastion['terrain_adaptation']=='beard_thin'
  if platform=='RUST':
   checks=bastion['horizon:start_checks']
   assert checks['liquid']=='top' and checks['terrain_radius']==2 and checks['terrain_range']==46 and checks['max_y']==(900 if height==1024 else 1830)
  else: assert 'horizon:start_checks' not in bastion
  end_major=json.loads(new.read('data/thalvoryn/worldgen/structure_set/ecology/end_major.json'))
  ids=[x['structure'] for x in end_major['structures']]
  assert 'trek:end/medium/end_city' not in ids and 'nova_structures:end_castle' in ids
  ec=json.loads(new.read('data/minecraft/worldgen/structure_set/end_cities.json'))
  assert ec==json.loads(old.read('data/minecraft/worldgen/structure_set/end_cities.json'))
  for n in [
   'data/trek/worldgen/template_pool/end/medium/end_city/ship.json',
   'data/trek/worldgen/template_pool/end/medium/end_city/ship_bridge.json',
   'data/trek/worldgen/template_pool/end/medium/end_city/ship_void.json',
   'data/nova_structures/worldgen/template_pool/end_castle/boat.json',
   'data/nova_structures/worldgen/template_pool/end_castle/docks.json',
   'data/nova_structures/worldgen/template_pool/end_castle/docks_fallback.json']:
   d=json.loads(new.read(n));assert len(d['elements'])==1 and d['elements'][0]['element']['element_type']=='minecraft:empty_pool_element'
  # Confirm actual root template fixed loot replaced, preserving every block state and coordinate.
  modified_nbt=[n for n in changed if n.endswith('.nbt')]
  assert len(modified_nbt)>=1
  c=0
  for n in modified_nbt:
   _,original=read_nbt(old.read(n));_,modified=read_nbt(new.read(n))
   assert original['size']==modified['size']
   assert original['palette']==modified['palette']
   olds=original['blocks'].value[1];news=modified['blocks'].value[1]
   assert len(olds)==len(news)
   for lhs,rhs in zip(olds,news):
    assert lhs['pos']==rhs['pos'] and lhs['state']==rhs['state']
    a=lhs.get('nbt');changed_tag=rhs.get('nbt')
    if a != changed_tag:
     assert a is not None and changed_tag is not None
     a=a.value;edited=changed_tag.value
     assert 'Items' in a and len(a['Items'].value[1])>0 and 'Items' not in edited
     assert edited['LootTable'].value.startswith('thalvoryn:chests/rc37/overworld/')
     assert 'LootTableSeed' not in edited
     dname='data/'+edited['LootTable'].value.replace(':','/loot_table/')+'.json'
     assert dname in bnames(new)
     c+=1
   assert original['entities']==modified['entities']
  assert c==10,(platform,height,c)
  digest=hashlib.sha256()
  for n in sorted(n for n in b if n.endswith('.nbt')):digest.update(n.encode());digest.update(new.read(n))
  nbt_fingerprints[f'{platform}_{height}']=digest.hexdigest()
  VERIFIED.append({'platform':platform,'nether_height':height,'entries':len(b),'changed_files':len(changed),'fixed_template_containers_converted':c,'file_sha256':hashlib.sha256(dest.read_bytes()).hexdigest(),'zip_crc':'PASS','data_format':121,'game_runtime':'NOT TESTED'})
assert len(set(nbt_fingerprints.values()))==1,nbt_fingerprints
assert sorted(changed_paths_by_variant['RUST_1024'])==sorted(changed_paths_by_variant['RUST_2032'])==sorted(changed_paths_by_variant['JAVA_1024'])==sorted(changed_paths_by_variant['JAVA_2032'])
report={'status':'PASS_STATIC_ONLY_USER_SAVE_ANALYZED','world_evidence':WORLD,'variants':VERIFIED,'same_changed_paths_all_four':True,'nbt_identical_across_variants':True,'world_generation_runtime':'NOT TESTED','cautions':['Existing chunks unaffected by datapack change','Natural cave connection not guaranteed by structure template','Nether fortress special generator remains vanilla','World save itself not modified']}
(OUT/'QA_REPORT.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print('PASS: world-grounded static QA, 4 variants, 17 source diffs each, all 10 fixed template inventories now loot tables')

# helpers