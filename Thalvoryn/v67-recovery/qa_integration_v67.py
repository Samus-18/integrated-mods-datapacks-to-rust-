#!/usr/bin/env python3
"""Static resource graph for large V67 integrated pack; not runtime verification.
Does not assume minecraft namespace references require local overrides.
"""
from pathlib import Path
import zipfile,json,collections,sys,hashlib,os,time
from nbt_codec_v67 import read_nbt
DIR=Path(__file__).resolve().parent
P=next(DIR.glob('*RUST-Nether1024-V67-INTEGRATION-TEST-UNRELEASED.zip'))

def target(kind,id):
 if id.startswith('#'):return 'data/'+(id[1:].split(':')[0] if ':' in id else 'minecraft')+'/tags/worldgen/'+kind+'/'+id[1:].split(':')[-1]+'.json'
 ns,rel=id.split(':',1) if ':' in id else ('minecraft',id)
 return f'data/{ns}/worldgen/{kind}/{rel}.json'

def structure_nbt_id(id):
 ns,rel=id.split(':',1) if ':' in id else ('minecraft',id)
 return f'data/{ns}/structure/{rel}.nbt'

report={'source':P.name,'status':'STATIC ONLY','errors':[], 'warnings':[],'counts':{},'samples':{}}
with zipfile.ZipFile(P) as z:
 names=z.namelist();known=set(names)
 assert z.testzip() is None
 assert len(names)==len(known)
 js={};invalid=[]
 for f in names:
  if f.endswith(('.json','.mcmeta')):
   try:js[f]=json.loads(z.read(f))
   except Exception as ex:invalid.append((f,str(ex)))
 report['counts']['all_entries']=len(names);report['counts']['json_and_mcmeta']=len(js)
 report['errors'].extend({'type':'JSON_SYNTAX','path':k,'detail':v} for k,v in invalid)
 defs={n.split('/')[1]+':'+n.split('/worldgen/structure/')[1][:-5]:n for n in js if '/worldgen/structure/' in n and '/tags/' not in n}
 sets={n:obj for n,obj in js.items() if '/worldgen/structure_set/' in n and '/tags/' not in n}
 pools={n:obj for n,obj in js.items() if '/worldgen/template_pool/' in n and '/tags/' not in n}
 checked={'sets':len(sets),'definitions':len(defs),'pools':len(pools)}
 used=set();missing_defs=[];missing_start_pools=[];missing_templates=[];missing_pool_fallback=[];bad_pools=[]
 for f,v in sets.items():
  for member in v.get('structures',[]):
   id=member.get('structure') if isinstance(member,dict) else member
   used.add(id)
   if id not in defs and not id.startswith('minecraft:'):
    missing_defs.append({'from':f,'id':id})
 for name,fn in defs.items():
  d=js[fn]
  if d.get('type') not in ('minecraft:jigsaw','minecraft:stronghold','minecraft:nether_fossil','minecraft:end_city','minecraft:fortress','minecraft:buried_treasure','minecraft:shipwreck','minecraft:igloo','minecraft:swamp_hut','minecraft:desert_pyramid','minecraft:jungle_temple','minecraft:ruined_portal','minecraft:mansion','minecraft:monument','minecraft:mineshaft','minecraft:bastion_remnant'):
   report['warnings'].append({'type':'UNKNOWN_STRUCTURE_TYPE','structure':name,'codec':d.get('type')})
  if d.get('type')=='minecraft:jigsaw':
   poolid=d.get('start_pool','')
   path=target('template_pool',poolid) if isinstance(poolid,str) else '?'
   if path not in pools and not poolid.startswith('minecraft:'):
    missing_start_pools.append({'structure':name,'pool':poolid})
 for f,v in pools.items():
  fallback=v.get('fallback','minecraft:empty')
  if fallback!='minecraft:empty' and target('template_pool',fallback) not in pools and not fallback.startswith('minecraft:'):
   missing_pool_fallback.append({'pool':f,'fallback':fallback})
  for entry in v.get('elements',[]):
   el=entry.get('element',{});et=el.get('element_type','')
   if et in ('minecraft:single_pool_element','minecraft:legacy_single_pool_element'):
    loc=el.get('location','')
    if loc!='minecraft:empty' and structure_nbt_id(loc) not in known and not loc.startswith('minecraft:'):
     missing_templates.append({'pool':f,'location':loc})
   elif et not in ('minecraft:empty_pool_element','minecraft:list_pool_element','minecraft:feature_pool_element'):
    bad_pools.append({'pool':f,'element_type':et})
 report['counts'].update(checked)
 report['counts'].update({'active_structures':len(used),'unreferenced_structures':len(defs.keys()-used),'template_missing':len(missing_templates),'start_pool_missing':len(missing_start_pools),'missing_set_definitions':len(missing_defs)})
 report['samples'].update({'missing_templates':missing_templates[:60], 'missing_start_pools':missing_start_pools[:60], 'missing_defs':missing_defs[:60], 'unknown_pool_element_type':bad_pools[:30],'fallback_missing':missing_pool_fallback[:30]})
 for typ,ls in [('UNRESOLVED_START_POOL',missing_start_pools),('UNRESOLVED_POOL_TEMPLATE',missing_templates),('UNRESOLVED_SET_DEF',missing_defs),('UNRESOLVED_FALLBACK',missing_pool_fallback)]:
  if ls:report['errors'].append({'type':typ,'count':len(ls),'examples':ls[:12]})
 for dim in ('overworld','the_nether','the_end'):
  d=js['data/minecraft/dimension/'+dim+'.json'];bs=d['generator']['biome_source']
  if bs['type']=='minecraft:multi_noise':
   vals=bs['biomes'];refs=set(a['biome'] for a in vals)
   miss=[ref for ref in refs if not ref.startswith('minecraft:') and target('biome',ref) not in js]
   report['counts']['biomes_'+dim]=len(refs)
   if miss:report['errors'].append({'type':'MISSING_BIOME_'+dim,'biomes':miss})
 dim=js['data/minecraft/dimension_type/the_end.json'];noise=js['data/minecraft/worldgen/noise_settings/end.json']
 report['counts']['end_noise_height']=noise['noise']['height']
 assert noise['noise']['height']==dim['height']
 # Quick sample binary NBT tests: 15 generated templates must decode with correct palettes / all voxel coords.
 gen=[s for s in names if s.endswith('.nbt') and ('v67_sky_isle_' in s or 'v67_cave_tunnel_' in s or 'v67_void_rock_' in s)]
 for fn in gen:
  _,r=read_nbt(z.read(fn));size=r['size'].value[1];blocks=r['blocks'].value[1];palette=r['palette'].value[1]
  assert len(palette)>=7
  coords=set()
  for block in blocks:
   x,y,zx=block['pos'].value[1]
   if (x,y,zx) in coords or not (0<=x<size[0] and 0<=y<size[1] and 0<=zx<size[2]):
    report['errors'].append({'type':'GENERATED_NBT_INVALID_BLOCK','path':fn,'pos':[x,y,zx]});break
   coords.add((x,y,zx))
 report['counts']['generated_nbt']=len(gen)
 # Safety: do not include mod-only class files anywhere in distribution.
 assert not any(n.endswith(('.class','.jar')) for n in names)

(DIR/'v67_integration_static_qa.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'entries':report['counts']['all_entries'],'json':report['counts']['json_and_mcmeta'],'defs':report['counts']['definitions'],'active':report['counts']['active_structures'],'pools':report['counts']['pools'],'nbt_new':report['counts']['generated_nbt'],'err':len(report['errors']),'warning':len(report['warnings']),'samples':report['errors'][:5]},ensure_ascii=False,indent=2))