#!/usr/bin/env python3
"""V71 targeted data changes; NO claim of in-game testing or guaranteed natural-cave link."""
from pathlib import Path
import math, json, zipfile, hashlib, collections, sys
from nbt_codec_v67 import read_nbt,write_nbt,t

ROOT=Path('/mnt/data/v71_work')
SRC=Path('/mnt/data/v68_work/v70_release_candidate')
OUT=ROOT/'release';OUT.mkdir(exist_ok=True)
VARIANTS=[('RUST',1024),('RUST',2032),('JAVA',1024),('JAVA',2032)]
NETHER_SUBSURFACE=['explorify:black_spiral','hellish_trials:nether_trial','nova_structures:piglin_donjon','nova_structures:sealing_halls']
NETHER_LARGE=['nova_structures:nether_port','nova_structures:nether_skeleton_camp_fort','nova_structures:nether_skeleton_tower_fort','structures:nether_arena','thalvoryn:nether/cinder_viaduct','thalvoryn:nether/skybridge_ruin']
OMIT_GROUND={'trek:overworld/medium/orbital','trek:overworld/medium/underwater_sword','trek:overworld/medium/offshore_platform_pillager','nova_structures:toxic_lair','nova_structures:witch_villa'}

def jbytes(v):return (json.dumps(v,indent=2,sort_keys=True,ensure_ascii=False)+'\n').encode('utf-8')
def sdpath(s):return 'data/'+s.replace(':','/worldgen/structure/')+'.json'
def center_tag(root,name):return root[name].value

def sidepassage(i,blob):
    nm,root=read_nbt(blob)
    size=root['size'].value[1]
    assert size==[79,93,114],(i,size)
    pals=root['palette'].value[1]
    air_state=next(k for k,el in enumerate(pals) if el.get('Name',el.get('id')).value=='minecraft:air')
    blocks=root['blocks'].value[1]
    coords={tuple(b['pos'].value[1]):b for b in blocks}
    original_blocks=len(blocks)
    changed=0; added=0
    old_tiles=[(tuple(b['pos'].value[1]),repr(b.get('nbt'))) for b in blocks if b.get('nbt')]
    # Opposite horizontal branches emerge from already-connected underground chamber.
    # They carve to the edge of the saved template. Beyond that, natural cave
    # adjacency depends on the generated seed and is NOT guaranteed.
    ends=[]
    for direction in ('left','right'):
        rng=range(0,31) if direction=='left' else range(49,79)
        z0=80+i if direction=='left' else 94-i
        off=0.35*i
        last=None
        for x in rng:
            center_z=round(z0+2.0*math.sin(x*.22+off))
            center_y=19+round(1.5*math.cos(x*.18+i))
            # 5-7-block-diameter curving tunnel. Space is intentionally limited
            # to keep additional block placement affordable.
            r=2.5 + (0.3 if i%2==0 else 0)
            for y in range(center_y-3,center_y+4):
                for z in range(center_z-3,center_z+4):
                    if not (0<=y<size[1] and 0<=z<size[2]):continue
                    if (y-center_y)**2+(z-center_z)**2>r*r:continue
                    p=(x,y,z)
                    if p in coords:
                        block=coords[p]
                        if block.get('nbt'):continue # never erase container/entities
                        if block['state'].value!=air_state:
                            block['state']=t(3,air_state);changed+=1
                    else:
                        block={'pos':t(9,(3,list(p))),'state':t(3,air_state)}
                        coords[p]=block;added+=1
            last=(x,center_y,center_z)
        ends.append(last)
    out=list(coords.values())
    out.sort(key=lambda b:tuple(b['pos'].value[1]))
    root['blocks']=t(9,(10,out))
    newtiles=[(tuple(b['pos'].value[1]),repr(b.get('nbt'))) for b in out if b.get('nbt')]
    assert old_tiles==newtiles or sorted(old_tiles)==sorted(newtiles)
    assert root['size'].value[1]==size and len(out)>=original_blocks
    return write_nbt(nm,root),{'variant':i,'original_blocks':original_blocks,'final_blocks':len(out),'solid_to_air':changed,'added_air':added,'ends':ends}

def patch(plan,platform,height):
    edits={};changes={'cave_geometry':[],'nether_height':[],'slope_guards':[],'nether_guards':[],'end_guards':[],'ground_anchor':[]}
    with zipfile.ZipFile(plan,'r') as zi:
        idx=set(zi.namelist())
        allsets={}
        for p in sorted(n for n in idx if '/worldgen/structure_set/' in n):
            d=json.loads(zi.read(p))
            for v in d.get('structures',[]):allsets[v['structure']]=p
        # Reuse same NBT template across platforms and variant heights.
        for i in range(1,7):
            p=f'data/thalvoryn/structure/underground/v67_cave_tunnel_{i:02}.nbt'
            b,meta=sidepassage(i,zi.read(p));edits[p]=b;changes['cave_geometry'].append(meta)
        # Four historically fixed Y=10-34 nether dungeons can now begin at
        # multiple elevations, while a safe buffer avoids top/bottom clipping.
        for sid in NETHER_SUBSURFACE:
            p=sdpath(sid);d=json.loads(zi.read(p)); old=d.get('start_height')
            assert old and 'absolute' in old,(sid,old)
            assert d.get('project_start_to_heightmap') is None
            d['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':28},'max_inclusive':{'absolute':height-160}}
            edits[p]=jbytes(d);changes['nether_height'].append({'id':sid,'from':old,'to':d['start_height']})
        # Surface witch villa was at fixed world Y63 and could appear suspended
        # when terrain rises or be buried when terrain falls. Not a water structure.
        sid='nova_structures:witch_villa';p=sdpath(sid);d=json.loads(zi.read(p))
        assert d['start_height']=={'absolute':63} and 'project_start_to_heightmap' not in d
        d['project_start_to_heightmap']='WORLD_SURFACE_WG';d['start_height']={'absolute':-2}
        if platform=='RUST':d['horizon:start_checks']={'terrain_radius':2,'terrain_range':25,'liquid':'top'}
        edits[p]=jbytes(d);changes['ground_anchor'].append(sid)
        # Rust-only checks reject conspicuously bad locations for SOME small
        # land structures. Do not impose these on sky/sea or deep structures.
        if platform=='RUST':
            for sid,source in sorted(allsets.items()):
                if not source.startswith('data/thalvoryn/worldgen/structure_set/ecology/ow_'):continue
                group=source.rsplit('/',1)[-1]
                if group not in ('ow_surface_dungeons.json','ow_settlements.json','ow_rare.json'):continue
                if sid in OMIT_GROUND or sid.startswith('thalvoryn:autumn/'):continue
                p=sdpath(sid);d=json.loads(zi.read(p))
                if p in edits or d.get('horizon:start_checks') is not None:continue
                if d.get('project_start_to_heightmap') not in ('WORLD_SURFACE_WG','MOTION_BLOCKING_NO_LEAVES','OCEAN_FLOOR_WG'):continue
                if d.get('step')!='surface_structures':continue
                if 'underwater' in sid or 'coast' in sid or 'ocean' in sid or 'island' in sid:continue
                # Conservative cut-off, no full-biome scan, keep runtime cost bounded.
                r=2
                slope=22 if group=='ow_settlements.json' else 25
                d['horizon:start_checks']={'terrain_radius':r,'terrain_range':slope,'liquid':'top'}
                edits[p]=jbytes(d);changes['slope_guards'].append(sid)
            # Major Nether construction has terrain checks within just 1 chunk;
            # extend to 2 chunks rather than forcing an arbitrary height.
            for sid in NETHER_LARGE:
                p=sdpath(sid);d=json.loads(zi.read(p));check=d['horizon:start_checks']
                assert d.get('project_start_to_heightmap')=='WORLD_SURFACE_WG'
                assert check['terrain_radius']==1
                check['terrain_radius']=2;check['terrain_range']=40
                edits[p]=jbytes(d);changes['nether_guards'].append(sid)
            # Large End land sites require a reasonably continuous island.
            for sid in ('thunderfusion:end/starlight_observatory','thunderfusion:end/void_citadel'):
                p=sdpath(sid);d=json.loads(zi.read(p));check=d['horizon:start_checks']
                assert d.get('project_start_to_heightmap')=='WORLD_SURFACE_WG'
                check['terrain_radius']=2;check['terrain_range']=32
                edits[p]=jbytes(d);changes['end_guards'].append(sid)
        for p in ('pack.mcmeta','data/thalvoryn/function/version.mcfunction','data/thalvoryn/function/check.mcfunction','data/thalvoryn/function/help.mcfunction','data/thalvoryn/function/horizon/status.mcfunction'):
            assert p in idx,p
            txt=zi.read(p).decode('utf-8')
            assert 'V70' in txt,p
            edits[p]=txt.replace('V70','V71').encode('utf-8')
        edits['V71_CHANGELOG_CZ.txt']=(f'Thalvoryn V71 {platform} Nether{height}, pouze kontrolovana DATA beta, ne herni test.\n'
         'Sest tunelu dostalo po dvou postrannich propojenych chodbach az na okraj NBT. NENI garantovano spojeni s prirodni jeskyni.\n'
         'Ctyri Nether underground dungeony maji nahodne Y napric rozsahem dimenze.\n'
         'Bazenna vila byla ukotvena k povrchu. Rust: filtrovani nevhodneho svahu u dalsich pozemnich staveb,\n'
         'Nether major staveb a dvou velkych End staveb. U Java nelze napodobit Horizon-only filtry.\n'
         'FPS/TPS a skutecne worldgen kolize nebyly herne otestovany.\n').encode('utf-8')
        assert all(p in idx or p=='V71_CHANGELOG_CZ.txt' for p in edits)
        dst=OUT/f'Thalvoryn-{platform}-Nether{height}-V71-STATIC-TEST.zip'
        with zipfile.ZipFile(dst,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6) as zo:
            for info in zi.infolist():
                b=edits.pop(info.filename,None)
                if b is None:b=zi.read(info.filename)
                zo.writestr(info,b)
            for p,b in sorted(edits.items()):zo.writestr(p,b,compress_type=zipfile.ZIP_DEFLATED)
        return {'platform':platform,'height':height,'target':str(dst),'sha256':hashlib.sha256(dst.read_bytes()).hexdigest(),'changes':changes}

if __name__=='__main__':
    manifest={'release':'V71','kind':'STATIC TEST ONLY','not_tested':['Minecraft game runtime','Horizon TPS/FPS','Natural cave adjacency','real terrain generation in seed'],'variants':[]}
    for platform,height in VARIANTS:
        inp=SRC/f'Thalvoryn-{platform}-Nether{height}-V70-PRIVATE-TEST.zip'
        row=patch(inp,platform,height);manifest['variants'].append(row)
        print('BUILT',platform,height,'slope',len(row['changes']['slope_guards']),'cave air',sum(v['solid_to_air'] for v in row['changes']['cave_geometry']),flush=True)
    (OUT/'V71_BUILD_MANIFEST.json').write_bytes(jbytes(manifest))