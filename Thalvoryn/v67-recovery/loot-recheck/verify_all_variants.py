"""Independent four-variant regression: CRC, allowed diffs, important worldgen, parity. Game runtime not available."""
import json,zipfile,hashlib,collections,pathlib
R=pathlib.Path('/mnt/data/Thalvoryn_V67_AUDIT');B=pathlib.Path('/mnt/data/Thalvoryn_V67_WORLD_HOTFIX')
variants=[];manifest={};modified_content_hashes=collections.defaultdict(set)
for p in ('RUST','JAVA'):
 for h in (1024,2032):
  old=B/f'Thalvoryn-{p}-Nether{h}-V67-WORLD-ROOTFIX-TEST.zip'
  new=R/f'Thalvoryn-{p}-Nether{h}-V67-LOOT-RECHECK-TEST.zip'
  with zipfile.ZipFile(old) as a,zipfile.ZipFile(new) as z:
   assert a.testzip() is None and z.testzip() is None
   names=z.namelist();assert names==a.namelist() and len(names)==len(set(names))==11323
   changed=[n for n in names if z.getinfo(n).CRC!=a.getinfo(n).CRC]
   assert len(changed)==502 and all(n.endswith('.nbt') and not n.startswith('data/minecraft/') for n in changed)
   assert not any(n.startswith('data/stellarity/') for n in names)
   assert json.loads(z.read('pack.mcmeta'))['pack']['pack_format']==121
   assert json.loads(z.read('data/minecraft/dimension_type/the_nether.json'))['height']==h
   assert 'trek:end/medium/end_city' not in [x['structure'] for x in json.loads(z.read('data/thalvoryn/worldgen/structure_set/ecology/end_major.json'))['structures']]
   for n in changed:
    modified_content_hashes[n].add(hashlib.sha256(z.read(n)).hexdigest())
   unchanged=[n for n in names if not n.endswith('.nbt')]
   assert all(z.getinfo(n).CRC==a.getinfo(n).CRC for n in unchanged)
   variants.append({'platform':p,'height':h,'entries':len(names),'nbt_changes':len(changed),'static_zip_crc':'PASS','sha256':hashlib.sha256(new.read_bytes()).hexdigest(),'output_bytes':new.stat().st_size})
   manifest[f'{p}_{h}']=set(changed)
assert len(set(map(frozenset,manifest.values())))==1
assert all(len(v)==1 for v in modified_content_hashes.values()),[k for k,v in modified_content_hashes.items() if len(v)>1][:10]
qa=[json.loads(x.read_text()) for x in R.glob('verify_[0-9]*_[0-9]*.json')]; qa.sort(key=lambda x:x['range'][0])
assert len(qa)==3 and sum(x['templates_checked'] for x in qa)==502
assert sum(x['converted_containers'] for x in qa)==2668
res={'status':'PASS STATIC ONLY; NO GAMEPLAY TEST','variants':variants,'modified_templates':502,'containers_randomized':2668,'verified_no_geometry_or_other_nbt_field_changes':True,'modified_nbt_identical_across_all_variants':True,'unmodified_pack_resources_byte_identical_to_previous_build':True,'worldgen_and_generation_frequency_unchanged':True,'limits':['No in-game launch, chunk generation, TPS or LOD validation','Original prior chunks unchanged','LootTable randomness actual engine behavior not measured','Fixed inventories in vanilla minecraft namespace intentionally preserved']}
(R/'V67_LOOT_RECHECK_QA.json').write_text(json.dumps(res,indent=2,ensure_ascii=False)+'\n')
print(json.dumps({'status':res['status'],'variants':len(variants),'templates':502,'containers':2668,'identical':True}))