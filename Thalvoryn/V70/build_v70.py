#!/usr/bin/env python3
"""Thalvoryn V70 private DATA-ONLY build, from independently statically verified V69.
Does NOT pretend to run Minecraft or Horizon. Uses only user-supplied archives.
"""
from pathlib import Path
import zipfile, json, math, hashlib, collections
from nbt_codec_v67 import read_nbt
ROOT=Path('/mnt/data/v68_work');SRC=ROOT/'v69_release_candidate';DST=ROOT/'v70_release_candidate';DST.mkdir(exist_ok=True)
NAMES=[('RUST',1024),('RUST',2032),('JAVA',1024),('JAVA',2032)]
SURFACE={'nova_structures:illager_camp','nova_structures:remnant_bunny_base','trek:overworld/medium/claw_pillager','trek:overworld/medium/oasis','trek:overworld/medium/wild_stone','trek:overworld/medium/small_pyramide'}
BOATS_SET='data/thalvoryn/worldgen/structure_set/ecology/ow_ocean.json'
DUNGEON_SET='data/thalvoryn/worldgen/structure_set/ecology/ow_dungeons.json'
SURF_DUNGEON_SET='data/thalvoryn/worldgen/structure_set/ecology/ow_surface_dungeons.json'

def jsonbytes(v):return (json.dumps(v,sort_keys=True,indent=2,ensure_ascii=False)+'\n').encode('utf-8')
def structure_path(id):return 'data/'+id.replace(':','/worldgen/structure/')+'.json'
def template_path(id):return 'data/'+id.replace(':','/structure/')+'.nbt'
def build(platform,height):
    src=SRC/f'Thalvoryn-{platform}-Nether{height}-V69-PRIVATE-TEST.zip'
    target=DST/f'Thalvoryn-{platform}-Nether{height}-V70-PRIVATE-TEST.zip'
    with zipfile.ZipFile(src) as zin:
      members=zin.namelist();members_set=set(members)
      edits={};boat_fixes=[];surfixes=[]
      ocean=json.loads(zin.read(BOATS_SET))
      # Horizon-only biome radius scans a CHUNK grid about the start. Because imported
      # boat template pieces originate from their corner, radius must cover full ship length.
      if platform=='RUST':
        for e in ocean['structures']:
          sid=e['structure'];n=structure_path(sid)
          if not (sid.startswith('thalvoryn:overworld/imported/ships_') or (sid.startswith('trek:') and any(x in sid for x in ('trade','ship_pillager')))):
            continue
          d=json.loads(zin.read(n))
          if d.get('project_start_to_heightmap') and sid.startswith('thalvoryn:overworld/imported/ships_'):
             raise ValueError('Unexpected ship anchor changed '+sid)
          start_pool=d['start_pool']; poolpath='data/'+start_pool.replace(':','/worldgen/template_pool/')+'.json'
          pool=json.loads(zin.read(poolpath))
          sizes=[]
          for item in pool['elements']:
            loc=item['element'].get('location')
            if not isinstance(loc,str):continue
            p=template_path(loc)
            if p not in members_set:raise ValueError('Boat template missing '+p)
            _,root=read_nbt(zin.read(p));sz=root['size'].value[1]
            sizes.append((loc,sz))
          if not sizes:raise ValueError('Boat has no NBT template '+sid)
          largest=max(max(sz[0],sz[2]) for _,sz in sizes)
          radius=min(9,max(3,math.ceil(largest/16)+1))
          old=d.get('horizon:start_checks',{}).get('biome_radius')
          if old is None:raise ValueError('Boat lacks existing biome checks '+sid)
          if radius>old:
            d['horizon:start_checks']['biome_radius']=radius
            edits[n]=jsonbytes(d);boat_fixes.append({'id':sid,'max_footprint':largest,'radius_from':old,'radius_to':radius})
        # Specific land structures with heightmap projection and 'beard' terrain adaptation,
        # yet no slope/radius checks; avoid changing special sky/orbital/ocean structures.
        for sid in sorted(SURFACE):
          n=structure_path(sid); d=json.loads(zin.read(n))
          if d.get('horizon:start_checks') is not None:raise ValueError('Already guarded '+sid)
          if not d.get('project_start_to_heightmap') or d.get('terrain_adaptation') not in ['beard_thin','beard_box']:
            raise ValueError('Not a supported land anchor '+sid)
          d['horizon:start_checks']={'terrain_radius':2,'terrain_range':24,'liquid':'top'}
          edits[n]=jsonbytes(d);surfixes.append(sid)
      # Universal fix: separate surface dungeons from actual underground dungeons.
      old=json.loads(zin.read(DUNGEON_SET)); underground=[];surface=[]
      for entry in old['structures']:
        sid=entry['structure'];d=json.loads(zin.read(structure_path(sid)))
        (surface if d.get('project_start_to_heightmap') else underground).append(entry)
      if len(surface)!=15 or len(underground)!=11:raise ValueError(f'Unexpected split: {len(surface)} surface/{len(underground)} deep')
      underground_obj=json.loads(zin.read(DUNGEON_SET))
      underground_obj['structures']=underground
      underground_obj['placement']['frequency']=0.47
      edits[DUNGEON_SET]=jsonbytes(underground_obj)
      surface_obj=json.loads(zin.read(DUNGEON_SET))
      surface_obj['structures']=surface
      surface_obj['placement'].update({'salt':40195729,'spacing':51,'separation':23,'frequency':0.33,'exclusion_zone':{'other_set':'thalvoryn:ecology/ow_settlements','chunk_count':10}})
      if SURF_DUNGEON_SET in members_set:raise ValueError('Surface set already exists')
      edits[SURF_DUNGEON_SET]=jsonbytes(surface_obj)
      # New version in all relevant command responses; preserve historic change logs.
      for n in members:
        if n in ['pack.mcmeta','data/thalvoryn/function/version.mcfunction','data/thalvoryn/function/check.mcfunction','data/thalvoryn/function/help.mcfunction','data/thalvoryn/function/horizon/status.mcfunction']:
          text=zin.read(n).decode('utf-8')
          if 'V69' not in text:raise ValueError('Missing expected V69 version '+n)
          edits[n]=text.replace('V69','V70').encode('utf-8')
      edits['V70_CHANGELOG_CZ.txt']=(f'Thalvoryn V70 {platform} Nether{height} - private static test\n\n'
       f'1) 15 povrchovych dungeon struktur presunuto do noveho sdileneho setu; 11 podzemnich zustava zvlast.\n'
       f'2) Povrchovy set spacing=51 separation=23 frequency=0.33 a exclusion 10 chunku od settlements.\n'
       f'3) Podzemni set spacing=48 separation=19 frequency=0.47; oddelena volba 11 podzemnich struktur.\n'
       f'4) Rust: {len(boat_fixes)} vodnich lodi kontroluje hluboky ocean podle skutecne velikosti NBT.\n'
       f'5) Rust: {len(surfixes)} pozemnich staveb ma novy kontrolni filtr sklonu a kapaliny.\n'
       f'6) Jinak worldgen, Nether/End, loot a NBT sablony beze zmen.\n'
       f'7) Chovani v Java 26.3/Horizon NENI herne testovano.\n').encode('utf-8')
      for n in edits:
        if n in members_set and n=='V70_CHANGELOG_CZ.txt':raise ValueError('Unexpected old V70 text')
      with zipfile.ZipFile(target,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6) as zout:
        for info in zin.infolist():
          data=edits.pop(info.filename, None)
          if data is None:data=zin.read(info.filename)
          zout.writestr(info,data)
        for name,data in sorted(edits.items()):zout.writestr(name,data,compress_type=zipfile.ZIP_DEFLATED)
      with zipfile.ZipFile(target) as zz:
        if zz.testzip():raise ValueError('ZIP failed CRC')
        sha=hashlib.sha256(target.read_bytes()).hexdigest()
      return {'platform':platform,'height':height,'src':str(src),'path':str(target),'sha256':sha,'boat_checks':boat_fixes,'land_checks':surfixes,'surface_dungeons':[x['structure'] for x in surface],'underground_dungeons':[x['structure'] for x in underground]}

if __name__=='__main__':
  out={'status':'STATIC_DATA_BUILD_ONLY_NOT_GAME_VERIFIED','variants':[]}
  for platform,height in NAMES:
    item=build(platform,height);out['variants'].append(item)
    print(platform,height,'boat checks',len(item['boat_checks']),'land checks',len(item['land_checks']),'SHA',item['sha256'][:12],flush=True)
  (DST/'V70_BUILD_MANIFEST.json').write_bytes(jsonbytes(out))