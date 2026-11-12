#!/usr/bin/env python3
"""Independent V69 static guardrails, not an in-game performance or worldgen proof."""
from pathlib import Path
import zipfile,json,hashlib,collections,sys
BASE=Path('/mnt/data/v68_work/v68_release_candidate')
OUT=Path('/mnt/data/v68_work/v69_release_candidate')
VARIANTS={
  'RUST_1024':('Thalvoryn-RUST-Nether1024-V68-PRIVATE-TEST.zip','Thalvoryn-RUST-Nether1024-V69-PRIVATE-TEST.zip'),
  'RUST_2032':('Thalvoryn-RUST-Nether2032-V68-PRIVATE-TEST.zip','Thalvoryn-RUST-Nether2032-V69-PRIVATE-TEST.zip'),
  'JAVA_1024':('Thalvoryn-JAVA-Nether1024-V68-PRIVATE-TEST.zip','Thalvoryn-JAVA-Nether1024-V69-PRIVATE-TEST.zip'),
  'JAVA_2032':('Thalvoryn-JAVA-Nether2032-V68-PRIVATE-TEST.zip','Thalvoryn-JAVA-Nether2032-V69-PRIVATE-TEST.zip')
}
allowed={
  'pack.mcmeta','data/thalvoryn/function/version.mcfunction',
  'data/thalvoryn/function/check.mcfunction',
  'data/thalvoryn/function/help.mcfunction',
  'data/thalvoryn/function/horizon/status.mcfunction',
  'data/trek/worldgen/structure/nether/common/nether_pillar.json',
  'data/trek/worldgen/structure/nether/medium/overworld_leak_1.json',
  'data/trek/worldgen/structure/nether/medium/overworld_leak_2.json',
  'data/thalvoryn/worldgen/structure/nether/cinder_viaduct.json',
  'data/thalvoryn/worldgen/structure/nether/skybridge_ruin.json',
  'data/nova_structures/worldgen/structure/stray_fort.json',
  'data/nova_structures/worldgen/structure/toxic_lair.json',
  'data/nova_structures/worldgen/structure/sealing_halls.json',
  'data/nova_structures/worldgen/structure/end_castle.json'
}
report={'status':'PASS_STATIC_ONLY_NO_GAME_TEST','baseline':'V68 private candidate','variants':{},'warnings':['Horizon Rust and Minecraft Java game not launched','Chunk position, actual terrain blending, loot-opening behavior, TPS and FPS not validated','Horizon-only start checks do not work in Minecraft Java; terrain safety is best effort']}
for v,(previous,new) in VARIANTS.items():
 platform,h=v.split('_');h=int(h)
 with zipfile.ZipFile(BASE/previous) as old, zipfile.ZipFile(OUT/new) as z:
  assert old.testzip() is None and z.testzip() is None
  names=z.namelist();oldnames=old.namelist()
  assert len(names)==len(set(names)) and set(names)==set(oldnames)|{'V69_CHANGELOG_CZ.txt'}
  assert not any(n.startswith('data/stellarity/') for n in names)
  changes=[];untouched=0;json_count=0;nbt_count=0
  for n in names:
   if n=='V69_CHANGELOG_CZ.txt':continue
   oi=old.getinfo(n);zi=z.getinfo(n)
   if (oi.CRC,oi.file_size)!=(zi.CRC,zi.file_size):
      assert n in allowed or (platform=='RUST' and ('/worldgen/structure/nether/imported/' in n or '/worldgen/structure/nether/sites/' in n or '/worldgen/structure/end/imported/' in n or n.startswith('data/thunderfusion/worldgen/structure/end/'))), f'Unexpected change {v} {n}'
      changes.append(n)
   else:untouched+=1
   if n.endswith('.nbt'):nbt_count+=1
   if n.endswith(('.json','.mcmeta')):
     if n!= 'pack.mcmeta' and n in changes and n.startswith('data/'):
        j=json.loads(z.read(n));j0=json.loads(old.read(n))
        # Verify only documented attributes changed and no structure geometry fields or biomes were touched.
        if '/worldgen/structure/' in n:
          for k in ['type','size','max_distance_from_center','start_pool','biomes','step','pool_aliases','use_expansion_hack','spawn_overrides']:
           if k=='spawn_overrides' and n in allowed and n.startswith('data/nova_structures/'):
             continue
           assert j.get(k)==j0.get(k),(v,n,k)
     json.loads(z.read(n));json_count+=1
  assert nbt_count==6835, (v,nbt_count)
  assert all(n not in changes for n in names if n.endswith(('.nbt','.nbt.gz'))),v
  assert all(n not in changes for n in names if any(t in n for t in ('/loot_table/','/density_function/','/worldgen/noise_settings/','/worldgen/biome/','/worldgen/structure_set/','/worldgen/feature/','/worldgen/placed_feature/','/dimension/','/dimension_type/'))),v
  assert json.loads(z.read('pack.mcmeta'))['pack']['pack_format']==121
  assert 'V69' in z.read('data/thalvoryn/function/version.mcfunction').decode()
  # Explicit regression assertions
  for n in [x for x in allowed if '/trek/worldgen/structure/nether/' in x]:
     o=json.loads(z.read(n));assert o['project_start_to_heightmap']=='WORLD_SURFACE_WG' and o['start_height']['type']=='minecraft:uniform'
     if platform=='RUST':assert o['horizon:start_checks']['max_y']==h-32
     else:assert 'horizon:start_checks' not in o
  for n in [x for x in allowed if x in ('data/thalvoryn/worldgen/structure/nether/cinder_viaduct.json','data/thalvoryn/worldgen/structure/nether/skybridge_ruin.json')]:
     assert json.loads(z.read(n))['terrain_adaptation']=='beard_thin'
  spawnmax={
    'data/nova_structures/worldgen/structure/stray_fort.json': {'minecraft:stray':4,'minecraft:zombie':4},
    'data/nova_structures/worldgen/structure/toxic_lair.json':{'minecraft:cave_spider':3,'minecraft:zombie':4},
    'data/nova_structures/worldgen/structure/sealing_halls.json':{'minecraft:wither_skeleton':3,'minecraft:skeleton':3},
    'data/nova_structures/worldgen/structure/end_castle.json':{'minecraft:phantom':2}
  }
  for n,sp in spawnmax.items():
    j=json.loads(z.read(n));actual={s['type']:s for o in j.get('spawn_overrides',{}).values() for s in o.get('spawns',[])}
    for mob,limit in sp.items():assert actual[mob]['count']['max_inclusive']<=limit,(v,n,mob)
  ssets={n:json.loads(z.read(n)) for n in names if '/worldgen/structure_set/' in n and n.endswith('.json')}
  active=[e['structure'] for s in ssets.values() for e in s.get('structures',[])]
  assert len(active)==len(set(active)) and len(active)==478
  assert not any('ships_' in n and (n.startswith('thalvoryn:end/') or n.startswith('thalvoryn:nether/')) for n in active)
  struct_count=0;adapt_dist_errors=[]
  for n in names:
    if '/worldgen/structure/' not in n or not n.endswith('.json'):continue
    s=json.loads(z.read(n))
    if s.get('type')!='minecraft:jigsaw':continue
    struct_count+=1
    d=s.get('max_distance_from_center',80);d=d.get('horizontal',80) if isinstance(d,dict) else d
    if s.get('terrain_adaptation','none')!='none' and d>116:adapt_dist_errors.append((n,d))
  assert not adapt_dist_errors,adapt_dist_errors[:10]
  # Rust checks must be exclusively on Rust-specific paths; differences between old and new are scoped.
  report['variants'][v]={'status':'PASS_STATIC','archive':new,'sha256':hashlib.sha256((OUT/new).read_bytes()).hexdigest(),
    'total_files':len(names),'nbt_count_unchanged':nbt_count,'json_mcmeta_valid':json_count,
    'changed_existing_files':len(changes),'unchanged_existing_files':untouched,'active_unique_structures':len(active),
    'jigsaw_range_errors':len(adapt_dist_errors),'changed_paths':sorted(changes)}
  print(v, 'PASS_STATIC',len(changes),'changes',len(names),'files', flush=True)
# validate only documented variant differences
with zipfile.ZipFile(OUT/'Thalvoryn-RUST-Nether1024-V69-PRIVATE-TEST.zip') as a,zipfile.ZipFile(OUT/'Thalvoryn-RUST-Nether2032-V69-PRIVATE-TEST.zip') as b:
 assert all(a.getinfo(n).CRC==b.getinfo(n).CRC for n in a.namelist() if n.endswith('.nbt'))
# End Castle no custom wooden boat or dock subpieces; vanilla End City ship left intact.
with zipfile.ZipFile(OUT/'Thalvoryn-RUST-Nether1024-V69-PRIVATE-TEST.zip') as zz:
 for pool in ('end_castle/boat','end_castle/docks','end_castle/docks_fallback'):
   jj=json.loads(zz.read('data/nova_structures/worldgen/template_pool/'+pool+'.json'))
   assert all(e.get('element',{}).get('element_type')=='minecraft:empty_pool_element' for e in jj.get('elements',[]))
 report['end_castle_pirate_subpools_empty']='PASS'
(OUT/'V69_INDEPENDENT_QA.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')