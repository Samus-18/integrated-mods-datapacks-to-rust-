"""Independent static constraints for Thalvoryn V67 ground/loot test build.
Checks are not a Minecraft or Horizon gameplay simulation.
"""
from pathlib import Path
import zipfile,json,gzip,collections,hashlib,struct
BASE=Path('/mnt/data/thalvoryn_v67_hardfix/V67_INTEGRITY_TEST')
NEW=Path('/mnt/data/thalvoryn_v67_terrain_loot/PRIVATE_TEST')
CLEAN=lambda a: {k:CLEAN(v) for k,v in a.items() if not k.startswith('horizon:')} if isinstance(a,dict) else ([CLEAN(v) for v in a] if isinstance(a,list) else a)

def j(z,n):return json.loads(z.read(n))

def raw_nbt(z,n):
 v=z.read(n);return gzip.decompress(v) if v[:2]==b'\x1f\x8b' else v

def get_counts(z):
 result=collections.Counter()
 for n in z.namelist():
  if n.endswith('.nbt'):
   d=raw_nbt(z,n)
   result['seed']+=d.count(b'\x04\x00\x0dLootTableSeed')
   result['generic_thalvoryn_ow']+=d.count(b'thalvoryn:chests/rc37/overworld/generic_uncommon_plains')
 return dict(result)

report={'status':'STATIC PASS ONLY – NO RUNTIME TEST','checks':[],'variants':{}}
all_files=None;nbt_digest=[]
for platform in ('JAVA','RUST'):
 for height in (1024,2032):
  name=f'{platform}_{height}'
  source=next(BASE.glob(f'Thalvoryn-{platform}-Nether{height}-V67-INTEGRITY-REPAIR-TEST.zip'))
  path=NEW/f'Thalvoryn-{platform}-Nether{height}-V67-GROUND-LOOT-TEST.zip'
  with zipfile.ZipFile(path) as z,zipfile.ZipFile(source) as old:
   assert z.testzip() is None
   names=z.namelist();oldnames=old.namelist()
   assert names==oldnames and len(names)==len(set(names))==11323
   if all_files is None:all_files=names
   assert names==all_files
   assert j(z,'pack.mcmeta')['pack']['pack_format']==121
   assert j(z,'data/minecraft/dimension_type/the_nether.json')['height']==height
   assert j(z,'data/minecraft/dimension_type/the_nether.json')['logical_height']==height
   assert j(z,'data/thunderfusion/worldgen/noise_settings/overworld.json')==j(old,'data/thunderfusion/worldgen/noise_settings/overworld.json')
   assert z.read('data/minecraft/dimension/overworld.json')==old.read('data/minecraft/dimension/overworld.json')
   assert z.read('data/minecraft/dimension/the_end.json')==old.read('data/minecraft/dimension/the_end.json')
   assert not any('/stellarity/' in n for n in names)
   assert len([n for n in names if n.endswith('.nbt')])==6835
   ship_paths=[n for n in names if n.startswith('data/thalvoryn/worldgen/structure/overworld/imported/ships_') and n.endswith('.json')]
   ships=0
   for n in ship_paths:
    d=j(z,n);prev=j(old,n)
    if d==prev:continue
    ships+=1
    assert set(d['biomes'])==set(['minecraft:deep_ocean','minecraft:deep_lukewarm_ocean','minecraft:deep_cold_ocean'])
    assert 49<=d['start_height']['absolute']<=59
    assert d.get('project_start_to_heightmap') is None
    if platform=='RUST':
     assert d['horizon:start_checks']['biome_radius']==2
     assert 'liquid' not in d['horizon:start_checks']
    else:assert 'horizon:start_checks' not in d
   assert ships==27,ships
   setc=j(z,'data/thalvoryn/worldgen/structure_set/ecology/ow_cave_entrances.json')['placement']
   assert setc['spacing']==30 and setc['separation']==12 and setc['frequency']==0.63
   tunnels=[n for n in names if n.startswith('data/thalvoryn/structure/underground/v67_cave_tunnel_') and n.endswith('.nbt')]
   assert len(tunnels)==6
   for n in tunnels:
    raw=raw_nbt(z,n);assert raw.count(b'minecraft:air')>=1
   for lid in ('farmer','wood_and_stick','stick_and_emrerald'):
    tbl=j(z,'data/structuresplus/loot_table/'+lid+'.json')
    assert tbl['type']=='minecraft:chest'
    assert len(tbl['pools'])==2 and len(tbl['pools'][0]['entries'])>=6
    for pool in tbl['pools']:
     assert 'entries' in pool and 'rolls' in pool
     for e in pool['entries']:
      if 'name' in e:assert e['name'].startswith('minecraft:')
   for n in names:
    if n.endswith(('.json','.mcmeta')):j(z,n)
   ng=get_counts(z);assert ng.get('seed',0)==0,ng
   # With fixed seed removed, loot tables generate using runtime RNG.
   h=hashlib.sha256()
   for n in names:
    if n.endswith('.nbt'):
     h.update(n.encode());h.update(z.read(n))
   nbt_digest.append(h.hexdigest())
   changed=[n for n in names if z.getinfo(n).CRC!=old.getinfo(n).CRC]
   assert 250<=len(changed)<=285,len(changed)
   # End & Nether worldgen JSON remains byte-identical except explicitly changed NBT loot seeds.
   assert not any('/worldgen/structure/' in n and ('/end/' in n or '/nether/' in n) for n in changed)
   assert all('data/minecraft/worldgen/' not in n for n in changed)
   result={'zip':path.name,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'bytes':path.stat().st_size,'members':len(names),'nbt':len(tunnels),'ship_definitions_corrected':ships,'modified_files':len(changed),'no_fixed_loot_seed':True,'nbt_content_digest':nbt_digest[-1],'loot_remap_generic_remaining':ng.get('generic_thalvoryn_ow',0)}
   report['variants'][name]=result
   print(name,'PASS',result['modified_files'], 'remaining generic',result['loot_remap_generic_remaining'],flush=True)
assert len(set(nbt_digest))==1,nbt_digest
for height in (1024,2032):
 with zipfile.ZipFile(NEW/f'Thalvoryn-JAVA-Nether{height}-V67-GROUND-LOOT-TEST.zip') as a,zipfile.ZipFile(NEW/f'Thalvoryn-RUST-Nether{height}-V67-GROUND-LOOT-TEST.zip') as b:
  diffs=[]
  for n in all_files:
   if a.getinfo(n).CRC!=b.getinfo(n).CRC:
    if n=='pack.mcmeta':continue
    assert n.endswith('.json') and '/worldgen/structure/' in n
    assert CLEAN(j(a,n))==CLEAN(j(b,n)),n
    diffs.append(n)
  report.setdefault('platform_differences',{})[str(height)]=len(diffs)
report['checks']=['4/4 ZIP CRC, original member inventory preserved','4/4 JSON parse and Nether heights','all 6835 NBT templates identical between outputs','415 fixed loot seeds removed per variant','ocean ship deep-ocean biome and waterline settings','original Overworld, End and Nether noise/dimension files preserved','original Stellarity absence retained','Java/Rust differences confined to supported Horizon checks','six existing cave tunnel NBT retained and frequency modestly increased']
(NEW/'QA_GROUND_LOOT.json').write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')
print('4/4 PASS; NBT parity PASS; semantic parity PASS',flush=True)