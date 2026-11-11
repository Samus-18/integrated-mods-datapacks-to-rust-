#!/usr/bin/env python3
"""Static adversarial audit of current V67 ecology pack. Does not claim runtime placement."""
from __future__ import annotations
import zipfile,pathlib,sys,json,collections,re,hashlib
from nbt_codec_v67 import read_nbt
from collections import Counter,defaultdict
BASE=pathlib.Path(__file__).resolve().parent
p=next(BASE.glob('*RUST*1024*ECOLOGY*'))
report={'source':p.name,'static_only':True,'major_problems':{},'examples':{},'statistics':{}}
with zipfile.ZipFile(p) as z:
 names=set(z.namelist())
 js={n:json.loads(z.read(n)) for n in names if n.endswith(('.json','.mcmeta'))}
 defs={n.split('/')[1]+':'+n.split('/worldgen/structure/')[1][:-5]:v for n,v in js.items() if re.match(r'^data/[^/]+/worldgen/structure/',n)}
 sets={n.split('/')[1]+':'+n.split('/worldgen/structure_set/')[1][:-5]:v for n,v in js.items() if re.match(r'^data/[^/]+/worldgen/structure_set/',n)}
 pools={n.split('/')[1]+':'+n.split('/worldgen/template_pool/')[1][:-5]:v for n,v in js.items() if re.match(r'^data/[^/]+/worldgen/template_pool/',n)}
 loot={n.split('/')[1]+':'+n.split('/loot_table/')[1][:-5]:v for n,v in js.items() if re.match(r'^data/[^/]+/loot_table/',n)}
 tags={n.split('/')[1]+':'+n.split('/tags/worldgen/biome/')[1][:-5]:v for n,v in js.items() if re.match(r'^data/[^/]+/tags/worldgen/biome/',n)}
 dimensions={s:js[f'data/minecraft/dimension/{s}.json'] for s in ['overworld','the_nether','the_end']}
 dim_biomes={k:{b['biome'] for b in v['generator']['biome_source'].get('biomes',[])} for k,v in dimensions.items()}
 # Externally shipped vanilla biome tags, map obvious vanilla #minecraft:is_<dimension> tags.
 external={'minecraft:is_overworld':dim_biomes['overworld'],'minecraft:is_nether':dim_biomes['the_nether'],'minecraft:is_end':dim_biomes['the_end']}
 def tagvalues(s,stack=()):
  if not s.startswith('#'):return {s}
  sid=s[1:]
  if sid in stack:return set()
  if sid not in tags:return external.get(sid,set())
  r=set()
  for v in tags[sid].get('values',[]):
   if isinstance(v,dict):v=v.get('id','')
   if v:r|=tagvalues(v,stack+(sid,))
  return r
 active=defaultdict(list)
 for setid,v in sets.items():
  for m in v.get('structures',[]):active[m['structure']].append(setid)
 no_biomes=[];multidim=[];biome_wrong=[];wrong_name=[];nonactive=[];unknown_tags=[]
 for sid in sorted(active):
  if sid not in defs:continue
  d=defs[sid];bi=d.get('biomes',[])
  if isinstance(bi,str):bi=[bi]
  expanded=set()
  for val in bi:
   if isinstance(val,str):
    res=tagvalues(val)
    if val.startswith('#') and not res:unknown_tags.append((sid,val))
    expanded|=res
  valid={dim:len(expanded & bios) for dim,bios in dim_biomes.items()}
  if not any(valid.values()):no_biomes.append((sid,bi,valid))
  if sum(n>0 for n in valid.values())>=2:multidim.append((sid,valid))
  pattern={'the_end':r'(nether|overworld|ocean|pirate|ships_|sail|frigate|boat|vessel|shipwreck)','the_nether':r'(overworld|ocean|pirate|ships_|sail|frigate|boat|vessel|shipwreck)','overworld':r'(nether|end_city|end_)'}
  for dim,v in valid.items():
   if v and re.search(pattern[dim],sid,re.I):wrong_name.append((sid,dim,valid))
 report['statistics']['dimensions']={k:len(v) for k,v in dim_biomes.items()}
 report['statistics']['defs_sets_pools_loot']=dict(defs=len(defs),sets=len(sets),pools=len(pools),loot=len(loot),tags=len(tags),active=len(active))
 report['major_problems']['active_definitions_missing_valid_biomes']=len(no_biomes)
 report['examples']['missing_biomes']=no_biomes[:90]
 report['major_problems']['active_biome_tags_unresolved']=len(unknown_tags)
 report['examples']['unresolved_biome_tags']=unknown_tags[:120]
 report['major_problems']['active_structures_matching_multiple_dimensions']=len(multidim)
 report['examples']['multi_dim']=multidim[:60]
 report['statistics']['suspect_structure_names_by_dim']=wrong_name[:50]
 # Loot schema key audit with semantics by context
 old_keys=defaultdict(list);loot_table_entries=defaultdict(list);loot_ref_missing=[];bad_empty=[];type_none=[]
 def loot_scan(node,n,ptr='',context=''):
  if isinstance(node,dict):
   for key,value in node.items():
    if key=='functions' and (context in ['pool','entry']):old_keys['old_functions'].append((n,ptr))
    if key=='conditions' and (context in ['pool','entry','modifier']):old_keys['old_conditions'].append((n,ptr))
    if key=='function' and context=='modifier':old_keys['old_function_type'].append((n,ptr))
    if key=='condition' and isinstance(value,str) and context=='predicate':old_keys['old_predicate_type'].append((n,ptr))
    if context=='entry' and node.get('type') in ('minecraft:tag','tag') and key=='name':old_keys['old_tag_name'].append((n,ptr))
   ty=node.get('type','')
   if context=='entry' and ty in ('minecraft:loot_table','loot_table'):
    ref=node.get('value',node.get('name'))
    if isinstance(ref,str) and not ref.startswith('#') and not ref.startswith('minecraft:') and ref not in loot:loot_ref_missing.append((n,ref))
   if context=='entry' and ty in ('minecraft:item','item') and not node.get('name'):bad_empty.append((n,ptr))
   for key,value in node.items():
    if key=='entries' or key=='children':
     if isinstance(value,list):
      for i,e in enumerate(value):loot_scan(e,n,ptr+f'/{key}/{i}','entry')
    elif key=='pools' and isinstance(value,list):
     for i,e in enumerate(value):loot_scan(e,n,ptr+f'/pools/{i}','pool')
    elif key in ('functions','modifier') and isinstance(value,list):
     for i,e in enumerate(value):loot_scan(e,n,ptr+f'/{key}/{i}','modifier')
    elif key in ('conditions','condition') and isinstance(value,list):
     for i,e in enumerate(value):loot_scan(e,n,ptr+f'/{key}/{i}','predicate')
    elif key=='condition' and isinstance(value,dict):loot_scan(value,n,ptr+'/condition','predicate')
    elif key in ('functions','modifier') and isinstance(value,dict):loot_scan(value,n,ptr+'/'+key,'modifier')
    elif isinstance(value,(dict,list)):loot_scan(value,n,ptr+'/'+key,'')
  elif isinstance(node,list):
   for i,e in enumerate(node):loot_scan(e,n,ptr+f'/{i}',context)
 for n,v in loot.items():loot_scan(v,n)
 report['major_problems']['legacy_loot_schema']={k:len(v) for k,v in old_keys.items()}
 report['examples']['legacy_loot_schema']={k:v[:20] for k,v in old_keys.items()}
 report['major_problems']['loot_entry_missing_references']=len(loot_ref_missing)
 report['examples']['loot_missing_refs']=loot_ref_missing[:30]
 report['major_problems']['loot_empty_item_entries']=len(bad_empty)
 report['examples']['loot_empty_items']=bad_empty[:30]

out=BASE/'V67_DEEP_AUDIT_METADATA.json';out.write_text(json.dumps(report,indent=2,ensure_ascii=False,default=lambda x:dict(x) if isinstance(x,Counter) else str(x))+'\n')
print('WROTE',out)
print(json.dumps({'problems':report['major_problems'],'statistics':report['statistics']['defs_sets_pools_loot']},indent=2,ensure_ascii=False))