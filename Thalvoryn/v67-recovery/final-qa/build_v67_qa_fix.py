#!/usr/bin/env python3
"""V67 QA-fix test build: repair proven MC 26.3 loot schema errors, broken loot refs,
biome registration/placement, and End castle maritime pieces. Not game tested.
Private testing only: source data includes third-party assets with unverified redistribution rights.
"""
from __future__ import annotations
import zipfile,pathlib,json,copy,re,collections,hashlib,sys,time
from nbt_codec_v67 import t,read_nbt,write_nbt
ROOT=pathlib.Path(__file__).resolve().parent
SRC={ (p,h):next(ROOT.glob(f'Thalvoryn-{p}-Nether{h}-V67-ECOLOGY-TEST-UNRELEASED.zip')) for p in ('JAVA','RUST') for h in (1024,2032) }
OUT=ROOT/'V67_QA_FIX_PRIVATE_TEST';OUT.mkdir(exist_ok=True)
JSON=lambda v:(json.dumps(v,indent=2,sort_keys=True,ensure_ascii=False)+'\n').encode()
AUDIT=json.loads((ROOT/'V67_FAST_NBT_AUDIT.json').read_text())
# These replacements exist in the source pack and keep loot thematic, not random.
ALIAS={
 'nova_structures:pots/pot_piglins':'nova_structures:pots/pot_piglin',
 'nova_structures:chests/illager_hideout_lesser_meat':'nova_structures:chests/illager_hideout_meat',
 'nova_structures:chests/illager_hideout_raw_trash':'nova_structures:chests/illager_hideout_trash',
 'nova_structures:chests/illager_hideout_raw_weaponry':'nova_structures:chests/illager_hideout_weaponry',
 'stellarity:stronghold/brewery_barrel':'stellarity:stronghold/portal_room/brewing',
 'fokastudio:end/stronghold/stronghold_library':'stellarity:stronghold/library',
 'fokastudio:end/stronghold/library':'stellarity:stronghold/library',
 **{f'nova_structures:chests/ruin_loot_{i}':'nova_structures:chests/ruin_loot_master' for i in range(1,8)}
}
# Unique 26.3 biomes in currently selected dimension. Do not change density functions.
FIX_BIOMES={
 'minecraft:small_end_islands':['minecraft:end_midlands','minecraft:end_barrens','stellarity:end_wilds'],
 'minecraft:dappled_forest':['minecraft:forest','minecraft:birch_forest'],
 'minecraft:pale_garden':['minecraft:dark_forest'],
 'minecraft:sulfur_caves':['minecraft:dripstone_caves'],
}
# Strong, biome-tag-grounded placements in the previous ecology grouping were wrong.
MOVES={
 'explorify:black_spiral':'nether_underground',
 'nova_structures:hamlet':'nether_minor',
 'nova_structures:piglin_camp':'nether_minor',
 'nova_structures:piglin_camp_collony':'nether_minor',
 'nova_structures:piglin_donjon':'nether_underground',
 'nova_structures:piglin_outstation':'nether_ruins',
 'stellarity:old_stronghold':'ow_underground_epic',
 # Independent QA detected category errors from imported packs.
 'nova_structures:illager_hideout':'ow_underground_epic',
 'nova_structures:sealing_halls':'nether_underground',
 'nova_structures:nether_port':'nether_major',
 'nova_structures:trident_trial_monument':'ow_ocean',
}
# These imports reference Minecraft-namespace jigsaw pools absent both from this
# pack and from Minecraft 26.3 vanilla data. Keep their files for future repair,
# but do not silently substitute unrelated structures or claim they can spawn.
UNRESOLVED_POOL_STARTS={
 'nova_structures:illager_manor',
 'nova_structures:nether_keep',
 'nova_structures:village_jungle',
 'nova_structures:village_swamp',
}
# Patterns for ship/port extensions inside End Castle. Ordinary End City ships not touched.
MARITIME=re.compile(r'(ship|galleon|brigantine|pirat|frigate|vessel|boat|dock)',re.I)

def number_nonempty_and(v):
 if not isinstance(v,list):return v
 if len(v)==0:return None
 if len(v)==1:return predicate(v[0])
 return {'type':'minecraft:all_of','terms':[predicate(x) for x in v]}

def predicate(v):
 if not isinstance(v,dict):return v
 out={}
 for k,val in v.items():
  if k=='condition' and isinstance(val,str):out['type']=val
  elif k in ('terms','term'):
   out[k]=[predicate(y) for y in val] if isinstance(val,list) else predicate(val)
  else:out[k]=val
 return out

def mod(v):
 if isinstance(v,list):return [mod(e) for e in v]
 if not isinstance(v,dict):return v
 out={}
 for k,val in v.items():
  if k=='function':out['type']=val
  elif k=='conditions':
   pred=number_nonempty_and(val)
   if pred is not None:out['condition']=pred
  elif k=='condition':out['condition']=predicate(val)
  elif k=='functions':out['functions']=[mod(x) for x in val] if isinstance(val,list) else mod(val) # minecraft:sequence stays functions
  else:out[k]=val
 return out

def entry(v):
 if not isinstance(v,dict):return v
 out={}
 et=v.get('type')
 for k,val in v.items():
  if k=='functions':out['modifier']=mod(val)
  elif k=='modifier':out[k]=mod(val)
  elif k=='conditions':
   pred=number_nonempty_and(val)
   if pred is not None:out['condition']=pred
  elif k=='condition':out[k]=predicate(val)
  elif k=='children':out[k]=[entry(e) for e in val]
  elif k=='name' and et in ('minecraft:tag','tag'):out['items']=val
  else:out[k]=val
 return out

def table(v):
 if not isinstance(v,dict):return v
 out=copy.deepcopy(v)
 for pool in out.get('pools',[]):
  if 'functions' in pool:pool['modifier']=mod(pool.pop('functions'))
  if 'modifier' in pool:pool['modifier']=mod(pool['modifier'])
  if 'conditions' in pool:
   pred=number_nonempty_and(pool.pop('conditions'))
   if pred is not None:pool['condition']=pred
  if 'condition' in pool:pool['condition']=predicate(pool['condition'])
  pool['entries']=[entry(e) for e in pool.get('entries',[])]
 if 'functions' in out:out['modifier']=mod(out.pop('functions'))
 if 'modifier' in out:out['modifier']=mod(out['modifier'])
 if 'conditions' in out:
  pred=number_nonempty_and(out.pop('conditions'))
  if pred is not None:out['condition']=pred
 if 'condition' in out:out['condition']=predicate(out['condition'])
 return out

def idpath(kind,resource):
 ns,name=resource.split(':',1)
 return f'data/{ns}/{kind}/{name}.json'

def choose_chest_loot(path):
 if '/hellish_trials/' in path:return 'hellish_trials:chests/nether_trial/nether_trial_normal'
 if '/conduit_ruin/' in path:return 'nova_structures:chests/conduit_ruin/conduit_ruin_small'
 if '/illager_hideout/' in path:return 'nova_structures:chests/illager_hideout_random'
 if '/remnant/' in path:return 'nova_structures:chests/ruin_loot_master'
 if '/stellarity/structure/stronghold/' in path:return 'stellarity:stronghold/storage'
 if '/nova_structures/' in path:return 'nova_structures:chests/ruin_loot_master'
 return 'minecraft:chests/simple_dungeon'

def fill_untagged_chests(raw,path,known_loot):
 rootname,root=read_nbt(raw)
 palette=root.get('palette')
 if not palette or 'blocks' not in root:return raw,0
 names=[x.get('Name',x.get('id',t(8,''))).value for x in palette.value[1]]
 changed=0
 lootid=choose_chest_loot(path)
 if not lootid.startswith('minecraft:') and lootid not in known_loot:raise AssertionError((path,lootid))
 for b in root['blocks'].value[1]:
  if not isinstance(b,dict) or 'state' not in b:continue
  state=b['state'].value
  if not 0<=state<len(names) or names[state]!='minecraft:chest':continue
  bd=b.get('nbt')
  if bd:
   data=bd.value
   if 'LootTable' in data or 'Items' in data:continue # preserve intentionally empty or explicit inventories
  else:
   data={'id':t(8,'minecraft:chest')};b['nbt']=t(10,data)
  data['LootTable']=t(8,lootid);changed+=1
 if not changed:return raw,0
 return write_nbt(rootname,root,compressed=raw[:2]==b'\x1f\x8b'),changed

def patch(z,p,h):
 files=z.namelist();known=set(files)
 updates={}; added={};stats=collections.Counter();moved=[]
 # Repair the imported loot tables old syntax, only when needed.
 for n in files:
  if n.endswith('.json') and '/loot_table/' in n and n.startswith('data/'):
   d=json.loads(z.read(n));dd=table(d)
   if d!=dd:updates[n]=JSON(dd);stats['legacy_loot_tables_updated']+=1
 # Preserve original themed items by copying a confirmed existing table, not placeholders.
 all_loot={q.split('/')[1]+':'+q.split('/loot_table/')[1][:-5] for q in files if '/loot_table/' in q and q.endswith('.json')}
 for missing,available in ALIAS.items():
  fn=idpath('loot_table',missing);src=idpath('loot_table',available)
  assert fn not in known and src in known,(fn,src)
  d=json.loads(updates.get(src,z.read(src)))
  added[fn]=JSON(table(d));all_loot.add(missing);stats['loot_aliases_created']+=1
 # Surface places, End island biome mappings and cave entrance footprint.
 for n in files:
  if not (n.startswith('data/') and '/worldgen/structure/' in n and n.endswith('.json')):continue
  d=json.loads(z.read(n));old=copy.deepcopy(d)
  b=d.get('biomes',[])
  if isinstance(b,str):b=[b]
  if isinstance(b,list):
   repl=[]
   for x in b:
    repl+=FIX_BIOMES.get(x,[x])
   # preserve order and remove duplicate biomes
   repl=list(dict.fromkeys(repl))
   if repl!=b:d['biomes']=repl;stats['structure_biome_repaired']+=1
  sid=n.split('/')[1]+':'+n.split('/worldgen/structure/')[1][:-5]
  if sid.startswith('thalvoryn:underground/v67_cave_tunnel_'):
   d['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':-65},'max_inclusive':{'absolute':-62}}
   d['terrain_adaptation']='beard_thin'
   d['biomes']=[b for b in d['biomes'] if b not in ('minecraft:plains','minecraft:forest')]
   stats['cave_entrance_ground_blend_tuned']+=1
  if re.fullmatch(r'thalvoryn:overworld/imported/ships_\d+',sid) and isinstance(d.get('start_height'),dict) and 'absolute' in d['start_height']:
   if d['start_height']['absolute'] in (65,68):d['start_height']={'absolute':62};stats['ocean_ship_y_tuned']+=1
  if sid in ('nova_structures:hamlet','nova_structures:piglin_camp','nova_structures:piglin_camp_collony','nova_structures:piglin_outstation') and d.get('type')=='minecraft:jigsaw':
   d['project_start_to_heightmap']='WORLD_SURFACE_WG'
   d['start_height']={'absolute':-5}
   d['terrain_adaptation']='beard_thin'
   if p=='RUST':
    d.setdefault('horizon:start_checks',{}).update({'liquid':'top','terrain_radius':1,'terrain_range':36,'min_y':75,'max_y':h-100})
   else:d.pop('horizon:start_checks',None)
   stats['nether_surface_placement_tuned']+=1
  if d!=old:updates[n]=JSON(d)
 # Important: marine jigsaw children inside End castles (root set disabling ship ID isn't sufficient).
 for n in files:
  if not (n.endswith('.json') and '/worldgen/template_pool/' in n):continue
  if not (n.startswith('data/nova_structures/worldgen/template_pool/end_castle/') or n.startswith('data/nova_structures/worldgen/template_pool/end_lighthouse/') or n.startswith('data/stellarity/worldgen/template_pool/shipwrecks/') or n.startswith('data/nova_structures/worldgen/template_pool/end_ships/') or n.startswith('data/explorify/worldgen/template_pool/end_shipwreck/')):continue
  j=json.loads(z.read(n));old=copy.deepcopy(j);entries=j.get('elements',[])
  if '/end_castle/dock' in n or '/end_castle/boat' in n or '/shipwrecks/' in n or '/end_shipwreck/' in n or '/end_ships/' in n:
   j['elements']=[{'weight':1,'element':{'element_type':'minecraft:empty_pool_element'}}]
   j['fallback']='minecraft:empty'
  else:
   j['elements']=[e for e in entries if not MARITIME.search(e.get('element',{}).get('location',''))]
   if not j['elements']:
    j['elements']=[{'weight':1,'element':{'element_type':'minecraft:empty_pool_element'}}]
    j['fallback']='minecraft:empty'
  if j!=old:
   updates[n]=JSON(j);stats['end_maritime_pool_repaired']+=1
   stats['end_maritime_pool_elements_removed']+=len(entries)-len(j['elements'])
 # Give every moved structure exactly one owner set, preserving its weight.
 set_paths={q.rsplit('/',1)[-1][:-5]:q for q in files if q.startswith('data/thalvoryn/worldgen/structure_set/ecology/') and q.endswith('.json')}
 sets={cat:json.loads(z.read(path)) for cat,path in set_paths.items()}
 # Remove four broken third-party entry points from active generation without
 # deleting source definitions, avoiding silent broken or wrong themed results.
 for sid in sorted(UNRESOLVED_POOL_STARTS):
  owners=[cat for cat,s in sets.items() if any(e['structure']==sid for e in s['structures'])]
  assert len(owners)==1,(sid,owners)
  cat=owners[0]
  sets[cat]['structures']=[e for e in sets[cat]['structures'] if e['structure']!=sid]
  stats['missing_start_pool_structures_deactivated']+=1
 for sid,newcat in MOVES.items():
  before=[(cat,e) for cat,s in sets.items() for e in s['structures'] if e['structure']==sid]
  if len(before)!=1:raise AssertionError((sid,before))
  oldcat,e=before[0]
  if oldcat==newcat:continue
  sets[oldcat]['structures']=[item for item in sets[oldcat]['structures'] if item['structure']!=sid]
  sets[newcat]['structures'].append(e)
  stats['dimension_misplacements_repaired']+=1;moved.append({'structure':sid,'from':oldcat,'to':newcat})
 for cat,s in sets.items():
  if not s['structures']:raise AssertionError('empty set '+cat)
  s['structures'].sort(key=lambda x:x['structure'])
  updates[set_paths[cat]]=JSON(s)
 # Correct plain empty CHESTS, not barrels or decorated pots (many intentionally decorative).
 suspect=sorted({path for path,dicts in AUDIT['unfilled_containers_by_template'] if 'minecraft:chest' in dicts})
 for fn in suspect:
  if fn not in known:raise AssertionError('missing chest template: '+fn)
  raw,nfix=fill_untagged_chests(z.read(fn),fn,all_loot)
  if nfix:updates[fn]=raw;stats['empty_chests_tagged']+=nfix;stats['chest_templates_modified']+=1
 # Cross-check references introduced in Jigsaw ecosystem.
 for fn in list(updates)+list(added):
  if fn not in known and fn not in added:raise AssertionError('Unexpected new path '+fn)
  if fn.endswith('.json'):json.loads(updates.get(fn,added.get(fn,b'{}')))
 output=OUT/f'Thalvoryn-{p}-Nether{h}-V67-QA-FIX-TEST-UNRELEASED.zip'
 with zipfile.ZipFile(output,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True) as dst:
  for name in files:
   b=updates.get(name)
   if b is None:b=z.read(name)
   dst.writestr(name,b)
  for fn,b in sorted(added.items()):dst.writestr(fn,b)
 return {'file':output.name,'bytes':output.stat().st_size,'sha256':hashlib.sha256(output.read_bytes()).hexdigest(),'stats':dict(stats),'moved':moved,'modified_files':sorted(updates.keys()),'added_files':sorted(added.keys())}

def main():
 manifest={'status':'PRIVATE STATIC TEST BUILD – GAMEPLAY NOT VERIFIED','inputs':{f'{p}{h}':str(v) for (p,h),v in SRC.items()},'variants':{}}
 for (p,h),src in SRC.items():
  with zipfile.ZipFile(src) as z:
   manifest['variants'][f'{p}_{h}']=patch(z,p,h)
   print('BUILT',p,h,manifest['variants'][f'{p}_{h}']['bytes'],manifest['variants'][f'{p}_{h}']['stats'],flush=True)
 (OUT/'V67_QA_FIX_BUILD_MANIFEST.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n')
 return manifest
if __name__=='__main__':main()