#!/usr/bin/env python3
"""Thalvoryn V67 conservative terrain / ocean / loot correction.

Reproducible private transform of last V67 INTEGRITY packages.
Does NOT claim Java or Horizon runtime-tested placement or natural cave access.
"""
from __future__ import annotations
from pathlib import Path
import zipfile, gzip, json, hashlib, struct, collections, copy, time, math, os
from nbt_codec_v67 import read_nbt

ROOT=Path('/mnt/data/thalvoryn_v67_hardfix/V67_INTEGRITY_TEST')
OUT=Path('/mnt/data/thalvoryn_v67_terrain_loot/PRIVATE_TEST')
OUT.mkdir(parents=True,exist_ok=True)
J=lambda ob:(json.dumps(ob,indent=2,ensure_ascii=False,sort_keys=True)+'\n').encode()
DEEP_OCEANS=['minecraft:deep_ocean','minecraft:deep_lukewarm_ocean','minecraft:deep_cold_ocean']

# Default Minecraft 26.3 loot schema uses `modifier` (singular) instead of functions.
def entry(item,w,lo=1,hi=None):
    e={'type':'minecraft:item','name':'minecraft:'+item,'weight':w}
    if hi is not None:
        e['modifier']={'type':'minecraft:set_count','count':{'type':'minecraft:uniform','min':lo,'max':hi}}
    return e

def revised_loot(table):
    # Keep simple supplies ordinary, not OP or one predetermined item.
    if table=='farmer':
        a=[entry('bread',15,1,3),entry('carrot',12,1,5),entry('potato',12,1,5),entry('wheat',14,2,5),entry('wheat_seeds',12,2,5),entry('bone_meal',9,1,3),entry('beetroot',7,1,4),entry('apple',7,1,2),entry('emerald',1)]
        b=[entry('bread',6,1,2),entry('pumpkin_pie',2),entry('cooked_chicken',1)]
    elif table=='wood_and_stick':
        a=[entry('stick',24,2,6),entry('oak_planks',12,2,5),entry('spruce_planks',9,2,5),entry('charcoal',8,1,3),entry('string',6,1,3),entry('torch',8,1,4)]
        b=[entry('wooden_axe',5),entry('wooden_pickaxe',5),entry('wooden_shovel',5),{'type':'minecraft:empty','weight':12}]
    elif table=='stick_and_emrerald':
        a=[entry('stick',18,2,6),entry('torch',13,1,4),entry('iron_nugget',10,1,3),entry('coal',12,1,3),entry('bread',11,1,2),entry('flint',7,1,3)]
        b=[entry('emerald',2),entry('iron_ingot',4),{'type':'minecraft:empty','weight':24}]
    else:raise ValueError(table)
    return {'type':'minecraft:chest','pools':[{'rolls':{'type':'minecraft:uniform','min':2,'max':4},'entries':a},{'rolls':1,'entries':b}]}

def seed_remove_and_loot_replace(buf,name,log):
    zipped=buf[:2]==b'\x1f\x8b';raw=gzip.decompress(buf) if zipped else buf
    orig=raw
    # NBT TAG_Long 4, name=LootTableSeed. Seed is a fixed 8-byte value.
    needle=b'\x04\x00\x0dLootTableSeed';found=0
    while True:
        pos=raw.find(needle)
        if pos<0:break
        if len(raw)<pos+len(needle)+8:raise ValueError('truncated seed '+name)
        raw=raw[:pos]+raw[pos+len(needle)+8:];found+=1
    if found:log['seed_removed']+=found;log['seed_files']+=1
    # The original generic loot is used across hundreds of unrelated templates;
    # use matching existing 26.3 Thalvoryn tables instead, by template category.
    old='thalvoryn:chests/rc37/overworld/generic_uncommon_plains'
    if name.startswith('data/thalvoryn/structure/overworld/') and old.encode() in raw:
        if '/ships_' in name:
            ids=['ship_uncommon_ocean','ship_rare_ocean','storage_common_ocean','storage_uncommon_ocean']
        elif '/buildings_' in name:
            ids=['settlement_common_plains','settlement_uncommon_plains','storage_common_plains','generic_common_plains','ruin_uncommon_plains']
        elif 'cave' in name or 'underground' in name:
            ids=['cave_common_dripstone','cave_rare_dripstone','storage_common_deep_dark']
        else:
            ids=['generic_common_plains','generic_uncommon_plains','storage_common_plains','ruin_uncommon_plains']
        # Different barrels/chests in a template get different types of loot.
        # Need actual binary encoding of NBT string: TAG_String + named LootTable.
        prefix=b'\x08\x00\x09LootTable';offset=0;newraw=bytearray();changes=0
        while True:
            ix=raw.find(prefix,offset)
            if ix<0:break
            npos=ix+len(prefix)
            if npos+2>len(raw):raise ValueError('truncated LootTable '+name)
            length=struct.unpack_from('>H',raw,npos)[0]
            end=npos+2+length
            value=raw[npos+2:end].decode('utf8') if length<400 else ''
            if value==old:
                key=hashlib.sha256((name+':'+str(changes)).encode()).digest()
                id=ids[int.from_bytes(key[:2],'little')%len(ids)]
                replacement=('thalvoryn:chests/rc37/overworld/'+id).encode()
                newraw.extend(raw[offset:npos]);newraw.extend(struct.pack('>H',len(replacement)));newraw.extend(replacement)
                offset=end;changes+=1
            else:
                # skip this table without changing contents
                offset=end
        if changes:
            newraw.extend(raw[offset:]);raw=bytes(newraw);log['generic_remapped']+=changes;log['remap_files']+=1
    if raw==orig:return buf
    # Validation: parse every altered NBT to ensure byte slicing didn't corrupt the structure.
    _, root=read_nbt(raw)
    if not root.get('blocks'):raise ValueError('missing root blocks '+name)
    return gzip.compress(raw,compresslevel=6,mtime=0) if zipped else raw

def classify_land_path(path):
    s=path.lower()
    if any(w in s for w in ['/nether/','/end/','/sky/','/ocean/','/ship','air_vessel','flying','underground','/cave/','/cave_','/dungeon','/buried','/floating','/reef','/conduit']):return False
    return True

def patch_structure(path,source,platform,active_sets,active_ow_ids,log):
    # All metadata is preserved apart from narrow terrain/ship safeguards.
    if not path.endswith('.json') or '/worldgen/structure/' not in path or '/tags/' in path:return None
    ns=path.split('/')[1]
    if ns=='minecraft':return None # never modify vanilla structure definitions
    id=ns+':'+path.split('/worldgen/structure/',1)[1][:-5]
    if id not in active_sets:return None
    d=json.loads(source)
    if d.get('type')!='minecraft:jigsaw':return None
    orig=copy.deepcopy(d)
    # Imported ships: ocean-only away from coasts and ice.
    if path.startswith('data/thalvoryn/worldgen/structure/overworld/imported/ships_'):
        d['biomes']=DEEP_OCEANS
        d['project_start_to_heightmap']=None
        # 63 = sea level in Overworld noise settings, adjusted hull submersion.
        # Template-dependent vertical profile is measured using first pool NBT.
        ship_y=SHIP_HEIGHTS.get(id,57)
        d['start_height']={'absolute':ship_y}
        d['terrain_adaptation']='none'
        if platform=='RUST':
            check=d.setdefault('horizon:start_checks',{})
            check['biome_radius']=2 # reject coastal/mixed biome columns (5x5 chunks)
            check.pop('liquid',None) # Ocean boats MUST allow liquid; liquid=top would reject water!
        else:d.pop('horizon:start_checks',None)
        log['ocean_ships_adjusted']+=1
        return J(d) if d!=orig else None
    # Existing ocean trader ships: avoid ice and shore; keep provided template height.
    if '/worldgen/structure/overworld/medium/' in path and ('trade' in path or 'ship_pillager' in path):
        d['biomes']=DEEP_OCEANS
        if platform=='RUST':
            c=d.setdefault('horizon:start_checks',{})
            c['biome_radius']=max(2,c.get('biome_radius',0))
            c.pop('liquid',None) # water is intended for boat locations
        else:d.pop('horizon:start_checks',None)
        log['ocean_traders_adjusted']+=1
        return J(d) if d!=orig else None
    # Keep End and Nether restrictions; they're intentionally distinct.
    if id not in active_ow_ids:return None
    if not classify_land_path(path):return None
    if not any('/'+w in path.lower() for w in ('overworld/', 'village/', 'settlement/', 'town/', 'structure/')):return None
    proj=d.get('project_start_to_heightmap')
    if proj not in ('WORLD_SURFACE_WG','MOTION_BLOCKING_NO_LEAVES'):return None
    name=path.lower()
    key_words=('village','settlement','castle','fort','hamlet','citadel','outpost','watchtower','tower','hut','house','ruin','sanctuary','temple')
    is_large=d.get('size',1)>=4 or any(w in name for w in key_words)
    if not is_large:return None
    # Heightmap deliberately ignores tree canopy; fixes high floating forest starts.
    d['project_start_to_heightmap']='MOTION_BLOCKING_NO_LEAVES'
    if d.get('terrain_adaptation') in (None,'none','beard_thin'):
        d['terrain_adaptation']='beard_box' if d.get('size',1)>=4 else 'beard_thin'
    if platform=='RUST':
        c=d.setdefault('horizon:start_checks',{})
        c['terrain_radius']=max(2,c.get('terrain_radius',0))
        c['terrain_range']=min(14,c.get('terrain_range',14))
        c['liquid']='top'
    else:d.pop('horizon:start_checks',None)
    if d!=orig:log['surface_settlements_safeguarded']+=1
    return J(d) if d!=orig else None

def find_ship_heights(z):
    result={};setpath='data/thalvoryn/worldgen/structure_set/ecology/ow_ocean.json'
    defs=json.loads(z.read(setpath))['structures']
    for v in defs:
        id=v['structure']
        if not id.startswith('thalvoryn:overworld/imported/ships_'):continue
        fname='data/thalvoryn/structure/overworld/imported/'+id.rsplit('/',1)[-1]+'.nbt'
        if fname not in z.namelist():continue
        _,root=read_nbt(z.read(fname)); palette=root['palette'].value[1]
        names=[e.get('Name',e.get('id')).value for e in palette]
        y=collections.Counter()
        for block in root['blocks'].value[1]:
            state=names[block['state'].value]
            if state not in ('minecraft:air','minecraft:structure_void','minecraft:water'):
                ly=block['pos'].value[1][1]
                if 1<=ly<=14:y[ly]+=1
        if y:
            peak=max(y,key=lambda ky:y[ky]); # first broad deck/hull density peak
            start=min(59,max(49,65-peak))
        else:start=57
        result[id]=start
    return result

SHIP_HEIGHTS={}
def patch_variant(platform,height):
    src=next(ROOT.glob(f'Thalvoryn-{platform}-Nether{height}-V67-INTEGRITY-REPAIR-TEST.zip'))
    dst=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-GROUND-LOOT-TEST.zip'
    log=collections.Counter();modified=[]
    with zipfile.ZipFile(src) as zin:
        global SHIP_HEIGHTS
        SHIP_HEIGHTS=find_ship_heights(zin)
        setpaths=[n for n in zin.namelist() if '/worldgen/structure_set/' in n and n.endswith('.json')]
        active_sets=set();active_ow_ids=set()
        for n in setpaths:
            js=json.loads(zin.read(n));ids={e['structure'] for e in js.get('structures',[])}
            active_sets.update(ids)
            if '/ecology/ow_' in n or n in ('data/minecraft/worldgen/structure_set/villages.json','data/minecraft/worldgen/structure_set/pillager_outposts.json'):active_ow_ids.update(ids)
        # Existing sea level in noise setting must match ship offsets.
        ov_settings=json.loads(zin.read('data/thunderfusion/worldgen/noise_settings/overworld.json'))
        assert ov_settings['sea_level']==63,'sea level changed'
        patchable_loot={f'data/structuresplus/loot_table/{t}.json':J(revised_loot(t)) for t in ('farmer','wood_and_stick','stick_and_emrerald')}
        with zipfile.ZipFile(dst,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True) as zout:
            for ii,n in enumerate(zin.namelist()):
                if ii%1500==0: print('  item',platform,height,ii,n,flush=True)
                data=zin.read(n);upd=None
                if n in patchable_loot:
                    upd=patchable_loot[n];log['improved_sparse_loot_tables']+=1
                elif n.endswith('.nbt'):
                    # Only parse structures whose serialized data actually changes.
                    raw_for_scan=gzip.decompress(data) if data[:2]==b'\x1f\x8b' else data
                    if (b'LootTableSeed' in raw_for_scan or (n.startswith('data/thalvoryn/structure/overworld/') and b'generic_uncommon_plains' in raw_for_scan)):
                        changed=seed_remove_and_loot_replace(data,n,log)
                        if changed!=data:upd=changed
                elif '/worldgen/structure/' in n and n.endswith('.json') and '/tags/' not in n:
                    upd=patch_structure(n,data,platform,active_sets,active_ow_ids,log)
                elif n=='data/thalvoryn/worldgen/structure_set/ecology/ow_cave_entrances.json':
                    d=json.loads(data)
                    d['placement']['spacing']=30
                    d['placement']['separation']=12
                    d['placement']['frequency']=0.63
                    upd=J(d);log['tunnel_frequency_adjusted']+=1
                # preserve exact members names/metadata ordering, no extra files.
                zout.writestr(n,upd if upd is not None else data)
                if upd is not None and upd!=data:modified.append(n)
        with zipfile.ZipFile(dst) as check:
            assert check.testzip() is None
    log['modified_file_count']=len(modified)
    return {'platform':platform,'height':height,'source':src.name,'zip':dst.name,'sha256':hashlib.sha256(dst.read_bytes()).hexdigest(),'bytes':dst.stat().st_size,'changes':dict(log),'ship_y_offsets':SHIP_HEIGHTS,'modified_file_sample':modified[:25]}

if __name__=='__main__':
    results=[];t=time.time()
    for platform in ('JAVA','RUST'):
        for height in (1024,2032):
            r=patch_variant(platform,height);print('built',r['zip'],r['changes'], 's',round(time.time()-t,1),flush=True);results.append(r)
    (OUT/'build_manifest.json').write_bytes(J({'status':'TEST BUILD ONLY – GAMEPLAY NOT VERIFIED','variants':results}))