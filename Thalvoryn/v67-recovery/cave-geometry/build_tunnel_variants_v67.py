#!/usr/bin/env python3
"""V67: six geometrically distinct steep cave entrances and roomy reachable caverns.
Only changes six original NBT templates + pack descriptions. This does not claim
natural cave connectivity, runtime placement correctness, or worldgen perf parity.
"""
from __future__ import annotations
import sys, math, random, zipfile, json, hashlib
from pathlib import Path
from collections import defaultdict, deque
sys.path.insert(0, '/mnt/data/thalvoryn_v67_terrain_loot')
from nbt_codec_v67 import basic_template, read_nbt

ROOT=Path('/mnt/data/Thalvoryn_V67_SPAWN_RECHECK')
OUT=ROOT/'GEOMETRY_TEST'
OUT.mkdir(exist_ok=True)
SZ=(79,93,114)
STYLES=[
 ['minecraft:stone','minecraft:mossy_stone_bricks','minecraft:tuff','minecraft:andesite','minecraft:oak_planks','minecraft:lantern','minecraft:glow_lichen','minecraft:air','minecraft:gravel'],
 ['minecraft:stone','minecraft:stone_bricks','minecraft:calcite','minecraft:polished_andesite','minecraft:spruce_planks','minecraft:lantern','minecraft:glow_lichen','minecraft:air','minecraft:gravel'],
 ['minecraft:stone','minecraft:tuff','minecraft:deepslate','minecraft:andesite','minecraft:dark_oak_planks','minecraft:lantern','minecraft:glow_lichen','minecraft:air','minecraft:gravel'],
 ['minecraft:stone','minecraft:dripstone_block','minecraft:calcite','minecraft:andesite','minecraft:oak_planks','minecraft:lantern','minecraft:glow_lichen','minecraft:air','minecraft:gravel'],
 ['minecraft:stone','minecraft:moss_block','minecraft:rooted_dirt','minecraft:tuff','minecraft:oak_planks','minecraft:lantern','minecraft:glow_lichen','minecraft:air','minecraft:gravel'],
 ['minecraft:stone','minecraft:andesite','minecraft:polished_andesite','minecraft:tuff','minecraft:oak_planks','minecraft:lantern','minecraft:glow_lichen','minecraft:air','minecraft:gravel'],
]
# IDs: wall=0, accent=1, rock=2, support=3, planks=4, light=5, lichen=6, air=7, gravel=8.
# Each tunnel has a fixed irregular centerline, instead of retexturing an identical corridor.
CURVES=[
 (6.0, .085, .4, 2.2, .19, .1, 0),
 (9.0, .078, 1.8, 3.0, .17, .7, -3),
 (5.0, .15, 1.0, 4.0, .064, 1.1, 5),
 (10.0, .11, 2.8, 3.1, .24, .4, -1),
 (7.5, .079, 4.3, 5.4, .13, 2.3, 2),
 (9.0, .065, 3.3, 5.0, .16, 3.6, -4),
]

def make_tunnel(style):
    rng=random.Random(92371+style*991)
    palette=STYLES[style]
    vox={}
    # Individualised cavern centers, offsets, and lobe placements.
    cvx=[40,41,37,42,39,37][style]
    cvz=[80,84,82,81,85,83][style]
    cvy=20
    rx=[23,26,24,27,25,26][style]
    rz=[26,25,28,25,27,24][style]
    ry=[13,16,15,14,14,17][style]
    def put(x,y,z,idx):
        if 0<=x<SZ[0] and 0<=y<SZ[1] and 0<=z<SZ[2]:vox[x,y,z]=idx
    # Curved, uneven cave, with a tight rough stone lip. No huge square template slab.
    for z in range(max(1,cvz-rz-3),min(SZ[2]-1,cvz+rz+4)):
      for x in range(max(1,cvx-rx-4),min(SZ[0]-1,cvx+rx+5)):
        ang=math.atan2(z-cvz,x-cvx)
        n=.080*math.sin(ang*(5+style%3)+style)+.045*math.cos(x*.31+z*.17+style*2)
        nx=(x-cvx)/(rx*(1+n)); nz=(z-cvz)/(rz*(1+n))
        planar=nx*nx+nz*nz
        if planar>1.35:continue
        for y in range(3,min(SZ[1]-5, cvy+ry+7)):
          ny=(y-cvy)/(ry*(1+.07*math.sin(z*.26+x*.08+style)))
          d=planar+ny*ny
          if d<.99:
             put(x,y,z,7)
          elif .99<=d<1.09:
             v=rng.random()
             put(x,y,z,1 if v<.17 else 2 if v<.34 else 0)
    # Optional cavern side lobes: their openings overlap the main cavern.
    # Two branches per variant; each geometry differs with the seed.
    off=[(-11,13),(15,-14),(-14,-12),(12,15),(-13,13),(14,-16)][style]
    for side,(ox,oz) in enumerate([(off[0],off[1]),(-off[0]//2,-off[1]//2)]):
       cx=cvx+ox;cz=cvz+oz;cy=cvy+(2 if side else -2)
       arx=8+style%4;arz=9+(style+side)%3;ary=6+side*3
       for z in range(cz-arz-2,cz+arz+3):
        for x in range(cx-arx-2,cx+arx+3):
         for y in range(max(2,cy-ary-2),min(SZ[1]-2,cy+ary+3)):
          d=((x-cx)/arx)**2+((z-cz)/arz)**2+((y-cy)/ary)**2
          if d<1:put(x,y,z,7)
          elif 1<=d<1.13 and (x,y,z) not in vox:put(x,y,z,1 if rng.random()<.25 else 0)
    # Long steep entrance leading down to the cave. Centerline changes in X with Z.
    amp,freq,phase,amp2,freq2,phase2,xoff=CURVES[style]
    centers=[]
    for z in range(2,76):
      smooth=z/74
      xcenter=int(round(38+xoff+amp*(math.sin(z*freq+phase)-math.sin(phase))*smooth+amp2*math.sin(z*freq2+phase2)*smooth))
      ycenter=int(round(87-0.88*(z-2) - 1.2*math.sin(.08*z+style)))
      ycenter=min(SZ[1]-7,max(14,ycenter))
      centers.append((xcenter,ycenter,z))
      wid=4+int(z>21 and z<64 and style%2==1)
      for dx in range(-wid-2,wid+3):
       for dy in range(-5,7):
        x=xcenter+dx; y=ycenter+dy
        d=(dx/(wid+0.15))**2+((dy-0.5)/4.5)**2
        if d<=1:
          put(x,y,z,7)
        elif 1<d<1.38:
          # Irregular rock shell rather than slab floors in open air.
          if rng.random()<.64:put(x,y,z,1 if rng.random()<.22 else 0)
      # Staggered steps on a walkable steep descending route.
      for dx in (-2,-1,0,1,2):
        x=xcenter+dx
        put(x,ycenter-3,z, 4 if z%11 in (0,1,2) else 3 if (z+style)%7==0 else 0)
        for dy in range(-2,3):put(x,ycenter+dy,z,7)
      # Pillars and lights remain confined to the tunnel (no decorative ships etc).
      if z in (16,31,47,60):
        for sg in (-1,1):
          x=xcenter+sg*(wid-1)
          for dy in range(-2,4):put(x,ycenter+dy,z,1 if dy>0 else 3)
        put(xcenter,ycenter+4,z,5)
    # Carve a clear broad transition at cavern/tunnel overlap, so no false dead end.
    ex,ey,ez=centers[-1]
    for z in range(66,86):
      f=(z-66)/20
      cx=round(ex*(1-f)+cvx*f)
      cy=round(ey*(1-f)+(cvy+1)*f)
      radius=5+round(3*f)
      for x in range(cx-radius,cx+radius+1):
       for y in range(cy-5,cy+6):
        if ((x-cx)/radius)**2+((y-cy)/5)**2<1:put(x,y,z,7)
    # Local cavern floor ledges / rocks and cavern lighting away from the main tunnel.
    for i in range(13):
      x=cvx+rng.randint(-rx+6,rx-6)
      z=cvz+rng.randint(-rz+5,rz-5)
      ys=sorted(y for (xx,y,zz),v in vox.items() if xx==x and zz==z and v==7)
      if not ys:continue
      floor=ys[0]
      if floor<12 and rng.random()<.6:
       height=rng.randint(2,5)
       for yy in range(floor,floor+height):put(x,yy,z,1 if yy==floor else 2)
      elif floor<20 and rng.random()<.4:
       put(x,floor+1,z,6)
    raw=basic_template(SZ,palette,vox,dataversion=4435)
    return raw,{'size':list(SZ),'blocks':len(vox),'air_blocks':sum(v==7 for v in vox.values()),'cavern_radius':[rx,ry,rz],'entry_center':centers[0],'tunnel_end':centers[-1],'cavern_center':[cvx,cvy,cvz]}

def validate_template(raw):
    _,root=read_nbt(raw)
    sx,sy,sz=root['size'].value[1]
    names=[d['id'].value for d in root['palette'].value[1]]
    blocks=root['blocks'].value[1]
    assert len({tuple(b['pos'].value[1]) for b in blocks})==len(blocks)
    assert all(0<=b['pos'].value[1][0]<sx and 0<=b['pos'].value[1][1]<sy and 0<=b['pos'].value[1][2]<sz for b in blocks)
    air={tuple(b['pos'].value[1]) for b in blocks if names[b['state'].value]=='minecraft:air'}
    assert len(air)>=16000,len(air)
    start=max((x,y,z) for (x,y,z) in air if z<=5 and y>=78)
    seen={start};todo=[start]
    while todo:
      x,y,z=todo.pop()
      for other in ((x+1,y,z),(x-1,y,z),(x,y+1,z),(x,y-1,z),(x,y,z+1),(x,y,z-1)):
        if other in air and other not in seen:
          seen.add(other);todo.append(other)
    cave_connected=sum(1 for x,y,z in seen if 63<=z<=100 and 11<=y<=32)
    assert cave_connected>=4500,(len(seen),cave_connected)
    assert min(y for x,y,z in seen)<=10
    return {'air':len(air),'reachable_air':len(seen),'connected_cave_air':cave_connected,'bbox':list(root['size'].value[1])}

def build_four():
    generated={};geometry={};audits={}; modified_definitions={}
    for idx in range(1,7):
      raw,meta=make_tunnel(idx-1)
      result=validate_template(raw)
      name=f'data/thalvoryn/structure/underground/v67_cave_tunnel_{idx:02}.nbt'
      generated[name]=raw;geometry[name]=meta;audits[name]=result
      sid=f'data/thalvoryn/worldgen/structure/underground/v67_cave_tunnel_{idx:02}.json'
      modified_definitions[sid]=None
      print('GEOM',idx,meta,'AUDIT',result,flush=True)
    assert len({hashlib.sha256(v).hexdigest() for v in generated.values()})==6
    reports=[]
    for platform in ['RUST','JAVA']:
      for height in [1024,2032]:
        src=ROOT/f'Thalvoryn-{platform}-Nether{height}-V67-SPAWN-RECHECK-TEST.zip'
        dst=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-CAVE-GEOMETRY-TEST.zip'
        with zipfile.ZipFile(src) as zi, zipfile.ZipFile(dst,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True) as zo:
          assert zi.testzip() is None
          names=zi.namelist();assert len(names)==11323 and set(generated).issubset(names)
          for info in zi.infolist():
            path=info.filename
            data=generated[path] if path in generated else zi.read(path)
            if path in modified_definitions:
              doc=json.loads(data)
              assert doc.get('project_start_to_heightmap')=='OCEAN_FLOOR_WG'
              assert doc['start_height'].get('min_inclusive',{}).get('absolute')==-65
              assert doc['start_height'].get('max_inclusive',{}).get('absolute')==-62
              # The template top/mouth was y≈67; new top is y≈87.
              # Keep the entrance around heightmap+1 instead of creating a floating stair.
              doc['start_height']={'absolute':-85}
              data=(json.dumps(doc,indent=2,sort_keys=True,ensure_ascii=False)+'\n').encode()
            if path=='pack.mcmeta':
              d=json.loads(data);d['pack']['description']=f'Thalvoryn V67 {platform} Nether{height} CAVE-GEOMETRY TEST (private)';data=(json.dumps(d,indent=2,ensure_ascii=False)+'\n').encode()
            zo.writestr(info,data)
        reports.append({'platform':platform,'height':height,'filename':dst.name,'bytes':dst.stat().st_size,'sha256':hashlib.sha256(dst.read_bytes()).hexdigest()})
        print('BUILT',dst.name,flush=True)
    (OUT/'CAVE_GEOMETRY_MANIFEST.json').write_text(json.dumps({'status':'STATIC ONLY NOT GAME TESTED','geometry':geometry,'audit':audits,'modified_structure_definitions':sorted(modified_definitions),'start_height_offset':-85,'variants':reports},indent=2,ensure_ascii=False)+'\n')
if __name__=='__main__':build_four()