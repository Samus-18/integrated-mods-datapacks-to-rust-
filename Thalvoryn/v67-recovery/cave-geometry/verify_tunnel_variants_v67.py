#!/usr/bin/env python3
"""Independent checks against four V67 cave-geometry test packs; no runtime claims."""
import sys,hashlib,zipfile,json,collections
from pathlib import Path
sys.path.insert(0, '/mnt/data/thalvoryn_v67_terrain_loot')
from nbt_codec_v67 import read_nbt
ROOT=Path('/mnt/data/Thalvoryn_V67_SPAWN_RECHECK')
OUT=ROOT/'GEOMETRY_TEST'
TEMPLATES=[f'data/thalvoryn/structure/underground/v67_cave_tunnel_{n:02}.nbt' for n in range(1,7)]
DEFS=[f'data/thalvoryn/worldgen/structure/underground/v67_cave_tunnel_{n:02}.json' for n in range(1,7)]
CHANGED=set(TEMPLATES+DEFS+['pack.mcmeta'])

def sig(p):return hashlib.sha256(p).hexdigest()

def audit():
 results=[]; template_sigs=[]
 for platform in ('RUST','JAVA'):
  for height in (1024,2032):
   name=f'Thalvoryn-{platform}-Nether{height}-V67-CAVE-GEOMETRY-TEST.zip'
   src=ROOT/f'Thalvoryn-{platform}-Nether{height}-V67-SPAWN-RECHECK-TEST.zip'
   dst=OUT/name
   with zipfile.ZipFile(src) as old,zipfile.ZipFile(dst) as new:
    assert old.testzip() is None and new.testzip() is None
    assert len(old.namelist())==len(new.namelist())==11323
    assert old.namelist()==new.namelist()
    assert all(not k.startswith('data/stellarity/') for k in new.namelist())
    diff=[]
    for path in new.namelist():
     if old.read(path)!=new.read(path):diff.append(path)
    assert set(diff)==CHANGED,(name,set(diff)^CHANGED)
    templates=[]
    for path in TEMPLATES:
      blob=new.read(path)
      templates.append(sig(blob))
      _,root=read_nbt(blob)
      sz=root['size'].value[1]
      assert sz==[79,93,114]
      block_list=root['blocks'].value[1]
      ids=[v['id'].value for v in root['palette'].value[1]]
      air=set()
      poses=set()
      for block in block_list:
       xyz=tuple(block['pos'].value[1]);assert xyz not in poses,(path,xyz)
       poses.add(xyz)
       assert 0<=xyz[0]<sz[0] and 0<=xyz[1]<sz[1] and 0<=xyz[2]<sz[2]
       if ids[block['state'].value]=='minecraft:air':air.add(xyz)
      assert len(air)>30000
      # Independent reachable-air BFS from exterior entrance to deep cavern.
      entrance=[v for v in air if v[2]<=5 and v[1]>=78]
      assert entrance,path
      seen={entrance[0]};stack=[entrance[0]]
      while stack:
       x,y,z=stack.pop()
       for dest in ((x+1,y,z),(x-1,y,z),(x,y+1,z),(x,y-1,z),(x,y,z+1),(x,y,z-1)):
        if dest in air and dest not in seen:
         seen.add(dest);stack.append(dest)
      deep=[(x,y,z) for (x,y,z) in seen if z>=68 and y<35]
      assert len(deep)>13000,(path,len(deep))
      assert max(v[1] for v in seen)>=84 and min(v[1] for v in seen)<=9
      # Source NBT had only 13-14K blocks, new cave sizes must be non-identical.
      assert len(block_list)>40000
    assert len(set(templates))==6
    template_sigs.append(templates)
    for path in DEFS:
      doc=json.loads(new.read(path))
      assert doc['start_height']=={'absolute':-85}
      assert doc['project_start_to_heightmap']=='OCEAN_FLOOR_WG'
      assert doc.get('terrain_adaptation')=='beard_thin'
      assert doc['max_distance_from_center']['horizontal']==116
      assert doc['max_distance_from_center']['vertical']>=93
    cave_set=json.loads(new.read('data/thalvoryn/worldgen/structure_set/ecology/ow_cave_entrances.json'))
    assert len(cave_set['structures'])==6 and cave_set['placement']['spacing']==32 and cave_set['placement']['separation']==25
    # Compare world noise, loot, and biome resources bit-for-bit against previous release.
    intact=sum(1 for p in new.namelist() if ('/worldgen/density_function/' in p or '/worldgen/biome/' in p or '/loot_table/' in p) and old.read(p)==new.read(p))
    assert intact>1200
    results.append({'filename':name,'sha256':sig(dst.read_bytes()),'members':len(new.namelist()),'modified':len(diff),'templates_unchanged_except_six':True,'intact_noise_biome_loot_files':intact,'size_bytes':dst.stat().st_size})
    print('PASS',name,len(diff),'changed; untouched worldgen/loot',intact,flush=True)
 assert all(v==template_sigs[0] for v in template_sigs)
 qa={'status':'PASS_STATIC_ONLY_NOT_GAME_TESTED','variants':results,'cave_entrance_template_hashes':template_sigs[0], 'known_limitation':'The created cave rooms connect to their hand-authored tunnels, not guaranteed natural procedural caves; no runtime profiling.'}
 (OUT/'CAVE_GEOMETRY_INDEPENDENT_QA.json').write_text(json.dumps(qa,indent=2,ensure_ascii=False)+'\n')
 return qa
if __name__=='__main__':audit()