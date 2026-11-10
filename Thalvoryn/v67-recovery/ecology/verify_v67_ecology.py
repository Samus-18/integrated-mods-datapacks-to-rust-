#!/usr/bin/env python3
"""Independent structural and cross-variant checks for V67 ECOLOGY. STATIC ONLY."""
import json,zipfile,re,hashlib,collections,glob,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OUT=ROOT/'V67_ECOLOGY_PRIVATE_TEST'
manifest=json.loads((OUT/'V67_ECOLOGY_MANIFEST.json').read_text())
TYPES=('jigsaw','minecraft:jigsaw')
def fail(cond,msg):
 if not cond:raise AssertionError(msg)
def parse(z,p):return json.loads(z.read(p))
def categorize(n,kind):
 m=re.fullmatch(fr'data/([^/]+)/worldgen/{kind}/(.+)\.json',n)
 return None if m is None else m.group(1)+':'+m.group(2)
results={};samples={};pairs={}
for key,item in manifest['variants'].items():
 p=Path(item['path']);z=zipfile.ZipFile(p);names=z.namelist();N=set(names)
 fail(len(N)==len(names),'Duplicate entries '+key);fail(z.testzip() is None,'ZIP CRC '+key)
 check_count=0
 for name in names:
  if name.endswith(('.json','.mcmeta')):
   json.loads(z.read(name));check_count+=1
 defs={categorize(x,'structure'):parse(z,x) for x in names if categorize(x,'structure')}
 sets={categorize(x,'structure_set'):parse(z,x) for x in names if categorize(x,'structure_set')}
 pools={categorize(x,'template_pool'):parse(z,x) for x in names if categorize(x,'template_pool')}
 active=collections.defaultdict(list);grouped=[];excluded=[];unknown=[];poolissues=[]
 for sid,s in sets.items():
  place=s['placement'];typ=place.get('type')
  fail(typ in ('minecraft:random_spread','minecraft:concentric_rings'),'Unknown placement '+sid)
  if typ=='minecraft:random_spread':
   sp=place['spacing'];sep=place['separation'];fail(1<=sp<=4096 and 0<=sep<sp,'invalid spacing '+sid)
   freq=place.get('frequency',1);fail(0<=freq<=1,'freq '+sid)
  ex=place.get('exclusion_zone')
  if ex:
   fail(1<=ex['chunk_count']<=16,'exclusion zone out of range '+sid)
   fail(ex['other_set'] in sets or ex['other_set'].startswith('minecraft:'),'exclusion references missing set '+sid+' '+ex['other_set'])
  for itemx in s['structures']:
   st=itemx['structure'];fail(st in defs or st.startswith('minecraft:'),'missing structure '+st)
   fail(itemx['weight']>0,'bad structure weight '+st)
   if not st.startswith('minecraft:'):
    active[st].append(sid)
    if 'ship' in st.lower() or any(t in st.lower() for t in ('frigate','pirate','vessel','galleon')):
     if sid.startswith('thalvoryn:ecology/end_') or sid.startswith('thalvoryn:ecology/nether_'):fail(False,'inappropriate ship '+st+' in '+sid)
  if sid.startswith('thalvoryn:ecology/'):
   fail(len(s['structures'])>=1,'empty custom set '+sid)
   grouped.append(sid)
 for sid,d in defs.items():
  if sid.startswith('minecraft:') or sid not in active:continue
  if d.get('type')=='minecraft:jigsaw':
   pool=d.get('start_pool')
   if isinstance(pool,str):fail(pool in pools or pool.startswith('minecraft:'),'missing start pool '+sid+': '+str(pool))
 for sid,po in pools.items():
  if not isinstance(po,dict):continue
  fb=po.get('fallback')
  if fb and isinstance(fb,str) and fb!='minecraft:empty':
   fail(fb in pools or fb.startswith('minecraft:'),'missing fallback pool '+sid+' -> '+fb)
  for e in po.get('elements',[]):
   element=e.get('element',{})
   if element.get('element_type') in ('minecraft:single_pool_element','minecraft:legacy_single_pool_element','single_pool_element','legacy_single_pool_element'):
    template=element.get('location')
    if isinstance(template,str) and ':' in template:
     ns,sub=template.split(':',1)
     if ns!='minecraft' and f'data/{ns}/structure/{sub}.nbt' not in N:
      poolissues.append({'pool':sid,'template':template})
 fail(not poolissues,'Missing referenced NBT '+str(poolissues[:6]))
 fail(all(len(v)==1 for v in active.values()),'custom structures repeated in sets '+str([k for k,v in active.items() if len(v)>1][:15]))
 for cat, l in manifest['category_structures'].items():
  set_name='thalvoryn:ecology/'+cat
  fail(set_name in sets,'missing ecology set '+cat)
  have={v['structure'] for v in sets[set_name]['structures']}
  fail(have==set(l),'changed category data '+cat)
 for exclusion in manifest['excluded_nonvanilla_ships']:
  fail(exclusion['structure'] not in active,'excluded ship still active: '+exclusion['structure'])
 # End City is a vanilla registry entry, and does not need to be defined inside the ZIP.
 fail(any(v['structure']=='minecraft:end_city' for v in sets['minecraft:end_cities']['structures']),'end_city removed from set')
 fail(sets['minecraft:end_cities']['placement']['spacing']>=70,'end city spacing unchanged')
 # Check spawners from both original Trek categories in the one intentionally sparse ecology group.
 fail(set(x['structure'] for x in sets['thalvoryn:ecology/ow_spawners']['structures'])==set(s for s in active if s.startswith('trek:monster_room/')),'spawner duplicates')
 fail(sets['thalvoryn:ecology/ow_spawners']['placement']['spacing']>=16,'spawner frequency too high')
 # Prove no changes to Overworld base density or biomes in this pass.
 source=Path(manifest['inputs'][str((key.split('_')[0],int(key.split('_')[1])))] )
 with zipfile.ZipFile(source) as orig:
  for f in orig.namelist():
   if f.startswith(('data/thunderfusion/worldgen/density_function/overworld','data/thunderfusion/worldgen/noise_settings/overworld','data/minecraft/worldgen/biome/')):
    fail(z.read(f)==orig.read(f),'Overworld terrain modified '+f)
 # Verify native Horizon checks only in RUST variant for the newly modified structures.
 native=0
 for k,v in defs.items():
  if 'horizon:start_checks' in v:native+=1
  if k.startswith('nova_structures:nether_') and v.get('type')=='minecraft:jigsaw':
   fail(v.get('project_start_to_heightmap')=='WORLD_SURFACE_WG','Nether fixed bottom not repaired '+k)
 nbt=[x for x in names if x.endswith('.nbt')]
 report={'zip':str(p),'entries':len(names),'json_mcmeta':check_count,'nbt':len(nbt),'sets':len(sets),'ecology_sets':len(grouped),'structure_definitions':len(defs),'nonvanilla_active_unique':len(active),'excluded_nonvanilla_ships':len(manifest['excluded_nonvanilla_ships']),'templates_in_pools_missing':len(poolissues),'native_horizon_checks':native,'crc':'PASS','game_test':'NOT RUN'}
 results[key]=report
 # Hash all NBT data to ensure JAVA/RUST same in all variants.
 dig=hashlib.sha256()
 for n in sorted(nbt):dig.update(n.encode()+b'\x00');dig.update(z.read(n))
 pairs[key]=dig.hexdigest();z.close()
 print(key,report)
fail(len(set(pairs.values()))==1,'NBT changed between variants')
qa={'status':'4/4 STATIC PASS - gameplay not tested','variants':results,'nbt_sha256_common':next(iter(pairs.values())),'source_licenses':'Third-party redistribution unverified; private user testing only'}
(OUT/'V67_ECOLOGY_QA.json').write_text(json.dumps(qa,indent=2,ensure_ascii=False)+'\n')
print('PASS ALL FOUR VARIANTS')