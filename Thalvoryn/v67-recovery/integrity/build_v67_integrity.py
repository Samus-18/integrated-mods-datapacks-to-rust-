#!/usr/bin/env python3
"""Thalvoryn V67 -- deterministic source-preserving static integrity repair.

Private game-test build. Does NOT claim that arbitrary terrain blends are proven.
Input: V67 isolated no-Stellarity zips. Changes only JSON jigsaw resources,
small structure placement config, and adds missing linked template pool files.
"""
from pathlib import Path
import zipfile,json,copy,hashlib,re,collections,time
HERE=Path(__file__).resolve().parent
OUT=HERE/'V67_INTEGRITY_TEST'
OUT.mkdir(exist_ok=True)
ENC=lambda o:(json.dumps(o,ensure_ascii=False,indent=2,sort_keys=True)+'\n').encode('utf-8')

# Correct nearby source is preferred; explicitly omit optional decorative connectors if
# no matching prefab exists rather than invent a mismatched tower/ship.
POOL_REPAIRS={
 'nova_structures:hamlet/jockey':('nova_structures:piglin_camp/jockey','nether jockey'),
 'nova_structures:nether_skeleton_camp/fake_caves':('nova_structures:nether_skeleton_camp/fake_caves_fort','neutral fortress cave'),
 'nova_structures:nether_skeleton_camp/fake_caves_only_cave':('nova_structures:nether_skeleton_camp/fake_caves_only_cave_fort','neutral fortress cave'),
 'nova_structures:nether_skeleton_camp/small_defence':('nova_structures:nether_skeleton_camp/small_defence_fort','neutral fortress defense'),
 'nova_structures:nether_skeleton_camp/tower_solo':('nova_structures:nether_skeleton_camp/tower_solo_fort','neutral fortress tower'),
 'nova_structures:pale_residence/decor_inside':('nova_structures:pale_residence/pale_residence_shelf_deco_room','pale interior decoration'),
 'nova_structures:piglin_camp/ore':(['nova_structures:piglin_camp/ore_gold','nova_structures:piglin_camp/ore_quartz'],'ore variety'),
 'nova_structures:shrine/combat/room_content':(None,'optional dungeon room decoration, no compatible junction pieces'),
 'nova_structures:stray_fort_walls':(None,'incompatible walls missing; retain jigsaw final stone block'),
 'nova_structures:witch_villa/bookshelf':(None,'optional bookshelf, retain existing final bookshelf block'),
}

def pool_path(id):
 ns,part=id.split(':',1)
 return f'data/{ns}/worldgen/template_pool/{part}.json'

def get_ref(n):return n.split('/')[1]+':'+n.split('/worldgen/template_pool/')[1][:-5]

def get_json(z,path):return json.loads(z.read(path))

def find_cycles(fallback):
 color={}; stack=[];cycles=[]
 def visit(a):
  color[a]=1;stack.append(a)
  b=fallback.get(a)
  if b in fallback:
   if color.get(b)==1:
    cycles.append(stack[stack.index(b):].copy())
   elif not color.get(b):visit(b)
  stack.pop();color[a]=2
 for a in sorted(fallback):
  if not color.get(a):visit(a)
 return cycles

def patch(z,platform,height):
 names=set(z.namelist())
 original_pool={get_ref(n):get_json(z,n) for n in names if '/worldgen/template_pool/' in n and n.endswith('.json')}
 changes={};changes_descr=[]
 fallback={k:v.get('fallback') for k,v in original_pool.items()}
 cycles=find_cycles(fallback)
 repaired_cycles=set()
 # Remove an edge from every cyclic SCC (only one necessary per cycle).
 for cycle in cycles:
  pool=sorted(cycle)[0]
  v=copy.deepcopy(original_pool[pool]);v['fallback']='minecraft:empty'
  changes[pool_path(pool)]=ENC(v)
  repaired_cycles.add(pool)
 for id,(source,reason) in POOL_REPAIRS.items():
  path=pool_path(id)
  if path in names:continue
  if source is None:
   elements=[{'weight':1,'element':{'element_type':'minecraft:empty_pool_element'}}]
  else:
   source_ids=[source] if isinstance(source,str) else source
   elements=[]
   for q in source_ids:
    assert q in original_pool,(id,q)
    base=original_pool[q]
    elements.extend(copy.deepcopy(base.get('elements',[])))
   assert elements,id
  changes[path]=ENC({'name':id,'fallback':'minecraft:empty','elements':elements})
  changes_descr.append({'id':id,'source':source,'description':reason,'elements':len(elements)})
 # Update visible description so testers do not mistake this for an earlier build.
 meta=get_json(z,'pack.mcmeta')
 meta['pack']['description']=f'Thalvoryn V67 {platform} Nether{height} integrity repair — PRIVATE TEST, no Stellarity'
 changes['pack.mcmeta']=ENC(meta)
 # Fix known too-dense End City group and locate grounded large End castle.
 target='data/minecraft/worldgen/structure_set/end_cities.json'
 if target in names:
  d=get_json(z,target)
  assert any(a.get('structure')=='minecraft:end_city' for a in d['structures'])
  d['placement']['spacing']=60
  d['placement']['separation']=25
  d['placement']['frequency']=0.52
  changes[target]=ENC(d)
 castle='data/nova_structures/worldgen/structure/end_castle.json'
 if castle in names:
  d=get_json(z,castle)
  assert d['type']=='minecraft:jigsaw'
  d['start_height']={'absolute':0}
  d['project_start_to_heightmap']='WORLD_SURFACE_WG'
  d['terrain_adaptation']='beard_thin'
  d['biomes']=['minecraft:end_highlands']
  if platform=='RUST':d['horizon:start_checks']={'terrain_radius':2,'terrain_range':24,'min_y':40,'max_y':160}
  else:d.pop('horizon:start_checks',None)
  changes[castle]=ENC(d)
 # Spawner rooms: already reduced from spacing=1, make adjacent spawners still rarer.
 room='data/thalvoryn/worldgen/structure_set/ecology/ow_spawners.json'
 if room in names:
  d=get_json(z,room)
  p=d['placement'];p['spacing']=30;p['separation']=12;p['frequency']=0.27
  changes[room]=ENC(d)
 # Avoid accidentally resetting original Overworld worldgen or Nether height.
 assert not any('/density_function/' in name or '/noise_settings/' in name or '/dimension/' in name for name in changes)
 assert not any('stellarity' in n.lower() for n in names|set(changes))
 return changes,{'platform':platform,'height':height,'fallback_cycles_in_source':len(cycles),'fallback_edges_rewired':len(repaired_cycles),'pool_aliases_added':changes_descr,'other_changes':['end_city_spacing_60_frequency_0.52','end_castle_ground_relative_highlands','ow_spawner_spacing_30_frequency_0.27'],'json_changed_or_added':len(changes)}

manifest={'test_build':True,'release_status':'STATIC PASS REQUIRED; NOT RUNTIME-VERIFIED','source':'V67 ISOLATED NO STELLARITY','patches':[]}
for platform in ('JAVA','RUST'):
 for height in (1024,2032):
  source=HERE/f'Thalvoryn-{platform}-Nether{height}-V67-ISOLATED-NO-STELLARITY-TEST.zip'
  if not source.exists():raise FileNotFoundError(source)
  dest=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-INTEGRITY-REPAIR-TEST.zip'
  t=time.time()
  with zipfile.ZipFile(source) as inz:
   assert inz.testzip() is None
   changed,notes=patch(inz,platform,height)
   with zipfile.ZipFile(dest,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6) as out:
    for member in inz.infolist():
     n=member.filename;raw=changed.pop(n,None)
     if raw is None:raw=inz.read(n)
     # preserve timestamps/file mode, write fresh ZIP safely
     out.writestr(member,raw,compress_type=zipfile.ZIP_DEFLATED,compresslevel=6)
    for n,data in sorted(changed.items()):out.writestr(n,data)
  notes.update({'source_zip':source.name,'result_zip':dest.name,'bytes':dest.stat().st_size,'sha256':hashlib.sha256(dest.read_bytes()).hexdigest(),'elapsed_s':round(time.time()-t,2)})
  print('BUILT',platform,height,notes['bytes'],notes['elapsed_s'],flush=True)
  manifest['patches'].append(notes)
(OUT/'BUILD_MANIFEST.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n')
print('ALL FOUR BUILT')