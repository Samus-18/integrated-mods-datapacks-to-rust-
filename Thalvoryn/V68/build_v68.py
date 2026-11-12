#!/usr/bin/env python3
"""Thalvoryn V68 targeted static-only improvement, based on V67 CAVE-GEOMETRY.
Private archival-derived assets: do not publish resulting binary without licenses.
Changes: retire unused custom End/Nether ship resources, preserve existing correctly scaled Nether heights, tune End Castle spawns, improve small
surface structure terrain adaptation, update pack version and diagnostic function.
"""
from __future__ import annotations
import zipfile,glob,json,hashlib,re,collections,pathlib
ROOT=pathlib.Path('/mnt/data/v68_work'); OUT=ROOT/'v68_release_candidate';OUT.mkdir(exist_ok=True)
MAP={
 'RUST_1024':'Thalvoryn-RUST-Nether1024-V67-CAVE-GEOMETRY-TEST.zip',
 'RUST_2032':'Thalvoryn-RUST-Nether2032-V67-CAVE-GEOMETRY-TEST.zip',
 'JAVA_1024':'Thalvoryn-JAVA-Nether1024-V67-CAVE-GEOMETRY-TEST.zip',
 'JAVA_2032':'Thalvoryn-JAVA-Nether2032-V67-CAVE-GEOMETRY-TEST.zip',
}
IDS_TO_RETIRE_RE=re.compile(r'^data/thalvoryn/worldgen/structure/(?:end/imported/ships_\d+|nether/imported/ships_\d+|nether/handmade/blackstone_frigate)\.json$')
OPTIMIZE_TERRAIN={
 'data/thalvoryn/worldgen/structure/end/echo_shrine.json',
 'data/thalvoryn/worldgen/structure/nether/variant/basalt_watch.json',
 'data/thalvoryn/worldgen/structure/nether/variant/crimson_watch.json',
 'data/thalvoryn/worldgen/structure/nether/variant/soul_watch.json',
 'data/thalvoryn/worldgen/structure/nether/warped_watch.json',
 'data/trek/worldgen/structure/overworld/medium/camp_2.json',
 'data/trek/worldgen/structure/overworld/medium/oasis.json',
 'data/trek/worldgen/structure/overworld/medium/snow_cottage.json',
 'data/trek/worldgen/structure/overworld/medium/spruce_cottage.json',
 'data/trek/worldgen/structure/overworld/medium/well_pillager_medium.json',
 'data/trek/worldgen/structure/overworld/medium/wild_stone.json',
 'data/trek/worldgen/structure/overworld/medium/small_pyramide.json',
}
JS=lambda ob:(json.dumps(ob,ensure_ascii=False,indent=2,sort_keys=True)+'\n').encode('utf-8')
manifest={'version':'V68','source':'V67 CAVE-GEOMETRY','verification':'STATIC ONLY - NOT GAME TESTED','variants':{}}
for variant,filename in MAP.items():
  platform,ht=variant.split('_');height=int(ht)
  path=ROOT/filename
  with zipfile.ZipFile(path) as z:
    names=set(z.namelist());json_of=lambda n:json.loads(z.read(n))
    retire_def=sorted(n for n in names if IDS_TO_RETIRE_RE.match(n))
    assert len(retire_def)==47,(variant,len(retire_def))
    retire_ids=set('thalvoryn:'+n.split('/worldgen/structure/')[1].removesuffix('.json') for n in retire_def)
    # Their own pool and template may only be removed if not reached from elsewhere.
    retire_bundle=set(retire_def)
    for n in retire_def:
      for sub,ext in [('worldgen/template_pool','.json'),('structure','.nbt')]:
        pp=n.replace('/worldgen/structure/','/'+sub+'/').removesuffix('.json')+ext
        if pp in names:retire_bundle.add(pp)
    active=[]
    for n in names:
      if '/worldgen/structure_set/' in n and n.endswith('.json'):
        for st in json_of(n).get('structures',[]):
          if st.get('structure') in retire_ids:active.append((n,st['structure']))
    assert not active,(variant,active)
    # Check no JSON outside the removed resources references their structure ID,
    # start-pool ID, or NBT location. Ignore docs which refer to historical source.
    references=[]
    for n in names:
      if n in retire_bundle or not n.endswith('.json'):continue
      if '/worldgen/' not in n:continue
      body=z.read(n).decode('utf-8')
      for rid in retire_ids:
        if rid in body: references.append((n,rid))
    assert not references,(variant,references[:20])
    # Preserve third-party template asset NBT if referenced through a nested jigsaw
    # path not present in JSON; all associated pool/resource files otherwise dropped.
    # We retain their NBT for conservative compatibility and remove ONLY root structures & pools.
    retire_bundle={n for n in retire_bundle if '/worldgen/' in n}
    updates={}; changed={}; count_height=0;count_terrain=0;count_mobs=0
    def put(n, data, reason):updates[n]=data;changed[n]=reason
    for n in names:
      if '/worldgen/structure/' not in n or not n.endswith('.json') or n in retire_bundle:continue
      if n.startswith('data/trek/') and 'ship' in n:continue
      j=json_of(n);edited=False;reasons=[]
      if n in OPTIMIZE_TERRAIN and j.get('project_start_to_heightmap')=='WORLD_SURFACE_WG' and j.get('terrain_adaptation')=='none':
        radius=j.get('max_distance_from_center',80);radius=radius.get('horizontal',80) if isinstance(radius,dict) else radius
        assert radius<=116,(n,radius)
        j['terrain_adaptation']='beard_thin';edited=True;reasons.append('gentle structure/ground blend');count_terrain+=1
      if n=='data/nova_structures/worldgen/structure/end_castle.json':
        for e in j.get('spawn_overrides',{}).get('monster',{}).get('spawns',[]):
          if e.get('type')=='minecraft:endermite' and e['weight']>100:
            e['weight']=35;co=e['count'];
            if isinstance(co,dict):co['max_inclusive']=min(co['max_inclusive'],2)
            edited=True;reasons.append('limit End Castle endermites');count_mobs+=1
      if edited:put(n,JS(j),'; '.join(reasons))
    version=f'Thalvoryn V68 {platform} Nether{height} - PRIVATE STATIC TEST'
    meta=json_of('pack.mcmeta');meta['pack']['description']=version;put('pack.mcmeta',JS(meta),'bump version V68')
    put('data/thalvoryn/function/version.mcfunction',(f'tellraw @s {{"text":"Thalvoryn V68 | {platform} | Nether {height} | STATIC TEST - not game verified","color":"aqua"}}\n').encode(),'new version check')
    put('data/thalvoryn/function/check.mcfunction',(f'tellraw @s {{"text":"Thalvoryn V68 loaded ({platform}, Nether {height}). Test only in a new world.","color":"green"}}\nfunction trek:check\n').encode(),'version display')
    put('data/thalvoryn/function/help.mcfunction',z.read('data/thalvoryn/function/help.mcfunction').replace(b'Thalvoryn 26.3 RC2',b'Thalvoryn V68 26.3'),'refresh outdated help version')
    put('data/thalvoryn/function/horizon/status.mcfunction',z.read('data/thalvoryn/function/horizon/status.mcfunction').replace(b'RC41',b'V68'),'refresh outdated manual status label')
    note=f'Version V68. Target Minecraft 26.3 / Horizon commit 2ebf00a. Private static test. Use /function thalvoryn:version. Removed inactive End/Nether pirate ship *definitions and root pools*, retained archived NBT source templates. Actual generation, performance, terrain blending and tunnel connections NOT GAME TESTED.\n'
    put('V68_CHANGELOG_CZ.txt',note.encode(),'release note')
    output=OUT/f'Thalvoryn-{platform}-Nether{height}-V68-PRIVATE-TEST.zip'
    with zipfile.ZipFile(output,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6) as dst:
      for old in z.infolist():
        n=old.filename
        if n in retire_bundle:continue
        data=updates.pop(n,z.read(n))
        dst.writestr(old,data,compress_type=zipfile.ZIP_DEFLATED,compresslevel=6)
      for n,data in updates.items():dst.writestr(n,data,compress_type=zipfile.ZIP_DEFLATED,compresslevel=6)
    manifest['variants'][variant]={'out_file':output.name,'sha256':hashlib.sha256(output.read_bytes()).hexdigest(),'bytes':output.stat().st_size,'retired_ship_definition_and_pools':len(retire_bundle),'remaining_nbt_sources_for_audit':len([x for x in retire_def if x.replace('/worldgen/structure/','/structure/').replace('.json','.nbt') in names]),'nether_height_gates_adjusted':count_height,'ground_adaptation_adjusted':count_terrain,'end_spawn_entries_adjusted':count_mobs,'changed_json':changed}
    print(variant,output.name,'retired',len(retire_bundle),'height',count_height,'terrain',count_terrain,'spawns',count_mobs,flush=True)
with open(OUT/'V68_BUILD_MANIFEST.json','wb') as f:f.write(JS(manifest))