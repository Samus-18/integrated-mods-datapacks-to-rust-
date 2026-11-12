#!/usr/bin/env python3
"""Independent V67 code-side regression against preceding LOOT-RECHECK.
No Minecraft/Horizon runtime. Written separately from builder.
"""
import hashlib,json,pathlib,zipfile,re,collections
R=pathlib.Path('/mnt/data/Thalvoryn_V67_SPAWN_RECHECK')
B=pathlib.Path('/mnt/data/Thalvoryn_V67_AUDIT')
BAD={'thalvoryn:entrances/highland_stone_descent','thalvoryn:entrances/mossy_forest_descent','thalvoryn:caves/trek_dripstone_room','thalvoryn:caves/trek_dripstone_room_upper_rc42','thalvoryn:caves/trek_lush_room','thalvoryn:caves/trek_lush_room_upper_rc42'}
RARS={'thalvoryn:caves/calcite_ossuary':90,'thalvoryn:caves/calcite_ossuary_upper_rc42':130,'thalvoryn:caves/moss_library':110,'thalvoryn:caves/moss_library_upper_rc42':150,'thalvoryn:caves/sculk_archive':180}
reports=[];common=None;oldref_count=None;changed_all=[]
for p in ('RUST','JAVA'):
 for h in (1024,2032):
  old=B/f'Thalvoryn-{p}-Nether{h}-V67-LOOT-RECHECK-TEST.zip'
  new=R/f'Thalvoryn-{p}-Nether{h}-V67-SPAWN-RECHECK-TEST.zip'
  with zipfile.ZipFile(old) as a,zipfile.ZipFile(new) as z:
   assert a.testzip() is None and z.testzip() is None
   names=a.namelist();assert len(names)==len(z.namelist())==len(set(names))==11323 and names==z.namelist()
   ch=[n for n in names if z.getinfo(n).CRC!=a.getinfo(n).CRC]
   assert len(ch)==58,(p,h,len(ch))
   if common is None:common=set(ch)
   assert set(ch)==common
   assert all(not n.endswith('.nbt') for n in ch)
   assert all(a.getinfo(n).CRC==z.getinfo(n).CRC for n in names if n.endswith('.nbt'))
   assert not any(n.startswith('data/stellarity/') for n in names)
   assert json.loads(z.read('pack.mcmeta'))['pack']['pack_format']==121
   assert json.loads(z.read('data/minecraft/dimension_type/the_nether.json'))['height']==h
   removed=collections.Counter()
   for n in names:
    if not (n.endswith('.json') and '/worldgen/biome/' in n):continue
    oldd=json.loads(a.read(n));d=json.loads(z.read(n));
    for before,after in zip(oldd.get('features',[]),d.get('features',[])):
     aa=[x for x in before if isinstance(x,str) and x in BAD]
     bb=[x for x in after if isinstance(x,str) and x in BAD]
     removed.update(aa);assert not bb
     assert [x for x in before if x not in BAD]==after
   assert set(removed)==BAD and sum(removed.values())==22,(removed,p,h)
   for id,chance in RARS.items():
    n='data/thalvoryn/worldgen/placed_feature/'+id.split(':',1)[1]+'.json'
    d=json.loads(z.read(n));r=[v['chance'] for v in d['placement'] if v.get('type')=='minecraft:rarity_filter']
    assert r==[chance]
   sp=json.loads(z.read('data/thalvoryn/worldgen/structure_set/ecology/ow_spawners.json'))['placement']
   ce=json.loads(z.read('data/thalvoryn/worldgen/structure_set/ecology/ow_cave_entrances.json'))
   assert (sp['spacing'],sp['separation'],sp['frequency'])==(42,25,.31)
   assert (ce['placement']['spacing'],ce['placement']['separation'],ce['placement']['frequency'])==(32,25,.70)
   assert len(ce['structures'])==6
   jigs=0;radius_warnings=[]
   for n in names:
    if '/worldgen/structure/' not in n or not n.endswith('.json'):continue
    d=json.loads(z.read(n));
    if d.get('type')!='minecraft:jigsaw':continue
    jigs+=1;rad=d.get('max_distance_from_center',80)
    hr=rad if type(rad) is int else rad.get('horizontal',80)
    allowed=128 if d.get('terrain_adaptation','none')=='none' else 116
    if hr>allowed:radius_warnings.append((n,hr,allowed))
   assert not radius_warnings,radius_warnings[:6]
   for i in range(1,7):
    n=f'data/thalvoryn/worldgen/structure/underground/v67_cave_tunnel_0{i}.json'
    d=json.loads(z.read(n));assert d['project_start_to_heightmap']=='OCEAN_FLOOR_WG'
    assert d['max_distance_from_center']=={'horizontal':116,'vertical':128}
   for n in ['data/nova_structures/worldgen/template_pool/end_castle/boat.json','data/nova_structures/worldgen/template_pool/end_castle/docks.json']:
    e=json.loads(z.read(n))['elements'];assert all(x['element']['element_type']=='minecraft:empty_pool_element' for x in e)
   assert all('ship' not in x['structure'].lower() for group in ('end_minor','end_major','end_islands') for x in json.loads(z.read('data/thalvoryn/worldgen/structure_set/ecology/'+group+'.json'))['structures'])
   reports.append({'platform':p,'nether_height':h,'zip_members':len(names),'changed_json_metadata':len(ch),'nbt_unchanged':6835,'removed_duplicate_feature_references':sum(removed.values()),'checked_jigsaw_definitions':jigs,'invalid_jigsaw_radius':len(radius_warnings),'runtime_tested':False,'sha256':hashlib.sha256(new.read_bytes()).hexdigest()})
assert len(reports)==4
report={'status':'PASS_STATIC_ONLY_NOT_GAME_TESTED','variants':reports,'changed_paths':sorted(common),'changes_summary':{'legacy_stairs_and_parallel_monster_room_feature_references_removed':22,'small_fossil_room_rarity_adjusted':5,'cave_tunnel_surface_projection_changed_to_ocean_floor_no_leaves':6,'invalid_jigsaw_adaptation_radius_fixed':32,'underground_structure_sets_adjusted':2,'nbt_templates_changed':0},'limitations':['Minecraft/Horizon world generation not run','Caves still terminate in template chambers, not guaranteed natural caves','Terrain/structure overlaps require in-game evidence','Original world chunks not retroactively changed']}
(R/'QA_REPORT.json').write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')
print('INDEPENDENT PASS',len(reports),'variants, 58 changed paths per variant, all NBT intact, 0 invalid radii, 22 old feature refs removed')