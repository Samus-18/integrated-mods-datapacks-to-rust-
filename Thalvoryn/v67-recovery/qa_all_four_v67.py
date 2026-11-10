from pathlib import Path
import zipfile,hashlib,json,collections
D=Path(__file__).resolve().parent
archives={(plat,h):next(D.glob(f'Thalvoryn-{plat}-Nether{h}-V67-INTEGRATION-TEST-UNRELEASED.zip')) for plat in ('JAVA','RUST') for h in (1024,2032)}
results={};zips={}
try:
 for (plat,h),f in archives.items():
  z=zipfile.ZipFile(f);zips[plat,h]=z
  names=z.namelist();assert z.testzip() is None
  assert len(names)==len(set(names))==15506
  assert json.loads(z.read('data/minecraft/dimension_type/the_nether.json'))['height']==h
  assert json.loads(z.read('data/minecraft/worldgen/noise_settings/nether.json'))['noise']['height']==h
  assert json.loads(z.read('data/minecraft/dimension_type/the_end.json'))['height']==512
  assert json.loads(z.read('data/minecraft/worldgen/noise_settings/end.json'))['noise']['height']==512
  e=json.loads(z.read('data/minecraft/dimension/the_end.json'))
  assert e['generator']['biome_source']['type']=='minecraft:multi_noise'
  assert len({o['biome'] for o in e['generator']['biome_source']['biomes']})==24
  av=json.loads(z.read('data/thalvoryn/worldgen/structure_set/overworld/v67_giant_air_vessel.json'))
  assert av['structures'][0]['structure']=='thalvoryn:overworld/handmade/air_vessel'
  andEnd=json.loads(z.read('data/thalvoryn/worldgen/structure_set/end/v67_spaced_monuments.json'))
  assert len(andEnd['structures'])==48 and andEnd['placement']['exclusion_zone']['other_set']=='minecraft:end_cities'
  results[f'{plat}_{h}']={'size_bytes':f.stat().st_size,'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),
                        'files':len(names),'native_horizon_checks':len([n for n in names if n.startswith('data/thalvoryn/worldgen/structure/nether/') and n.endswith('.json') and b'horizon:start_checks' in z.read(n)])}
 base=set(zips['JAVA',1024].namelist());assert all(set(z.namelist())==base for z in zips.values())
 # NBT templates are identical for all four variants; source packs may vary only custom game checks.
 base_nbt={n:hashlib.sha256(zips['JAVA',1024].read(n)).digest() for n in base if n.endswith('.nbt')}
 for z in zips.values():
  assert all(hashlib.sha256(z.read(n)).digest()==sha for n,sha in base_nbt.items())
 report={'status':'4/4 static PASS; NOT GAME TESTED','files_per_pack':len(base),
         'new_source_compatibility':'MC 26.3 native json + Horizon commit 2ebf00a target',
         'nbt_identical_across_platforms_and_heights':True,'nbt_templates_per_pack':len(base_nbt),
         'variants':results,'game_tests':'NOT RUN'}
 (D/'v67_qa_four_variants.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
 print(json.dumps(report,ensure_ascii=False,indent=2)[:3200])
finally:
 for z in zips.values():z.close()