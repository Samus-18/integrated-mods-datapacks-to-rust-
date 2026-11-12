#!/usr/bin/env python3
"""Independent changed-file whitelist and registry integrity checks for V68 archives."""
from __future__ import annotations
import zipfile,hashlib,json,collections,pathlib,sys,re
ROOT=pathlib.Path('/mnt/data/v68_work');OUT=ROOT/'v68_release_candidate'
MAP={'RUST_1024':'Thalvoryn-RUST-Nether1024-V67-CAVE-GEOMETRY-TEST.zip','RUST_2032':'Thalvoryn-RUST-Nether2032-V67-CAVE-GEOMETRY-TEST.zip','JAVA_1024':'Thalvoryn-JAVA-Nether1024-V67-CAVE-GEOMETRY-TEST.zip','JAVA_2032':'Thalvoryn-JAVA-Nether2032-V67-CAVE-GEOMETRY-TEST.zip'}
RESULT={'status':'PASS_STATIC_ONLY_NOT_GAME_TESTED','checks':{},'variants':{},'warnings':['No actual Horizon/Java launch in this environment','Terrain adaptation does not guarantee every block of a structure physically intersects the ground','Tunnel-to-natural-cave connections, FPS/TPS, chunk loading and open-chest behavior are untested']}
JS=lambda j:(json.dumps(j,indent=2,sort_keys=True,ensure_ascii=False)+'\n').encode()
all_nbthashes=[];all_whitelists=[]
for v,srcfn in MAP.items():
  platform,hstr=v.split('_');h=int(hstr)
  out=OUT/f'Thalvoryn-{platform}-Nether{h}-V68-PRIVATE-TEST.zip'
  with zipfile.ZipFile(ROOT/srcfn) as src, zipfile.ZipFile(out) as dst:
    s1=set(src.namelist());s2=set(dst.namelist())
    assert len(src.namelist())==len(s1) and len(dst.namelist())==len(s2),'duplicated paths'
    assert dst.testzip() is None,'corrupt CRC'
    removed=s1-s2;added=s2-s1;changed={name for name in s1&s2 if src.read(name)!=dst.read(name)}
    assert len(removed)==94,(v,len(removed));assert all(name.startswith('data/thalvoryn/worldgen/') and '/ships_' in name or name.endswith('/blackstone_frigate.json') for name in removed),removed
    assert added=={'V68_CHANGELOG_CZ.txt','data/thalvoryn/function/version.mcfunction'},added
    assert changed==({'pack.mcmeta','data/thalvoryn/function/check.mcfunction','data/thalvoryn/function/help.mcfunction','data/thalvoryn/function/horizon/status.mcfunction','data/nova_structures/worldgen/structure/end_castle.json'}|set([n for n in changed if '/worldgen/structure/' in n and n.endswith('.json') and n!='data/nova_structures/worldgen/structure/end_castle.json'])),(v,len(changed),changed)
    changed_structures={n for n in changed if '/worldgen/structure/' in n and n.endswith('.json')}
    assert len(changed_structures)==13,(v,len(changed_structures),changed_structures)
    assert all(src.read(n)==dst.read(n) for n in s1 if n.endswith('.nbt')),'modified NBT'
    assert all(src.read(n)==dst.read(n) for n in s1 if '/loot_table/' in n or '/loot_tables/' in n),'modified loot'
    assert all(src.read(n)==dst.read(n) for n in s1 if '/worldgen/density_function/' in n or '/worldgen/noise_settings/' in n),'modified density/noise'
    assert all(src.read(n)==dst.read(n) for n in s1 if '/worldgen/structure_set/' in n),'modified structure placement sets'
    assert all(src.read(n)==dst.read(n) for n in s1 if '/dimension/' in n and n.endswith('.json')),'modified dimensions'
    assert b'V68' in dst.read('pack.mcmeta') and b'V68' in dst.read('data/thalvoryn/function/check.mcfunction')
    assert 'Stellarity' not in dst.namelist() and not any(n.startswith('data/stellarity/') for n in s2)
    assert json.loads(dst.read('pack.mcmeta'))['pack']['pack_format']==121
    c=0
    for n in s2:
      if n.endswith('.json') or n.endswith('.mcmeta'):
        try:json.loads(dst.read(n))
        except Exception as e:raise ValueError(f'{v} bad JSON {n}: {e}')
        c+=1
    sets=[n for n in s2 if '/worldgen/structure_set/' in n and n.endswith('.json')]
    defs={n.replace('data/','').replace('/worldgen/structure/',':').removesuffix('.json') for n in s2 if '/worldgen/structure/' in n and n.endswith('.json')}
    entry=[]
    for n in sets:
      j=json.loads(dst.read(n));entry.extend(x['structure'] for x in j.get('structures',[]))
    assert len(entry)==len(set(entry)),(v,'duplicate structure across structure sets')
    assert not [id for id in entry if '/end/imported/ships_' in id or '/nether/imported/ships_' in id or 'blackstone_frigate' in id],(v,'retired ship active')
    assert not [id for id in entry if id not in defs and not id.startswith('minecraft:')],(v,'missing defined active structures')
    assert 'minecraft:end_city' in entry
    assert 'nova_structures:end_castle' in entry
    assert all(n in s2 for n in ['data/minecraft/dimension/the_end.json','data/minecraft/dimension/the_nether.json'])
    assert json.loads(dst.read('data/minecraft/dimension/the_end.json'))['generator']['biome_source']=={'type':'minecraft:the_end'}
    for n in changed_structures:
      old=json.loads(src.read(n));new=json.loads(dst.read(n))
      if n.endswith('/end_castle.json'):
        olde=next(x for x in old['spawn_overrides']['monster']['spawns'] if x['type']=='minecraft:endermite')
        newe=next(x for x in new['spawn_overrides']['monster']['spawns'] if x['type']=='minecraft:endermite')
        assert newe['weight']==35 and newe['count']['max_inclusive']==2
        old['spawn_overrides']['monster']['spawns']=[x for x in old['spawn_overrides']['monster']['spawns'] if x['type']!='minecraft:endermite']
        new['spawn_overrides']['monster']['spawns']=[x for x in new['spawn_overrides']['monster']['spawns'] if x['type']!='minecraft:endermite']
        assert old==new
      else:
        assert old.get('terrain_adaptation')=='none' and new.get('terrain_adaptation')=='beard_thin',n
        old['terrain_adaptation']='beard_thin';assert old==new,n
    nbts=[n for n in s2 if n.endswith('.nbt')];digest=hashlib.sha256()
    for n in sorted(nbts):digest.update(n.encode());digest.update(hashlib.sha256(dst.read(n)).digest())
    all_nbthashes.append(digest.hexdigest())
    all_whitelists.append(sorted(changed))
    RESULT['variants'][v]={'status':'PASS_STATIC','zip_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'files':len(s2),'removed_dormant_ship_definitions_and_pool_files':len(removed),'changed_existing_files':len(changed),'new_files':len(added),'jigsaw_structure_sets':len(sets),'active_structure_ids':len(entry),'json_mcmeta_checked':c,'nbt_files_unchanged':len(nbts),'nbt_content_digest':digest.hexdigest()}
    print(v,'PASS', 'files',len(s2),'changed',len(changed),'active',len(entry),'nbt',len(nbts),flush=True)
assert len(set(all_nbthashes))==1,'cross-variant NBT content mismatch'
assert len(set(tuple(x) for x in all_whitelists))==1,'cross-variant changed-path mismatch'
RESULT['checks']={'all_variants_crc':'PASS','changed_paths_whitelisted':'PASS','loot_and_NBT_identical_to_V67':'PASS','noise_and_biome_sources_identical_to_V67':'PASS','unreferenced_end_nether_ship_root_removed':'PASS','end_city_vanilla_preserved':'PASS','stellarity_absent':'PASS','horizon_net_height_preserved':'PASS','identical_NBT_across_variants':'PASS','game_runtime':'NOT_RUN'}
with open(OUT/'V68_INDEPENDENT_QA.json','wb') as f:f.write(JS(RESULT))
print('FINAL',RESULT['status'])