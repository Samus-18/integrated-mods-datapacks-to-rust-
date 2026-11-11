#!/usr/bin/env python3
"""Independent static verification of V67 QA Fix. Not a runtime game test."""
import json,pathlib,zipfile,collections,hashlib,re,sys,copy
from collections import Counter,defaultdict
ROOT=pathlib.Path(__file__).resolve().parent
SRC=ROOT/'V67_QA_FIX_PRIVATE_TEST'
base_audit=json.loads((ROOT/'V67_FAST_NBT_AUDIT.json').read_text())
req_alias=set(__import__('build_v67_qa_fix').ALIAS)
report={'status':'STATIC_QA_ONLY_NOT_GAMEPLAY_VALIDATED','variants':{},'cross_variant':{},'errors':[],'warnings':[]}
REGIONS={'overworld':'data/minecraft/dimension/overworld.json','the_nether':'data/minecraft/dimension/the_nether.json','the_end':'data/minecraft/dimension/the_end.json'}
PREV={}
NEW={}
for platform in ('JAVA','RUST'):
 for h in (1024,2032):
  p=next(SRC.glob(f'Thalvoryn-{platform}-Nether{h}-V67-QA-FIX-TEST-UNRELEASED.zip'))
  orig=next(ROOT.glob(f'Thalvoryn-{platform}-Nether{h}-V67-ECOLOGY-TEST-UNRELEASED.zip'))
  with zipfile.ZipFile(p) as z,zipfile.ZipFile(orig) as old:
   members=z.namelist();named=set(members)
   assert len(members)==len(named),f'duplicate entries {p}'
   assert z.testzip() is None
   alljson={n:json.loads(z.read(n)) for n in members if n.endswith(('.json','.mcmeta'))}
   structures={n.split('/')[1]+':'+n.split('/worldgen/structure/')[1][:-5]:v for n,v in alljson.items() if re.match(r'data/[^/]+/worldgen/structure/.*\.json$',n)}
   sets={n.split('/')[1]+':'+n.split('/worldgen/structure_set/')[1][:-5]:v for n,v in alljson.items() if re.match(r'data/[^/]+/worldgen/structure_set/.*\.json$',n)}
   pools={n.split('/')[1]+':'+n.split('/worldgen/template_pool/')[1][:-5]:v for n,v in alljson.items() if re.match(r'data/[^/]+/worldgen/template_pool/.*\.json$',n)}
   loots={n.split('/')[1]+':'+n.split('/loot_table/')[1][:-5]:v for n,v in alljson.items() if re.match(r'data/[^/]+/loot_table/.*\.json$',n)}
   assert req_alias<=set(loots),req_alias-set(loots)
   # 26.3 schema checks on pool, entry, and modifier contexts; minecraft:sequence retains functions by design.
   oldfields=[];loot_deps=defaultdict(set)
   def scan_entry(e,src):
    if not isinstance(e,dict):return
    for key in ('functions','conditions'):
     if key in e:oldfields.append((src,'entry',key))
    for mod in (e.get('modifier',[]) if isinstance(e.get('modifier'),list) else [e.get('modifier')] if e.get('modifier') else []):
     if isinstance(mod,dict) and ('function' in mod or 'conditions' in mod):oldfields.append((src,'modifier',mod))
    if e.get('type') in ('minecraft:loot_table','loot_table'):
     target=e.get('value')
     if isinstance(target,str) and not target.startswith('#'):
      if not target.startswith('minecraft:') and target not in loots:report['errors'].append(['DANGLING_LOOT_REF',src,target])
      if target in loots:loot_deps[src].add(target)
    for c in e.get('children',[]):scan_entry(c,src)
   for lid,v in loots.items():
    for pool in v.get('pools',[]):
     if 'functions' in pool or 'conditions' in pool:oldfields.append((lid,'pool'))
     for e in pool.get('entries',[]):scan_entry(e,lid)
   assert not oldfields,oldfields[:10]
   for req in base_audit['missing_custom_loot']:
    if req[2] not in loots:report['errors'].append(['NBT_LOOT_REFERENCE_STILL_MISSING',req])
   # Every NBT remains intact (only JSON changes). Explicitly empty containers intentionally unchanged.
   nbt=[n for n in members if n.endswith('.nbt')]
   assert all(n in old.namelist() and z.getinfo(n).CRC==old.getinfo(n).CRC for n in nbt)
   # Set ownership and internal start pool references.
   ownership=Counter()
   for sid,v in sets.items():
    if 'random_spread' in v.get('placement',{}).get('type',''):
     pp=v['placement']; assert pp['spacing']>pp['separation']>=0
     assert 1<=pp.get('exclusion_zone',{}).get('chunk_count',1)<=16
    for e in v.get('structures',[]):
     st=e.get('structure','')
     assert st in structures or st.startswith('minecraft:'),(sid,st)
     if not st.startswith('minecraft:'):ownership[st]+=1
   assert ownership and all(v==1 for v in ownership.values()),ownership.most_common()[:15]
   for sid,dd in structures.items():
    if dd.get('type')=='minecraft:jigsaw':
     pool=dd.get('start_pool','')
     assert pool.startswith('minecraft:') or pool in pools,(sid,pool)
   # Imported Minecraft-namespace pools must exist either in this ZIP or in known vanilla 26.3.
   VANILLA_START_POOLS={
    'minecraft:bastion/starts','minecraft:village/plains/town_centers',
    'minecraft:village/snowy/town_centers','minecraft:trial_chambers/chamber/end'
   }
   for sid,dd in structures.items():
    if sid not in ownership or dd.get('type')!='minecraft:jigsaw':continue
    pool=dd.get('start_pool','')
    if pool.startswith('minecraft:'):
     ns,path=pool.split(':',1)
     assert (f'data/{ns}/worldgen/template_pool/{path}.json' in named or pool in VANILLA_START_POOLS),(sid,pool)
   assert not (__import__('build_v67_qa_fix').UNRESOLVED_POOL_STARTS & set(ownership))
   for pid,v in pools.items():
    assert len(v.get('elements',[]))>=1,(pid,'empty_pool')
    for e in v['elements']:
     el=e.get('element',{});loc=el.get('location')
     if el.get('element_type') in ('minecraft:single_pool_element','minecraft:legacy_single_pool_element') and isinstance(loc,str):
      ns,rest=(loc.split(':',1) if ':' in loc else ('minecraft',loc))
      if not loc.startswith('minecraft:') and f'data/{ns}/structure/{rest}.nbt' not in named:report['errors'].append(['MISSING_POOL_NBT',pid,loc])
   # End Castle may no longer select pirate ships via its affected pools.
   for pid,v in pools.items():
    if pid.startswith(('nova_structures:end_castle/','nova_structures:end_lighthouse/','stellarity:shipwrecks/','explorify:end_shipwreck/','nova_structures:end_ships/')):
     for e in v['elements']:
      loc=e.get('element',{}).get('location','')
      if re.search(r'(ship|galleon|brigantine|pirat|frigate|vessel|boat|dock)',loc,re.I):report['errors'].append(['END_MARITIME_POOL_REMAINS',pid,loc])
   assert any('minecraft:end_city'==x.get('structure') for x in sets['minecraft:end_cities']['structures'])
   # Concrete biome list entry references cannot point to absent biome definitions in target preset.
   bios=set()
   for dim,fn in REGIONS.items():bios.update(e['biome'] for e in alljson[fn]['generator']['biome_source'].get('biomes',[]))
   for sid,v in structures.items():
    if sid not in ownership:continue
    b=v.get('biomes',[])
    if isinstance(b,str):b=[b]
    for biome in b:
     if biome.startswith('#'):continue  # vanilla tag registry + local tag JSON, do not guess absent.
     if biome not in bios:report['errors'].append(['BIOME_NOT_IN_ACTIVE_SOURCES',sid,biome])
   end_dim=alljson['data/minecraft/dimension_type/the_end.json']
   end_noise=alljson['data/minecraft/worldgen/noise_settings/end.json']['noise']
   net_dim=alljson['data/minecraft/dimension_type/the_nether.json']
   assert end_dim['height']==end_noise['height']==512
   assert net_dim['height']==h
   assert alljson['pack.mcmeta']['pack']['pack_format']==121
   # Guard against accidental changes to original Overworld geometry.
   protected=[n for n in members if '/worldgen/density_function/' in n and ('overworld' in n or n.startswith('data/thunderfusion/'))]
   assert all(z.getinfo(n).CRC==old.getinfo(n).CRC for n in protected)
   def statkey(x):return (len(x),hashlib.sha256(x).hexdigest())
   info={'zip':p.name,'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'files':len(members),'json_meta':len(alljson),'nbt':len(nbt),'structures':len(structures),'owned_nonvanilla':len(ownership),'sets':len(sets),'active_biomes':{dim:len({a['biome'] for a in alljson[fn]['generator']['biome_source'].get('biomes',[])}) for dim,fn in REGIONS.items()},'changes_against_ecology':len(set(members)^set(old.namelist()))+sum(z.getinfo(n).CRC!=old.getinfo(n).CRC for n in named & set(old.namelist())),'nbt_bit_identical_to_ecology':True,'legacy_loot_schema_errors':len(oldfields)}
   report['variants'][f'{platform}_{h}']=info
   NEW[(platform,h)]=set(members)
   print('PASS',platform,h,info['files'],'members',info['nbt'],'NBT',flush=True)
assert not report['errors'],report['errors'][:30]
assert len({tuple(sorted(names)) for names in NEW.values()})==1
report['cross_variant']['all_same_member_paths']=True
report['cross_variant']['same_nbt_inventory']=base_audit['scanned_templates']
out=SRC/'V67_QA_FIX_INDEPENDENT_AUDIT.json';out.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')
print('RESULT',out,'OK')