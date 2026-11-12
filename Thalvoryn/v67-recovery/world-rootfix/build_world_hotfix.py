#!/usr/bin/env python3
"""V67 user-save-grounded conservative repair.
Edits only explicitly audited resources; keeps all existing NBT geometry,
worldgen density and unrelated assets byte-identical.
No game runtime test claimed. Private test only; third-party assets in pack.
"""
import sys, zipfile, json, hashlib, copy, time, collections, os
from pathlib import Path
sys.path.insert(0,'/mnt/data/thalvoryn_v67_terrain_loot')
from nbt_codec_v67 import read_nbt,write_nbt,t

ROOT=Path('/mnt/data')
BASE=ROOT/'thalvoryn_v67_terrain_loot/PRIVATE_TEST'
SAVE=ROOT/'v67_world_analysis/world/datapacks/Thalvoryn-RUST-Nether1024-V67-GROUND-LOOT-TEST.zip'
OUT=ROOT/'Thalvoryn_V67_WORLD_HOTFIX'
VARIANTS={
  ('RUST',1024):SAVE,
  ('RUST',2032):BASE/'Thalvoryn-RUST-Nether2032-V67-GROUND-LOOT-TEST.zip',
  ('JAVA',1024):BASE/'Thalvoryn-JAVA-Nether1024-V67-GROUND-LOOT-TEST.zip',
  ('JAVA',2032):BASE/'Thalvoryn-JAVA-Nether2032-V67-GROUND-LOOT-TEST.zip',
}
J=lambda d:(json.dumps(d,indent=2,ensure_ascii=False,sort_keys=True)+'\n').encode()
PLACED_RARITIES={
 'data/thalvoryn/worldgen/placed_feature/caves/trek_dripstone_room.json':650,
 'data/thalvoryn/worldgen/placed_feature/caves/trek_dripstone_room_upper_rc42.json':900,
 'data/thalvoryn/worldgen/placed_feature/caves/trek_lush_room.json':700,
 'data/thalvoryn/worldgen/placed_feature/caves/trek_lush_room_upper_rc42.json':950,
}
DISABLE_POOLS=[
 'data/trek/worldgen/template_pool/end/medium/end_city/ship.json',
 'data/trek/worldgen/template_pool/end/medium/end_city/ship_bridge.json',
 'data/trek/worldgen/template_pool/end/medium/end_city/ship_void.json',
 'data/nova_structures/worldgen/template_pool/end_castle/boat.json',
 'data/nova_structures/worldgen/template_pool/end_castle/docks.json',
 'data/nova_structures/worldgen/template_pool/end_castle/docks_fallback.json',
]
EMPTY_POOL={'fallback':'minecraft:empty','elements':[{'element':{'element_type':'minecraft:empty_pool_element'},'weight':1}]}


def choose_loot(path,ordinal):
 p=path.lower()
 prefix='thalvoryn:chests/rc37/overworld/'
 if 'dripstone' in p:return prefix+('cave_common_dripstone' if ordinal%3 else 'cave_rare_dripstone')
 if 'deep_dark' in p:return prefix+'cave_common_deep_dark'
 if 'lush' in p:return prefix+'storage_common_lush'
 return prefix+'generic_common_plains'


def process(platform,height):
 source=VARIANTS[(platform,height)]
 assert source.is_file(),source
 output=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-WORLD-ROOTFIX-TEST.zip'
 rewrites={};repair_log={'platform':platform,'nether_height':height,'source':source.name,'changed_files':[],'converted_fixed_items':[],'rarity':{},'end_ships_disabled':[],'bastion':{},'nether_pillars':[]}
 with zipfile.ZipFile(source) as z:
  names=z.namelist();keys=set(names)
  assert len(names)==len(keys),(len(names),len(keys))
  assert len(names)==11323,len(names)
  def jread(n):return json.loads(z.read(n))
  def jset(n,d):
   assert n in keys,n
   dat=J(d)
   if dat != z.read(n):rewrites[n]=dat
  # Disable aggressive feature dungeons directly responsible for the flood of identical chests.
  for n,chance in PLACED_RARITIES.items():
   d=jread(n);rarity=[p for p in d['placement'] if p.get('type')=='minecraft:rarity_filter']
   assert len(rarity)==1,(n,rarity)
   old=rarity[0]['chance'];assert old in (5,6,7,9),(n,old)
   rarity[0]['chance']=chance
   jset(n,d);repair_log['rarity'][n]={'old_chance':old,'new_chance':chance}
  # Keep a separate rare-dungeon structure set without overlapping feature spam.
  n='data/thalvoryn/worldgen/structure_set/ecology/ow_spawners.json';d=jread(n)
  assert d['placement']['spacing']==30 and d['placement']['frequency']==.27
  d['placement'].update(spacing=42,separation=17,frequency=.31)
  jset(n,d)
  # Leave six separate large cave tunnel templates intact; frequency ≈ one candidate per 450-650 blocks,
  # not six independently spawning grids.
  n='data/thalvoryn/worldgen/structure_set/ecology/ow_cave_entrances.json';d=jread(n)
  assert len(d['structures'])==6 and d['placement']['spacing']==30
  d['placement'].update(spacing=29,separation=12,frequency=.68)
  jset(n,d)
  # Fix floating bastions: existing 90..704 absolute start height ignored terrain height.
  # Use the same vanilla bastion pool and pieces, but anchor starts to the surface.
  n='data/minecraft/worldgen/structure/bastion_remnant.json';d=jread(n)
  assert d['type']=='minecraft:jigsaw' and d['start_pool']=='minecraft:bastion/starts'
  prior=d['start_height'];assert isinstance(prior,dict)
  d['project_start_to_heightmap']='WORLD_SURFACE_WG'
  d['start_height']={'absolute':-9}
  d['terrain_adaptation']='beard_thin'
  if platform=='RUST':d['horizon:start_checks']={'liquid':'top','terrain_radius':2,'terrain_range':46,'min_y':48,'max_y':(900 if height==1024 else 1830)}
  else:d.pop('horizon:start_checks',None)
  jset(n,d)
  repair_log['bastion']={'previous_start_height':prior,'surface_projected':True,'start_height_offset':-9,'horizon_checks':d.get('horizon:start_checks'),'default_pieces_preserved':True}
  # Previously active bespoke Trek End City produces a wooden wool-sail ship. Vanilla End City is kept.
  n='data/thalvoryn/worldgen/structure_set/ecology/end_major.json';d=jread(n)
  old=[s['structure'] for s in d['structures']]
  assert 'trek:end/medium/end_city' in old
  d['structures']=[entry for entry in d['structures'] if entry['structure']!='trek:end/medium/end_city']
  assert len(d['structures'])==len(old)-1
  jset(n,d)
  for n in DISABLE_POOLS:
   d=jread(n)
   assert len(d.get('elements',[]))>=1,n
   d['elements']=copy.deepcopy(EMPTY_POOL['elements'])
   d['fallback']='minecraft:empty'
   jset(n,d);repair_log['end_ships_disabled'].append(n)
  # TREK fixed 4-item cave chests become actual separate loot tables; preserve decoration inventories elsewhere.
  changed_nbt=0;converted=0
  for n in names:
   if not (n.startswith('data/trek/structure/monster_room/') and n.endswith('.nbt')):continue
   name,root=read_nbt(z.read(n))
   if 'blocks' not in root:continue
   ordinal=0;replaced=0
   for block in root['blocks'].value[1]:
    nbt=block.get('nbt')
    if not nbt or nbt.typ!=10:continue
    entity=nbt.value
    ident=entity.get('id')
    if not ident or ident.value not in ['minecraft:chest','minecraft:barrel','minecraft:trapped_chest']:continue
    items=entity.get('Items')
    if items is None or items.typ!=9 or len(items.value[1])==0:continue
    if entity.get('LootTable') is not None:continue
    ordinal+=1
    lt=choose_loot(n,ordinal)
    target='data/'+lt.replace(':','/loot_table/')+'.json'
    assert target in keys,(n,lt,target)
    del entity['Items']
    entity.pop('LootTableSeed',None)
    entity['LootTable']=t(8,lt)
    repaired= {'template':n,'loot':lt,'pos':block['pos'].value[1]}
    repair_log['converted_fixed_items'].append(repaired)
    converted+=1;replaced+=1
   if replaced:
    rewrites[n]=write_nbt(name,root,compressed=z.read(n).startswith(b'\x1f\x8b'))
    changed_nbt+=1
  assert any(i['template']=='data/trek/structure/monster_room/dripstone.nbt' for i in repair_log['converted_fixed_items'])
  # Note: other static loot in structures might be intentional; do not mass overwrite.
  pack=jread('pack.mcmeta');pack['pack']['description']=f'Thalvoryn V67 {platform} Nether{height} WORLD ROOTFIX PRIVATE TEST'
  jset('pack.mcmeta',pack)
  # Reserve all resource bytes and original order for safe diffing.
  with zipfile.ZipFile(output,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True) as out:
   for info in z.infolist():
    b=rewrites.get(info.filename)
    if b is None:b=z.read(info.filename)
    out.writestr(info,b)
  repair_log['changed_files']=sorted(rewrites)
  repair_log['changed_file_count']=len(rewrites)
  repair_log['fixed_inventory_to_loot_count']=converted
  repair_log['modified_nbt_template_count']=changed_nbt
  repair_log['files_per_zip']=len(names)
  repair_log['source_sha256']=hashlib.sha256(source.read_bytes()).hexdigest()
  repair_log['output_sha256']=hashlib.sha256(output.read_bytes()).hexdigest()
  repair_log['output_bytes']=output.stat().st_size
  repair_log['output_name']=output.name
  print('BUILT',platform,height,'changed',len(rewrites),'chests converted',converted,'size',output.stat().st_size,flush=True)
 return repair_log

if __name__=='__main__':
 start=time.time();logs=[]
 for platform,height in [('RUST',1024),('RUST',2032),('JAVA',1024),('JAVA',2032)]:logs.append(process(platform,height))
 (OUT/'BUILD_MANIFEST.json').write_text(json.dumps({'status':'OFFLINE_TEST_BUILD; NO RUNTIME GUARANTEE','variants':logs},indent=2,ensure_ascii=False)+'\n')
 print('DONE all',round(time.time()-start,1),'sec')