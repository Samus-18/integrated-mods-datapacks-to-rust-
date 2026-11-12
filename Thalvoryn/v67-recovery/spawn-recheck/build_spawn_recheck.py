#!/usr/bin/env python3
"""Conservative V67 follow-up: remove parallel micro-room/old stair generation.
Only biome feature refs, feature rarity, entry structure_set, six heightmaps,
pack description are modified. User supplied third-party structures remain private.
Does NOT claim in-game compatibility or cave connectivity.
"""
import json, zipfile, collections, hashlib, pathlib
ROOT=pathlib.Path('/mnt/data'); SRC=ROOT/'Thalvoryn_V67_AUDIT'; OUT=ROOT/'Thalvoryn_V67_SPAWN_RECHECK'
REMOVE_FEATURES={
 'thalvoryn:entrances/highland_stone_descent',
 'thalvoryn:entrances/mossy_forest_descent',
 'thalvoryn:caves/trek_dripstone_room',
 'thalvoryn:caves/trek_dripstone_room_upper_rc42',
 'thalvoryn:caves/trek_lush_room',
 'thalvoryn:caves/trek_lush_room_upper_rc42',
}
# Five fossil-template microdecorations should not appear every 6-16 chunks.
RARITIES={
 'thalvoryn:caves/calcite_ossuary':90,
 'thalvoryn:caves/calcite_ossuary_upper_rc42':130,
 'thalvoryn:caves/moss_library':110,
 'thalvoryn:caves/moss_library_upper_rc42':150,
 'thalvoryn:caves/sculk_archive':180,
}
CAVE_SET='data/thalvoryn/worldgen/structure_set/ecology/ow_cave_entrances.json'
SPAWNER_SET='data/thalvoryn/worldgen/structure_set/ecology/ow_spawners.json'
TUNNELS={f'data/thalvoryn/worldgen/structure/underground/v67_cave_tunnel_0{i}.json' for i in range(1,7)}

def j(raw):return json.loads(raw)
def jj(doc):return (json.dumps(doc,sort_keys=True,ensure_ascii=False,indent=2)+'\n').encode()

def build(platform,height):
 src=SRC/f'Thalvoryn-{platform}-Nether{height}-V67-LOOT-RECHECK-TEST.zip'
 dst=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-SPAWN-RECHECK-TEST.zip'
 modified={}; removed=collections.Counter(); touched_biomes={}; previous={}; jigsaw_radius_fixes=[]
 with zipfile.ZipFile(src) as zin:
  orig=set(zin.namelist()); assert len(orig)==len(zin.namelist())==11323
  def put(path,doc):
   if jj(doc)!=zin.read(path):modified[path]=jj(doc)
  for name in zin.namelist():
   if not (name.endswith('.json') and '/worldgen/biome/' in name):continue
   d=j(zin.read(name));features=d.get('features',[])
   if not isinstance(features,list):continue
   old=sum(len(s) if isinstance(s,list) else 0 for s in features)
   for si,step in enumerate(features):
    if not isinstance(step,list):continue
    present=[x for x in step if isinstance(x,str) and x in REMOVE_FEATURES]
    for x in present:removed[x]+=1
    features[si]=[x for x in step if x not in REMOVE_FEATURES]
   new=sum(len(s) if isinstance(s,list) else 0 for s in features)
   if new!=old:put(name,d);touched_biomes[name]=old-new
  assert set(removed)==REMOVE_FEATURES,removed
  assert sum(removed.values())==22,(sum(removed.values()),removed)
  for id,chance in RARITIES.items():
   path='data/thalvoryn/worldgen/placed_feature/'+id.split(':',1)[1]+'.json'
   d=j(zin.read(path));a=[x for x in d.get('placement',[]) if x.get('type')=='minecraft:rarity_filter']
   assert len(a)==1 and a[0].get('chance',1)<chance, (path,a)
   previous[id]=a[0]['chance'];a[0]['chance']=chance;put(path,d)
  d=j(zin.read(CAVE_SET));assert len(d['structures'])==6
  p=d['placement'];assert p['type']=='minecraft:random_spread' and p['spacing']==29
  p.update({'spacing':32,'separation':25,'frequency':.70});put(CAVE_SET,d)
  d=j(zin.read(SPAWNER_SET));p=d['placement'];assert p['spacing']==42 and p['separation']==17
  p['separation']=25;put(SPAWNER_SET,d)
  for path in TUNNELS:
   d=j(zin.read(path));assert d['project_start_to_heightmap']=='WORLD_SURFACE_WG' and d['type']=='minecraft:jigsaw'
   assert d['start_pool'].startswith('thalvoryn:underground/v67_cave_tunnel')
   d['project_start_to_heightmap']='OCEAN_FLOOR_WG';put(path,d)
  # MC 26.3 jigsaw codec: horizontal distance + 12 for any terrain adaptation must be <=128.
  # Without terrain adaptation, the maximum remains 128. The earlier V67
  # wrongly gave many surface-adapted structures 127/128 and would be rejected.
  for path in zin.namelist():
   if not (path.endswith('.json') and '/worldgen/structure/' in path):continue
   d=j(modified.get(path,zin.read(path)))
   if d.get('type')!='minecraft:jigsaw':continue
   adaptation=d.get('terrain_adaptation','none')
   limit=128 if adaptation=='none' else 116
   radius=d.get('max_distance_from_center',80)
   horizontal=radius if isinstance(radius,int) else radius.get('horizontal',80)
   if horizontal>limit:
    if isinstance(radius,int):d['max_distance_from_center']={'horizontal':limit,'vertical':radius}
    else:d['max_distance_from_center']['horizontal']=limit
    put(path,d)
    jigsaw_radius_fixes.append({'path':path,'old':horizontal,'new':limit,'adaptation':adaptation})
  assert len(jigsaw_radius_fixes)==32,len(jigsaw_radius_fixes)
  pack=j(zin.read('pack.mcmeta'));assert pack['pack']['pack_format']==121
  pack['pack']['description']=f'Thalvoryn V67 {platform} Nether{height} SPAWN RECHECK TEST - no Stellarity'
  put('pack.mcmeta',pack)
  assert all(n in orig for n in modified)
  with zipfile.ZipFile(dst,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True) as zout:
   for info in zin.infolist():zout.writestr(info,modified.get(info.filename,zin.read(info.filename)))
 manifest={
 'platform':platform,'height':height,'source':src.name,'result':dst.name,
 'modified_paths':sorted(modified),'jigsaw_radius_fixes':jigsaw_radius_fixes,'removed_legacy_placed_feature_refs':dict(removed),
 'biomes_with_removed_refs':touched_biomes,
 'micro_room_rarity_old':previous,'micro_room_rarity_new':RARITIES,
 'spawner_structure_set_spacing':42,'spawner_structure_set_separation':25,'spawner_structure_set_frequency':.31,
 'cave_tunnel_structure_set_spacing':32,'cave_tunnel_structure_set_separation':25,'cave_tunnel_structure_set_frequency':.70,
 'changed_templates':0,'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),
 'result_sha256':hashlib.sha256(dst.read_bytes()).hexdigest(),
 }
 return manifest
if __name__=='__main__':
 reports=[]
 for p in ('RUST','JAVA'):
  for h in (1024,2032):
   a=build(p,h);reports.append(a);print('BUILT',p,h,'modified',len(a['modified_paths']),'old refs removed',sum(a['removed_legacy_placed_feature_refs'].values()),flush=True)
 (OUT/'BUILD_MANIFEST.json').write_text(json.dumps({'status':'STATIC TEST ONLY; NOT GAME-TESTED','variants':reports},indent=2,ensure_ascii=False)+'\n')