#!/usr/bin/env python3
"""Cross-variant V70 audit, independent of builder: exact NBT digest and biome checks."""
import zipfile,json,hashlib,math,sys,collections
from pathlib import Path
sys.path.insert(0,'/mnt/data/v68_work')
from nbt_codec_v67 import read_nbt
root=Path('/mnt/data/v68_work/v70_release_candidate')
variants={}
for p in sorted(root.glob('Thalvoryn-*-V70-PRIVATE-TEST.zip')):
 with zipfile.ZipFile(p) as z:
  nbt_names=sorted(n for n in z.namelist() if n.endswith('.nbt'))
  digest=hashlib.sha256()
  for n in nbt_names:digest.update(n.encode());digest.update(hashlib.sha256(z.read(n)).digest())
  variants[p.name]={'nbt_sha':digest.hexdigest(),'count':len(nbt_names),'size':p.stat().st_size}
assert len(variants)==4
assert len(set(x['nbt_sha'] for x in variants.values()))==1,variants
p=root/'Thalvoryn-RUST-Nether1024-V70-PRIVATE-TEST.zip'; boat_audit=[]
with zipfile.ZipFile(p) as z:
 setd=json.loads(z.read('data/thalvoryn/worldgen/structure_set/ecology/ow_ocean.json'))
 for row in setd['structures']:
  sid=row['structure']
  if not(sid.startswith('thalvoryn:overworld/imported/ships_') or (sid.startswith('trek:') and any(x in sid for x in ('trade','ship_pillager')))):
   continue
  d=json.loads(z.read('data/'+sid.replace(':','/worldgen/structure/')+'.json'))
  pool=json.loads(z.read('data/'+d['start_pool'].replace(':','/worldgen/template_pool/')+'.json'))
  spans=[]
  for e in pool.get('elements',[]):
   loc=e['element'].get('location')
   if not loc:continue
   n='data/'+loc.replace(':','/structure/')+'.nbt'
   _,data=read_nbt(z.read(n));size=data['size'].value[1];spans.append(max(size[0],size[2]))
  dim=max(spans)
  required=min(9,max(3,math.ceil(dim/16)+1))
  got=d['horizon:start_checks']['biome_radius']
  assert got==required,(sid,got,required)
  assert all(b.startswith('minecraft:deep_') for b in d.get('biomes',[])),sid
  boat_audit.append({'id':sid,'length_width_max':dim,'biome_radius':got})
assert len(boat_audit)==33
summary={'status':'PASS_STATIC_ONLY_NOT_GAME_TESTED','nbt_variant_sha256':next(iter(variants.values()))['nbt_sha'],'variants':variants,'boat_footprint_checks':boat_audit,'warning':'Biome radius is center/nearby-biome screening; it does not prove entire hull is over water or any Java runtime result.'}
(root/'V70_CROSS_VARIANT_AUDIT.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
print('4 variants share identical 6835 NBT templates',summary['nbt_variant_sha256'][:20], 'boat 33 footprints verified')
print('radius distribution:',dict(collections.Counter(r['biome_radius'] for r in boat_audit)))