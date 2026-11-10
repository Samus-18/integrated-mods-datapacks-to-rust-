#!/usr/bin/env python3
"""V67 ecology pass, private test only. Rebuild four full datapacks with shared, deduped
cross-source placement groups. Never publish third-party licensed templates publicly.
No Java/Horizon gameplay runtime verification is claimed.
"""
from __future__ import annotations
from pathlib import Path
import zipfile, json, re, hashlib, collections, copy, os, sys, math
ROOT=Path(__file__).resolve().parent
SOURCE={}
for platform in ('JAVA','RUST'):
    for height in (1024,2032):
        SOURCE[(platform,height)]=next(ROOT.glob(f'Thalvoryn-{platform}-Nether{height}-V67-INTEGRATION-TEST-UNRELEASED.zip'))
OUT=ROOT/'V67_ECOLOGY_PRIVATE_TEST';OUT.mkdir(exist_ok=True)
DECISIONS={}
LOG={}
GROUPS={
 'ow_small': ('overworld',26,11,.77,'linear',None),
 'ow_settlements': ('overworld',39,16,.62,'triangular',('minecraft:villages',10)),
 'ow_rare': ('overworld',66,25,.48,'triangular',('thalvoryn:ecology/ow_settlements',12)),
 'ow_epic': ('overworld',112,44,.34,'triangular',('thalvoryn:ecology/ow_rare',16)),
 'ow_ocean': ('overworld',54,22,.57,'triangular',('minecraft:shipwrecks',8)),
 'ow_sky_islands': ('overworld',44,19,.64,'linear',None),
 'ow_airship': ('overworld',119,49,.32,'triangular',('thalvoryn:ecology/ow_sky_islands',16)),
 'ow_spawners': ('overworld',21,9,.34,'linear',('thalvoryn:ecology/ow_dungeons',7)),
 'ow_dungeons': ('overworld',48,19,.52,'triangular',('thalvoryn:ecology/ow_underground_epic',14)),
 'ow_underground_epic': ('overworld',99,38,.36,'triangular',None),
 'ow_cave_entrances': ('overworld',35,13,.60,'linear',('thalvoryn:ecology/ow_underground_epic',10)),
 'nether_minor': ('nether',30,12,.70,'linear',('minecraft:nether_complexes',8)),
 'nether_ruins': ('nether',44,19,.63,'triangular',('thalvoryn:ecology/nether_major',12)),
 'nether_major': ('nether',86,34,.39,'triangular',('minecraft:nether_complexes',14)),
 'nether_underground': ('nether',60,23,.45,'triangular',('thalvoryn:ecology/nether_major',14)),
 'end_islands': ('end',33,13,.65,'linear',None),
 'end_minor': ('end',48,20,.58,'triangular',('minecraft:end_cities',12)),
 'end_major': ('end',105,43,.37,'triangular',('minecraft:end_cities',16)),
}
# Some dimension-specific names/structure sources and tags disambiguate the native ID.
BOATS=re.compile(r'(^|[/_])(ships?(?:_|$)|shipwreck|pirate|frigate|galleon|vessel|airship|sky_caravan)(?:[/_]|$)',re.I)
RE_ST=re.compile(r'data/([^/]+)/worldgen/structure/(.+)\.json$')
RE_SS=re.compile(r'data/([^/]+)/worldgen/structure_set/(.+)\.json$')

def jdump(x): return (json.dumps(x,ensure_ascii=False,indent=2,sort_keys=True)+'\n').encode()
def id_of(path,rx):
    m=rx.fullmatch(path)
    return None if not m else m.group(1)+':'+m.group(2)
def choose_dim(sid,source_paths,defn):
    s=sid.lower();p=' '.join(source_paths).lower()
    if s.startswith('minecraft:'):return 'vanilla'
    if s.startswith('stellarity:'):return 'end'
    if s.startswith('hellish_trials:'):return 'nether'
    if s.startswith('underground_villages:'):return 'overworld'
    if s.startswith('thalvoryn:nether/') or s.startswith('trek:nether/') or s.startswith('nova_structures:nether_') or s.startswith('structures:nether_') or s.startswith('thunderfusion:nether/'):return 'nether'
    if s.startswith(('thalvoryn:end/','trek:end/','thunderfusion:end/')) or s.startswith(('nova_structures:end_', 'explorify:end_', 'structures:end_')):return 'end'
    if any(x in p for x in ('/nether/','/nether_')) and not any(x in p for x in ('/overworld','/end/')):return 'nether'
    if '/end/' in p or '/end_' in p:return 'end'
    biomes=defn.get('biomes',[]) if isinstance(defn,dict) else []
    if isinstance(biomes,str):biomes=[biomes]
    if any('nether' in x or 'crimson' in x or 'warped' in x or 'basalt_delta' in x for x in biomes):return 'nether'
    if any('end_highlands' in x or 'end_midlands' in x or 'end_barrens' in x for x in biomes):return 'end'
    return 'overworld'

def choose_cat(sid,orig,defn,dim):
    v=sid.lower(); refs=' '.join(orig).lower()
    if dim=='end':
        if any(x in v for x in ('island','void_rock','archipelago','floating_treasure','fossil','obsidian_spike','gateway')):return 'end_islands'
        if any(x in v for x in ('castle','citadel','palace','stronghold','monument','sanctuary','observatory','end_city','temple','chapel')):return 'end_major'
        return 'end_minor'
    if dim=='nether':
        if any(x in v for x in ('cave','underground','buried','crypt','dungeon','trial')):return 'nether_underground'
        if any(x in v for x in ('fortress','fort','castle','citadel','bastion','arena','temple','bridge','viaduct','monument','large','skybridge')):return 'nether_major'
        if any(x in v for x in ('ruin','outpost','waystation','cairn','remnant','camp','watch','pillar','house','small')):return 'nether_ruins'
        return 'nether_minor'
    if v=='thalvoryn:overworld/handmade/air_vessel' or 'air_vessel' in v:return 'ow_airship'
    if v.startswith('thalvoryn:sky/') or 'sky_isle' in v or 'floating_island' in v:return 'ow_sky_islands'
    if ('ships_' in v or 'shipwreck' in v or 'ship_pillager' in v or v.endswith('_trade') or 'ocean_' in v or 'conduit' in v or 'deep_ocean' in refs):return 'ow_ocean'
    if 'monster_room' in v or 'spawner' in v:return 'ow_spawners'
    if 'cave_tunnel' in v or 'entrance' in v or 'descent' in v:return 'ow_cave_entrances'
    if any(q in v for q in ('ancient_city','underground_village','buried_village','deep_city','spiral_hamlet','stronghold','mine_complex','trial_chamber')):return 'ow_underground_epic'
    if any(q in v for q in ('dungeon','cave','crypt','catacomb','underground','mineshaft','vault','monster_room')) or 'dungeon' in refs or 'small_cave' in refs:return 'ow_dungeons'
    if any(q in v for q in ('giant','palace','mansion','castle','citadel','mega','monument','very_rare','great_','ruin_town','large_fort')):return 'ow_epic'
    if any(q in v for q in ('fort','temple','tower','watchtower','rare/','bunker','shrine','pyramid','outpost','stronghold','arena','tomb')):return 'ow_rare'
    if any(q in v for q in ('village','hamlet','house','tavern','settlement','hut','farm','camp','base','residence','inn','trading','manor')) or '/medium' in refs:return 'ow_settlements'
    return 'ow_small'

def load_src(z):
    names=z.namelist()
    defs={id_of(n,RE_ST):json.loads(z.read(n)) for n in names if id_of(n,RE_ST)}
    sets={id_of(n,RE_SS):json.loads(z.read(n)) for n in names if id_of(n,RE_SS)}
    return defs,sets

def decide(defs,sets):
    references=collections.defaultdict(list); weight=collections.defaultdict(list)
    for source,d in sets.items():
        for s in d.get('structures',[]):
            sid=s['structure'];references[sid].append(source);weight[sid].append(s.get('weight',1))
    grouped=collections.defaultdict(list);block=[];other=[];ambig=[]
    for sid,d in sorted(defs.items()):
        if sid.startswith('minecraft:'):continue
        orig=references.get(sid,[])
        # Leave special concentric placement alone, to preserve underground rings.
        if orig and (all(sets[o]['placement']['type']=='minecraft:concentric_rings' for o in orig) or any(o.startswith('minecraft:') or o=='nova_structures:strongholds_bunker' for o in orig)):
            other.append(sid);continue
        dim=choose_dim(sid,orig,d)
        if dim not in ('overworld','nether','end'):ambig.append([sid,dim,orig]);continue
        if dim in ('nether','end') and (BOATS.search(sid) or 'ship' in sid.lower() or (dim=='end' and sid in ('nova_structures:end_lighthouse',))):
            block.append({'structure':sid,'dim':dim,'reason':'Inappropriate maritime/ship theme excluded from Nether/End'});continue
        cat=choose_cat(sid,orig,d,dim)
        if GROUPS[cat][0]!=dim:raise AssertionError((sid,dim,cat))
        # All structurally valid definitions get one owner; external junk is retained but never double-spawned.
        grouped[cat].append({'structure':sid,'weight':1 if 'very_rare/' in sid else min(4,max(1,int(round(sum(weight[sid])/len(weight[sid]))))) if weight[sid] else 1})
    return grouped,block,other,ambig,references

def build_group(cat,entries):
    d,spacing,sep,freq,spread,exclude=GROUPS[cat]
    if not entries:return None
    place={'type':'minecraft:random_spread','salt':40187000+list(GROUPS).index(cat)*1091,'spacing':spacing,'separation':sep,'spread_type':spread,'frequency':freq,'frequency_reduction_method':'default'}
    if exclude:place['exclusion_zone']={'other_set':exclude[0],'chunk_count':min(16,exclude[1])}
    return {'structures':sorted(entries,key=lambda x:x['structure']),'placement':place}

def alter_def(sid,defn,platform,height):
    o=copy.deepcopy(defn);dim=choose_dim(sid,[],o)
    # Keep native design unless we can state what the changes accomplish.
    if sid.startswith('thalvoryn:sky/v67_sky_isle_'):
        o['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':145},'max_inclusive':{'absolute':245}}
    elif sid=='thalvoryn:overworld/handmade/air_vessel':
        o['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':180},'max_inclusive':{'absolute':245}}
    elif sid.startswith('thalvoryn:underground/v67_cave_tunnel_'):
        # Template is 73 tall; generation starts in the mountain body so the top sticks through a slope.
        # This guarantees its BUILT cavern, not a connection to an arbitrary procedural cave.
        o['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':-72},'max_inclusive':{'absolute':-53}}
        o['terrain_adaptation']='beard_box'
        if platform=='RUST':
            o.setdefault('horizon:start_checks',{}).update({'terrain_radius':1,'terrain_range':42})
    elif sid.startswith('trek:monster_room/'):
        # Do not adjust authored depth; these rooms are rate-limited at the set level.
        pass
    elif (sid.startswith('thalvoryn:nether/') or sid.startswith('nova_structures:nether_') or sid=='structures:nether_arena') and 'ships_' not in sid and 'frigate' not in sid and o.get('type')=='minecraft:jigsaw':
        # Select actual Nether terrain surface, not fixed Vanilla 60-90 heights.
        # Retain deliberate height offsets for existing terrain-following surface structures.
        if 'project_start_to_heightmap' not in o and not any(k in sid for k in ('buried','underground','cave')):
            o['project_start_to_heightmap']='WORLD_SURFACE_WG'
            o['start_height']={'absolute':-6}
        if platform=='RUST' and 'project_start_to_heightmap' in o:
            ch=o.setdefault('horizon:start_checks',{})
            ch.setdefault('liquid','top')
            ch.setdefault('terrain_radius',1)
            ch.setdefault('terrain_range',36)
            ch['min_y']=max(60,int(ch.get('min_y',60)))
            ch['max_y']=min(height-100,int(ch.get('max_y',height-100)))
        elif platform=='JAVA':
            o.pop('horizon:start_checks',None)
    return o

def apply(z,platform,height,shared):
    defs,sets=load_src(z)
    grouped,excluded,other,ambig,references=shared
    skip=set();mods={}
    preserved=[]
    for n in z.namelist():
        sid=id_of(n,RE_SS)
        if sid:
            # Vanilla structure sets have their own native spacing semantics; preserve except End City.
            if sid.startswith('minecraft:') or sid=='nova_structures:strongholds_bunker':
                preserved.append(sid)
                if sid=='minecraft:end_cities':
                    val=copy.deepcopy(sets[sid]);val['placement'].update({'spacing':77,'separation':31,'frequency':.42})
                    mods[n]=jdump(val)
            else:skip.add(n)
        did=id_of(n,RE_ST)
        if did:
            nv=alter_def(did,defs[did],platform,height)
            if nv!=defs[did]:mods[n]=jdump(nv)
    for cat,entries in grouped.items():
        g=build_group(cat,entries)
        if not g:continue
        mods[f'data/thalvoryn/worldgen/structure_set/ecology/{cat}.json']=jdump(g)
    # The source pack incorrectly had exclusion_zone chunk_count=18 in one End set.
    # Original set is removed; all new exclusion_zone fields are restricted to [1,16].
    output=OUT/f'Thalvoryn-{platform}-Nether{height}-V67-ECOLOGY-TEST-UNRELEASED.zip'
    with zipfile.ZipFile(output,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True) as out:
        for n in z.namelist():
            if n in skip:continue
            b=mods.pop(n,None)
            if b is None:b=z.read(n)
            out.writestr(n,b)
        for n,b in sorted(mods.items()):out.writestr(n,b)
    with zipfile.ZipFile(output) as result:
        assert result.testzip() is None
        names=result.namelist();assert len(names)==len(set(names))
        outsets={id_of(n,RE_SS):json.loads(result.read(n)) for n in names if id_of(n,RE_SS)}
        seen=collections.Counter(x['structure'] for s in outsets.values() for x in s.get('structures',[]) if not x['structure'].startswith('minecraft:'))
        assert all(n==1 for n in seen.values()),[(k,v) for k,v in seen.items() if v!=1][:30]
        assert all(not ('ship' in k.lower() or BOATS.search(k)) for k in seen if choose_dim(k,references.get(k,[]),defs.get(k,{})) in ('end','nether'))
        assert all(1<=s['placement'].get('exclusion_zone',{}).get('chunk_count',1)<=16 for s in outsets.values())
        assert len(seen)==sum(map(len,grouped.values()))+len(other) # any non-Minecraft concentric rings
        size=output.stat().st_size
        return {'path':str(output),'sha256':hashlib.file_digest(open(output,'rb'),'sha256').hexdigest(),'size_bytes':size,'structure_sets':len(outsets),'unique_nonvanilla_active':len(seen),'category_counts':{cat:len(x) for cat,x in grouped.items()},'removed_legacy_sets':len(skip),'preserved_structure_sets':preserved}

def main():
    with zipfile.ZipFile(SOURCE[('RUST',1024)]) as zz:
        defs,sets=load_src(zz)
    shared=decide(defs,sets)
    grouped,excluded,other,ambig,references=shared
    print('categories:',{k:len(v) for k,v in sorted(grouped.items())})
    print('excluded ships:',len(excluded),'other concentric:',len(other),'ambiguous:',len(ambig))
    assert not ambig,ambig
    assert len(excluded)>30,excluded
    assert grouped['ow_spawners'] and grouped['end_major'] and grouped['nether_major']
    summary={'status':'STATIC_ONLY_PRIVATE_GAME_TEST_REQUIRED','group_parameters':{k:{'dimension':v[0],'spacing_chunks':v[1],'separation_chunks':v[2],'frequency':v[3],'exclusion':v[5]} for k,v in GROUPS.items()},'excluded_nonvanilla_ships':excluded,'preserved_special_ring_structures':other,'category_structures':{k:[x['structure'] for x in v] for k,v in grouped.items()},'inputs':{str(k):str(v) for k,v in SOURCE.items()},'variants':{}}
    for key,src in SOURCE.items():
        with zipfile.ZipFile(src) as zin:summary['variants'][f'{key[0]}_{key[1]}']=apply(zin,*key,shared)
    (OUT/'V67_ECOLOGY_MANIFEST.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
    print('PASS',[(k,v['size_bytes']) for k,v in summary['variants'].items()]);return summary
if __name__=='__main__':main()