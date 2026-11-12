#!/usr/bin/env python3
"""Independent V71 ZIP/NBT/structure/registry verification; NOT an in-game test."""
from pathlib import Path
import glob,zipfile,json,collections,hashlib,sys
from nbt_codec_v67 import read_nbt
BASE=Path('/mnt/data/v68_work/v70_release_candidate');OUT=Path('/mnt/data/v71_work/release')
EXPECTED={'pack.mcmeta','data/thalvoryn/function/version.mcfunction','data/thalvoryn/function/check.mcfunction','data/thalvoryn/function/help.mcfunction','data/thalvoryn/function/horizon/status.mcfunction','data/nova_structures/worldgen/structure/witch_villa.json','V71_CHANGELOG_CZ.txt'}
UNDER=['explorify:black_spiral','hellish_trials:nether_trial','nova_structures:piglin_donjon','nova_structures:sealing_halls']

def path(s):return 'data/'+s.replace(':','/worldgen/structure/')+'.json'
def blocks(root):return {tuple(b['pos'].value[1]):b for b in root['blocks'].value[1]}
def main_connected(root):
    from collections import deque
    pal=root['palette'].value[1];ai=next(i for i,p in enumerate(pal) if p.get('Name',p.get('id')).value=='minecraft:air')
    vox={tuple(b['pos'].value[1]) for b in root['blocks'].value[1] if b['state'].value==ai}
    entrance=next((x,y,z) for z in range(2,10) for y in range(89,82,-1) for x in range(36,43) if (x,y,z) in vox)
    q=deque([entrance]);seen={entrance}
    while q:
        x,y,z=q.popleft()
        for a in ((x+1,y,z),(x-1,y,z),(x,y+1,z),(x,y-1,z),(x,y,z+1),(x,y,z-1)):
            if a not in seen and a in vox: seen.add(a);q.append(a)
    assert any(z>78 and 17<y<27 for x,y,z in seen),'Main chamber disconnected'
    assert any(x==0 and 50<z<110 for x,y,z in seen),'Left passage not connected'
    assert any(x==78 and 50<z<110 for x,y,z in seen),'Right passage not connected'
    return {'air_blocks':len(vox),'connected_air_blocks':len(seen),'entrance':entrance}

report={'status':'PASS_STATIC_ONLY_NOT_GAME_TESTED','variants':[],'known_limitations':['Natural cave intersection is not deterministic','Java cannot use Horizon-only slope checks','Game/TPS/FPS not measured']}
for platform in ('RUST','JAVA'):
 for height in (1024,2032):
    a=BASE/f'Thalvoryn-{platform}-Nether{height}-V70-PRIVATE-TEST.zip';b=OUT/f'Thalvoryn-{platform}-Nether{height}-V71-STATIC-TEST.zip'
    with zipfile.ZipFile(a) as old, zipfile.ZipFile(b) as new:
        assert old.testzip() is None and new.testzip() is None
        o=set(old.namelist());n=set(new.namelist());assert n-o=={'V71_CHANGELOG_CZ.txt'} and not o-n
        changed={s for s in o if old.getinfo(s).CRC!=new.getinfo(s).CRC or old.getinfo(s).file_size!=new.getinfo(s).file_size}
        expected=EXPECTED|{f'data/thalvoryn/structure/underground/v67_cave_tunnel_{i:02}.nbt' for i in range(1,7)}|{path(s) for s in UNDER}
        if platform=='RUST':
            for s in n:
                if '/worldgen/structure/' in s and s.endswith('.json') and s not in changed:
                    aobj=json.loads(old.read(s));bobj=json.loads(new.read(s))
                    if aobj!=bobj:raise AssertionError(('CRC mismatch check',s))
            expected |= {'data/nova_structures/worldgen/structure/witch_villa.json'}
            # Terrain guard edits validated by separate diff below.
        if platform=='JAVA':assert changed<=expected,(platform,changed-expected)
        else:
            assert all(s.endswith('.json') and '/worldgen/structure/' in s or s in expected for s in changed),changed-expected
        cav=[]
        for i in range(1,7):
            s=f'data/thalvoryn/structure/underground/v67_cave_tunnel_{i:02}.nbt';_,r0=read_nbt(old.read(s));_,r1=read_nbt(new.read(s));assert r0['size']==r1['size'] and r0['palette']==r1['palette'] and r0['entities']==r1['entities'];bb0=blocks(r0);bb1=blocks(r1);assert set(bb0)<=set(bb1)
            airstate=next(i for i,p in enumerate(r1['palette'].value[1]) if p.get('Name',p.get('id')).value=='minecraft:air')
            made=0
            for pos,bl in bb0.items():
                if bl!=bb1[pos]:
                    assert 'nbt' not in bl and bb1[pos]['state'].value==airstate,('Non-air change',s,pos)
                    made+=1
            assert made+len(set(bb1)-set(bb0))>200,(s,made,len(set(bb1)-set(bb0)))
            for pos in set(bb1)-set(bb0):assert bb1[pos]['state'].value==airstate
            link=main_connected(r1);link['old_to_air']=made;cav.append(link)
        for sid in UNDER:
            d=json.loads(new.read(path(sid)));h=d['start_height'];assert h['type']=='minecraft:uniform';assert h['min_inclusive']['absolute']==28;assert h['max_inclusive']['absolute']==height-160
            assert 'project_start_to_heightmap' not in json.loads(old.read(path(sid)))
        villa=json.loads(new.read(path('nova_structures:witch_villa')))
        assert villa['project_start_to_heightmap']=='WORLD_SURFACE_WG' and villa['start_height']=={'absolute':-2}
        for s in n:
            assert not s.startswith('data/stellarity/'),s
            if s.endswith('.json'):
                d=json.loads(new.read(s))
                if '/worldgen/structure/' in s and d.get('type') in ('minecraft:jigsaw','yungsapi:yung_jigsaw'):
                    reach=d.get('max_distance_from_center',80)
                    horizontal=reach.get('horizontal',80) if isinstance(reach,dict) else reach
                    assert isinstance(horizontal,int) and 1<=horizontal<=128,(s,horizontal)
                    assert 0<=d.get('size',0)<=20,s
        # All active local structure start pools exist; do not rely on mere ZIP CRC.
        ids=[];groupcount=0
        for s in n:
            if '/worldgen/structure_set/' not in s:continue
            d=json.loads(new.read(s));groupcount+=1
            for item in d.get('structures',[]):
                sid=item['structure'];ids.append(sid)
                sp=path(sid)
                if sp in n:
                    defn=json.loads(new.read(sp));pool=defn.get('start_pool')
                    if pool and not pool.startswith('minecraft:'):
                        p='data/'+pool.replace(':','/worldgen/template_pool/')+'.json'
                        assert p in n,(sp,p)
        assert len(ids)==len(set(ids)),('duplicate active structure ids',len(ids),len(set(ids)))
        report['variants'].append({'platform':platform,'nether_height':height,'zip_sha256':hashlib.sha256(b.read_bytes()).hexdigest(),'zip_entries':len(n),'changed_old_files':len(changed),'new_files':1,'structure_sets':groupcount,'active_structure_refs':len(ids),'tunnels':cav,'errors':[]})
        print('PASS',platform,height,'changed',len(changed),'entrance-links',len(cav),flush=True)
(OUT/'V71_INDEPENDENT_QA.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')