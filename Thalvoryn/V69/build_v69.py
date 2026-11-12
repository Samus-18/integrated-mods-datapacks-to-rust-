#!/usr/bin/env python3
"""Thalvoryn V69 — constrained worldgen + structure mob balance.
No guarantee of in-game behavior. Build from four user-owned V68 private archives.

Only changes approved structure JSONs, metadata, and status function files.
No NBT assets, loot tables, biomes, noise, density or structure sets are edited.
"""
from __future__ import annotations
from pathlib import Path
import zipfile, json, hashlib, copy, re

ROOT=Path('/mnt/data/v68_work')
OUT=ROOT/'v69_release_candidate';OUT.mkdir(exist_ok=True)
BASE={
 'JAVA_1024': ROOT/'v68_release_candidate/Thalvoryn-JAVA-Nether1024-V68-PRIVATE-TEST.zip',
 'JAVA_2032': ROOT/'v68_release_candidate/Thalvoryn-JAVA-Nether2032-V68-PRIVATE-TEST.zip',
 'RUST_1024': ROOT/'v68_release_candidate/Thalvoryn-RUST-Nether1024-V68-PRIVATE-TEST.zip',
 'RUST_2032': ROOT/'v68_release_candidate/Thalvoryn-RUST-Nether2032-V68-PRIVATE-TEST.zip',
}
NETHER_TREK={
 'data/trek/worldgen/structure/nether/common/nether_pillar.json',
 'data/trek/worldgen/structure/nether/medium/overworld_leak_1.json',
 'data/trek/worldgen/structure/nether/medium/overworld_leak_2.json'
}
NETHER_BRIDGES={
 'data/thalvoryn/worldgen/structure/nether/cinder_viaduct.json',
 'data/thalvoryn/worldgen/structure/nether/skybridge_ruin.json'
}
NETHER_GROUND_RE=re.compile(r'^data/thalvoryn/worldgen/structure/nether/(?:imported/buildings_\d+|sites/[^/]+)\.json$')
END_GROUND_RE=re.compile(r'^data/thalvoryn/worldgen/structure/end/imported/buildings_\d+\.json$')
THUNDERFUSION_END_RE=re.compile(r'^data/thunderfusion/worldgen/structure/end/[^/]+\.json$')
MOB_PATCH={
 'data/nova_structures/worldgen/structure/stray_fort.json': {
   'minecraft:stray': {'max':4, 'weight':140},
   'minecraft:zombie': {'max':4},
 },
 'data/nova_structures/worldgen/structure/toxic_lair.json': {
   'minecraft:cave_spider': {'max':3, 'weight':90},
   'minecraft:zombie': {'max':4},
   'minecraft:witch': {'weight':100},
 },
 'data/nova_structures/worldgen/structure/sealing_halls.json': {
   'minecraft:blaze': {'weight':140},
   'minecraft:wither_skeleton': {'max':3, 'weight':110},
   'minecraft:magma_cube': {'weight':100},
   'minecraft:skeleton': {'max':3},
 },
 'data/nova_structures/worldgen/structure/end_castle.json': {
   'minecraft:phantom': {'max':2, 'weight':25},
 },
}

def js(obj):return (json.dumps(obj,sort_keys=True,ensure_ascii=False,indent=2)+'\n').encode()

manifest={'version':'V69','based_on':'V68 private candidate','test_status':'STATIC ONLY: game not run','changes_by_variant':{}}
for variant,src in BASE.items():
  platform,hstring=variant.split('_'); height=int(hstring)
  output=OUT/f'Thalvoryn-{platform}-Nether{height}-V69-PRIVATE-TEST.zip'
  with zipfile.ZipFile(src) as orig:
    names=set(orig.namelist()); updates={}; changes=[];counts={
       'trek_nether_surface':0,'nether_bridge_terrain':0,'nether_surface_rust_checks':0,
       'end_surface_rust_checks':0,'mob_structures_tuned':0,'mob_entries_tuned':0}
    def put(n,b,reason):
      assert n in names or n=='V69_CHANGELOG_CZ.txt'
      updates[n]=b
      changes.append({'path':n,'reason':reason})
    assert all(n in names for n in (NETHER_TREK | NETHER_BRIDGES | set(MOB_PATCH)))
    for n in sorted(x for x in names if '/worldgen/structure/' in x and x.endswith('.json')):
      j=json.loads(orig.read(n));original=copy.deepcopy(j);tags=[]
      if n in NETHER_TREK:
         assert j['type']=='minecraft:jigsaw' and j.get('project_start_to_heightmap') is None
         assert j['start_height']['type']=='minecraft:trapezoid'
         j['project_start_to_heightmap']='WORLD_SURFACE_WG'
         j['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':-6},'max_inclusive':{'absolute':-2}}
         # Preserve their intended structural step, biome eligibility, layout and mob overrides.
         counts['trek_nether_surface']+=1
         tags.append('project to real Nether surface instead of absolute Y64..340')
      if n in NETHER_BRIDGES:
         assert j['max_distance_from_center']<=116 and j.get('terrain_adaptation')=='none'
         j['terrain_adaptation']='beard_thin';counts['nether_bridge_terrain']+=1
         tags.append('terrain beard to soften exposed bridge footing')
      if platform=='RUST' and (n in NETHER_TREK or NETHER_GROUND_RE.match(n)):
         assert j.get('project_start_to_heightmap')=='WORLD_SURFACE_WG',n
         old=j.get('horizon:start_checks',{})
         # Reject obvious lava starts and severe nearby terrain discontinuities.
         new={**old,'liquid':'top','terrain_radius':1,'terrain_range':64,'min_y':45,'max_y':height-32}
         if old!=new:
             j['horizon:start_checks']=new;counts['nether_surface_rust_checks']+=1
             tags.append('Horizon-only slope/liquid/height checks for ground structure')
      if platform=='RUST' and (END_GROUND_RE.match(n) or THUNDERFUSION_END_RE.match(n)):
         if j.get('project_start_to_heightmap')=='WORLD_SURFACE_WG':
             old=j.get('horizon:start_checks',{})
             new={**old,'terrain_radius':1,'terrain_range':80,'min_y':44}
             if old!=new:
                j['horizon:start_checks']=new;counts['end_surface_rust_checks']+=1
                tags.append('Horizon-only End-island grounding check')
      if n in MOB_PATCH:
         edited=0
         for k,spawns in j.get('spawn_overrides',{}).items():
            for s in spawns.get('spawns',[]):
               rules=MOB_PATCH[n].get(s.get('type'))
               if not rules:continue
               if 'weight' in rules:
                  assert s['weight']>rules['weight'],(n,s)
                  s['weight']=rules['weight'];edited+=1
               if 'max' in rules:
                  obj=s.get('count')
                  assert isinstance(obj,dict) and obj.get('type')=='minecraft:uniform'
                  assert obj['max_inclusive']>rules['max'],(n,s)
                  obj['max_inclusive']=rules['max'];obj['min_inclusive']=min(obj['min_inclusive'],rules['max']);edited+=1
         assert edited>0,n
         counts['mob_structures_tuned']+=1;counts['mob_entries_tuned']+=edited
         tags.append('cap excessive structure-spawn weights/group maxima')
      if j!=original:
         put(n,js(j),'; '.join(tags))
    meta=json.loads(orig.read('pack.mcmeta'))
    meta['pack']['description']=f'Thalvoryn V69 {platform} Nether{height} — PRIVATE TEST (MC 26.3)'
    put('pack.mcmeta',js(meta),'update V69 packaging description')
    put('data/thalvoryn/function/version.mcfunction',(f'tellraw @s {{"text":"Thalvoryn V69 | {platform} | Nether {height} | PRIVATE STATIC TEST - not game verified","color":"aqua"}}\n').encode(),'V69 version command')
    put('data/thalvoryn/function/check.mcfunction',(f'tellraw @s {{"text":"Thalvoryn V69 ({platform}, Nether {height}) loaded – private test.","color":"green"}}\nfunction trek:check\n').encode(),'V69 check command')
    old_help=orig.read('data/thalvoryn/function/help.mcfunction')
    put('data/thalvoryn/function/help.mcfunction',old_help.replace(b'V68',b'V69'),'V69 help label')
    old_status=orig.read('data/thalvoryn/function/horizon/status.mcfunction')
    put('data/thalvoryn/function/horizon/status.mcfunction',old_status.replace(b'V68',b'V69'),'V69 status label')
    put('V69_CHANGELOG_CZ.txt',('Thalvoryn V69. Three TREK Nether structures now follow terrain; two Nether bridges have thin terrain blending; Horizon-only checks reject unsuitable starts of imported Nether/End buildings; four high-mob structures rebalanced. All structural geometry/loot/worldgen unchanged. STATIC TEST ONLY; cannot guarantee TPS or structure placement until tested in game.\n').encode(),'V69 changelog')
    assert counts['trek_nether_surface']==3 and counts['nether_bridge_terrain']==2
    assert counts['mob_structures_tuned']==4
    assert (platform=='RUST')==(counts['nether_surface_rust_checks']>0)
    assert (platform=='RUST')==(counts['end_surface_rust_checks']>0)
    with zipfile.ZipFile(output,'w',zipfile.ZIP_DEFLATED,compresslevel=6) as dst:
        for member in orig.infolist():
            name=member.filename
            data=updates.pop(name,orig.read(name))
            dst.writestr(member,data,compress_type=zipfile.ZIP_DEFLATED,compresslevel=6)
        for n,data in updates.items():dst.writestr(n,data)
  manifest['changes_by_variant'][variant]={'file':output.name,'sha256':hashlib.sha256(output.read_bytes()).hexdigest(),'bytes':output.stat().st_size,'counts':counts,'changes':changes}
  print(variant,counts,'bytes',output.stat().st_size,flush=True)
(OUT/'V69_BUILD_MANIFEST.json').write_bytes(js(manifest))