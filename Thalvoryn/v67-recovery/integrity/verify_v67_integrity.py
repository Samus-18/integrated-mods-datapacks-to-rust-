#!/usr/bin/env python3
"""Independent cross-variant static audit for Thalvoryn V67 isolated repair.
Does not simulate Minecraft Java/Horizon worldgen. Fails loudly on regressions.
"""
from pathlib import Path
import json,zipfile,hashlib,collections,gzip,struct,time
ROOT=Path(__file__).resolve().parent
OUT=ROOT/'V67_INTEGRITY_TEST'
J=lambda z,p:json.loads(z.read(p))

def pool_path(id):
 ns,part=(id.split(':',1) if ':' in id else ('minecraft',id))
 return f'data/{ns}/worldgen/template_pool/{part}.json'

def structure_path(id):
 ns,part=id.split(':',1)
 return f'data/{ns}/worldgen/structure/{part}.json'

def check_one(z,platform,height):
 files=z.namelist();names=set(files);errs=[];warn=[]
 if len(files)!=len(names):errs.append('ZIP_DUPLICATES')
 crc=z.testzip()
 if crc:errs.append('ZIP_CRC:'+crc)
 if any('stellarity' in x.lower() for x in files):errs.append('STELLARITY_PRESENT')
 for f in files:
  if f.endswith(('.json','.mcmeta')):
   try:json.loads(z.read(f))
   except Exception:errs.append('JSON:'+f)
 sets={p:J(z,p) for p in files if '/worldgen/structure_set/' in p and p.endswith('.json')}
 definitions={p:J(z,p) for p in files if '/worldgen/structure/' in p and p.endswith('.json') and '/tags/' not in p}
 pools={p:J(z,p) for p in files if '/worldgen/template_pool/' in p and p.endswith('.json')}
 for p,v in pools.items():
  if v.get('fallback') not in ('minecraft:empty',None) and pool_path(v['fallback']) not in names and not v['fallback'].startswith('minecraft:'):
   errs.append('MISSING_POOL_FALLBACK:'+p)
 fallback={p.split('/')[1]+':'+p.split('/worldgen/template_pool/')[1][:-5]:v.get('fallback') for p,v in pools.items()}
 visited=set();stack=set();cycles=[]
 def walk(id):
  if id in stack:cycles.append(id);return
  if id in visited:return
  visited.add(id);stack.add(id)
  nextref=fallback.get(id)
  if nextref in fallback:walk(nextref)
  stack.remove(id)
 for k in sorted(fallback):walk(k)
 if cycles:errs.append('FALLBACK_CYCLES:'+str(cycles[:5]))
 active=collections.defaultdict(list)
 for name,v in sets.items():
  for member in v.get('structures',[]):
   s=member.get('structure','');active[s].append(name)
   if s.startswith('minecraft:'):continue
   if structure_path(s) not in names:errs.append('MISSING_STRUCTURE:'+s)
 for id,group in active.items():
  if len(group)>1:errs.append('DUPLICATE_ACTIVE_STRUCTURE:'+id)
 # User-specified dimension rule: pirate/custom ships only in the Overworld.
 for p,v in sets.items():
  if '/ecology/end_' in p or '/ecology/nether_' in p:
   for entry in v.get('structures',[]):
    id=entry.get('structure','').lower()
    if any(w in id for w in ('pirate','galleon','frigate','shipwreck','custom_ship','air_vessel')):
     errs.append('WRONG_DIMENSION_SHIP:'+id)
 for id,groups in active.items():
  p=structure_path(id) if id.count(':')==1 else ''
  if p not in definitions:continue
  v=definitions[p]
  if v.get('type')=='minecraft:jigsaw':
   target=v.get('start_pool')
   if target and not target.startswith('minecraft:') and pool_path(target) not in names:errs.append('START_POOL_MISSING:'+id)
 dim=J(z,'data/minecraft/dimension_type/the_nether.json')
 if dim.get('height')!=height or dim.get('logical_height')!=height:errs.append('NETHER_HEIGHT')
 mcmeta=J(z,'pack.mcmeta')
 if mcmeta['pack'].get('pack_format')!=121:errs.append('PACK_FORMAT')
 if any('horizon:start_checks' in v for p,v in definitions.items()) != (platform=='RUST'):
  errs.append('PLATFORM_CHECK_FIELDS')
 for n in ['data/minecraft/dimension/overworld.json','data/minecraft/dimension/the_end.json','data/minecraft/dimension/the_nether.json']:
  if n not in names:errs.append('MISSING_DIM:'+n)
 end_city=J(z,'data/minecraft/worldgen/structure_set/end_cities.json')['placement']
 if (end_city['spacing'],end_city['frequency'])!=(60,.52):errs.append('END_CITY_SPACING')
 end_castle=J(z,'data/nova_structures/worldgen/structure/end_castle.json')
 if not (end_castle.get('project_start_to_heightmap')=='WORLD_SURFACE_WG' and end_castle.get('biomes')==['minecraft:end_highlands']):errs.append('END_CASTLE_NOT_GROUNDED')
 # Preserved base NBT content is cross-variant tested separately.
 return {'platform':platform,'height':height,'entries':len(files),'nbt':len([f for f in files if f.endswith('.nbt')]),'pool_count':len(pools),'structure_defs':len(definitions),'structure_sets':len(sets),'unique_active':len(active),'fallback_cycles':len(cycles),'errors':errs,'warnings':warn}

variants={};filemembers=None;nbt_digest=None;worldgen_overworld_digest=None
for platform in ['JAVA','RUST']:
 for height in [1024,2032]:
  f=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-INTEGRITY-REPAIR-TEST.zip'
  with zipfile.ZipFile(f) as z:
   record=check_one(z,platform,height)
   nbt=hashlib.sha256()
   for p in sorted(n for n in z.namelist() if n.endswith('.nbt')):
    nbt.update(p.encode('utf-8'));nbt.update(z.read(p))
   record['nbt_content_hash']=nbt.hexdigest()
   ow=hashlib.sha256()
   for p in sorted(n for n in z.namelist() if (('/density_function/' in n and 'overworld' in n) or '/worldgen/noise_settings/overworld' in n) and n.endswith('.json')):
    ow.update(p.encode('utf-8'));ow.update(z.read(p))
   record['density_overworld_hash']=ow.hexdigest()
   record['zip_sha256']=hashlib.sha256(f.read_bytes()).hexdigest()
   variants[f'{platform}_{height}']=record
   print(platform,height,'errors',len(record['errors']),'nbt',record['nbt'],'pools',record['pool_count'],flush=True)
nbt_ids={v['nbt_content_hash'] for v in variants.values()}
ow_ids={v['density_overworld_hash'] for v in variants.values()}
assert len(nbt_ids)==1,'NBT changed across variants'
assert len(ow_ids)==1,'Overworld densities differ across variants'
assert all(not v['errors'] for v in variants.values()),'Static QA errors: '+str([v['errors'] for v in variants.values()])
# Check changes vs source: all underlying NBT and noise remained byte-identical.
for key,v in variants.items():
 plat,h=key.split('_');source=ROOT/f'Thalvoryn-{plat}-Nether{h}-V67-ISOLATED-NO-STELLARITY-TEST.zip'
 with zipfile.ZipFile(source) as z:
  old_nbt=hashlib.sha256()
  for p in sorted(n for n in z.namelist() if n.endswith('.nbt')):
   old_nbt.update(p.encode('utf-8'));old_nbt.update(z.read(p))
  assert old_nbt.hexdigest()==v['nbt_content_hash'],'Unexpected NBT rewrite'
  old_ow=hashlib.sha256()
  for p in sorted(n for n in z.namelist() if (('/density_function/' in n and 'overworld' in n) or '/worldgen/noise_settings/overworld' in n) and n.endswith('.json')):
   old_ow.update(p.encode('utf-8'));old_ow.update(z.read(p))
  assert old_ow.hexdigest()==v['density_overworld_hash'],'Overworld density overwritten'
report={'status':'PASS_STATIC_ONLY_NOT_GAME_TESTED','variants':variants,'nbt_identical_4_variants':True,'overworld_density_identical_4_variants_and_sources':True,'validated': ['ZIP CRC and JSON','fallback cycles','missing active root pool and structure refs','variation/height integrity','Stellarity namespace absent','preserved NBT and Overworld density'],'not_verified':['Minecraft 26.3 in-game launch','Horizon 2ebf00a runtime compatibility','real-world jigsaw adjacency/collision','actual loot-open behavior','performance/TPS and portal transitions','procedural caves connected to tunnels']}
(OUT/'QA_REPORT.json').write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')
print('ALL FOUR PASS',flush=True)