#!/usr/bin/env python3
"""Extra independent static QA: loot-table dependency graph and active structure terrain-placement review.
Cannot evaluate in-game procedural worldgen, TPS, collisions, chests upon opening, or cave connectivity.
"""
from pathlib import Path
import zipfile,json,collections,re,hashlib
ROOT=Path(__file__).resolve().parent
OUT=ROOT/'V67_QA_FIX_PRIVATE_TEST'
ZIP=OUT/'Thalvoryn-RUST-Nether1024-V67-QA-FIX-TEST-UNRELEASED.zip'
with zipfile.ZipFile(ZIP) as z:
 names=set(z.namelist())
 j=lambda path:json.loads(z.read(path))
 sets={n.split('/')[1]+':'+n.split('/worldgen/structure_set/')[1][:-5]:j(n) for n in names if re.fullmatch(r'data/[^/]+/worldgen/structure_set/.*\.json',n)}
 structs={n.split('/')[1]+':'+n.split('/worldgen/structure/')[1][:-5]:j(n) for n in names if re.fullmatch(r'data/[^/]+/worldgen/structure/.*\.json',n)}
 loot={n.split('/')[1]+':'+n.split('/loot_table/')[1][:-5]:j(n) for n in names if re.fullmatch(r'data/[^/]+/loot_table/.*\.json',n)}
 active=collections.defaultdict(list)
 for setid,sd in sets.items():
  for item in sd.get('structures',[]):
   sid=item['structure']
   if not sid.startswith('minecraft:'):active[sid].append(setid)
 assert active and all(len(owners)==1 for owners in active.values())
 dep=collections.defaultdict(set); missing=[]
 def entries(e,lid):
  if e.get('type') in ('minecraft:loot_table','loot_table'):
   tar=e.get('value',e.get('name',''))
   if isinstance(tar,str) and tar and not tar.startswith('#') and not tar.startswith('minecraft:'):
    dep[lid].add(tar)
    if tar not in loot:missing.append((lid,tar))
  for x in e.get('children',[]):entries(x,lid)
 for lid,v in loot.items():
  for pool in v.get('pools',[]):
   for e in pool.get('entries',[]):entries(e,lid)
 assert not missing,missing[:5]
 visiting=set();visited=set();cycles=[]
 def walk(k,stack):
  if k in visiting: cycles.append(stack+[k]);return
  if k in visited:return
  visiting.add(k)
  for t in dep.get(k,()):walk(t,stack+[k])
  visiting.remove(k);visited.add(k)
 for lid in loot:walk(lid,[])
 assert not cycles,cycles[:5]
 no_ground=[];cnt=collections.Counter()
 for sid,(setid,) in active.items():
  d=structs[sid]
  if d.get('type')=='minecraft:jigsaw' and not d.get('project_start_to_heightmap') and d.get('terrain_adaptation','none')=='none':
   cnt[setid.split('/')[-1]]+=1
   no_ground.append({'id':sid,'group':setid,'height':d.get('start_height')})
 # Strictly verify 26.3 documented engine built-ins, otherwise package pool must exist.
 vanilla={'minecraft:bastion/starts','minecraft:village/plains/town_centers','minecraft:village/snowy/town_centers','minecraft:trial_chambers/chamber/end'}
 unresolved=[]
 for sid in active:
  d=structs[sid]
  if d.get('type')!='minecraft:jigsaw':continue
  ref=d.get('start_pool','')
  if not ref:unresolved.append((sid,'missing start_pool'));continue
  ns,rel=ref.split(':',1) if ':' in ref else ('minecraft',ref)
  if f'data/{ns}/worldgen/template_pool/{rel}.json' not in names and ref not in vanilla:unresolved.append((sid,ref))
 assert not unresolved,unresolved
 inactive_missing={"nova_structures:illager_manor","nova_structures:village_swamp","nova_structures:village_jungle","nova_structures:nether_keep"}
 assert inactive_missing.isdisjoint(active)
 report={
  'status':'STATIC_ONLY; GAMEPLAY, CHEST OPENING, LAVA/SLOPE, PORTALS, CAVE CONNECTION, AND TERRAIN BLENDING NOT VERIFIED',
  'active_nonvanilla_definitions':len(active),
  'structure_sets_total':len(sets),
  'loots_total':len(loot),
  'loot_internal_refs':sum(len(x) for x in dep.values()),
  'missing_custom_loot_refs':missing,
  'loot_dependency_cycles':cycles,
  'active_start_pools_unresolved':unresolved,
  'disabled_known_broken_external_starts':sorted(inactive_missing),
  'no_projection_or_adaptation_jigsaws':len(no_ground),
  'no_projection_or_adaptation_by_group':dict(cnt),
  'interpretation':'Many of these are intentionally free-floating sky/end islands, ocean ships at sea height, or underground dungeons. A worldgen game test, not just static JSON, is required for proof of terrain blending.','some_no_projection_examples':no_ground[:20],
  'known_limitations':["6 descending tunnels lead to own authored cavities, not guaranteed naturally generated caves", "69 chests with explicit Items [] retained as original empty inventories (not auto-filled)", "64 jigsaw placements lack both heightmap projection and adaptation; most are intentional but their environment fit is unproven", "End pirate ship subpool entries removed; End Castle/NBT runtime placement not tested", "No original JJThunder terrain restoration in this QA pass; Overworld density resources unchanged from ECOLOGY"],
 }
 (OUT/'V67_FINAL_LOOT_PLACEMENT_REVIEW.json').write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
 print('PASS',report['active_nonvanilla_definitions'],'active structures,',report['loots_total'],'loot tables;',sum(len(x) for x in dep.values()),'loot refs, 0 cycles, 0 broken start pools; placement risk count',len(no_ground))