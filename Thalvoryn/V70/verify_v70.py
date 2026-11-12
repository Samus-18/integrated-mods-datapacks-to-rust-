#!/usr/bin/env python3
"""Independent validator for Thalvoryn V70. Does not import its builder.
Rules are checked against V69 source zip. This is NOT a game/runtime test.
"""
import zipfile, json, hashlib, collections, re, math
from pathlib import Path
ROOT=Path('/mnt/data/v68_work');SRC=ROOT/'v69_release_candidate';DST=ROOT/'v70_release_candidate'
VERS=[('RUST',1024),('RUST',2032),('JAVA',1024),('JAVA',2032)]
SETU='data/thalvoryn/worldgen/structure_set/ecology/ow_dungeons.json'
SETS='data/thalvoryn/worldgen/structure_set/ecology/ow_surface_dungeons.json'
report={'status':'PASS_STATIC_ONLY_NOT_GAME_TESTED','variants':{},'checks':['all ZIP CRC and JSON','exact diff V69=>V70','all NBT and loot unchanged','two dungeon groups properly partitioned','no duplicate active definitions','Rust ship biome checks and selected land checks','version strings and absence of Stellarity','all placements valid']}
for plat,height in VERS:
    old=SRC/f'Thalvoryn-{plat}-Nether{height}-V69-PRIVATE-TEST.zip'
    new=DST/f'Thalvoryn-{plat}-Nether{height}-V70-PRIVATE-TEST.zip'
    with zipfile.ZipFile(old) as a, zipfile.ZipFile(new) as b:
      an=set(a.namelist());bn=set(b.namelist())
      assert len(an)==len(a.namelist()),'Duplicate names in base'
      assert len(bn)==len(b.namelist()),'Duplicate names in output'
      assert b.testzip() is None,'CRC failure'
      added=sorted(bn-an);removed=sorted(an-bn)
      assert added==[SETS,'V70_CHANGELOG_CZ.txt'] or added==['V70_CHANGELOG_CZ.txt',SETS],added
      assert not removed,removed
      changed=sorted(n for n in an if a.read(n)!=b.read(n))
      meta=['pack.mcmeta','data/thalvoryn/function/check.mcfunction','data/thalvoryn/function/help.mcfunction','data/thalvoryn/function/horizon/status.mcfunction','data/thalvoryn/function/version.mcfunction']
      assert all(n in changed for n in meta)
      assert SETU in changed
      unexpected=[]
      for n in changed:
        if n in meta or n==SETU:continue
        if plat=='RUST' and n.startswith('data/') and n.endswith('/nonsense'):
          unexpected.append(n)
        elif plat=='RUST' and '/worldgen/structure/' in n and n.endswith('.json'):
          pass
        else:
          unexpected.append(n)
      assert not unexpected,('unexpected touched paths',unexpected)
      assert all(not n.endswith('.nbt') for n in changed)
      assert all('/loot_table/' not in n and '/loot_tables/' not in n for n in changed)
      assert all('/dimension/' not in n and '/noise_settings/' not in n and '/density_function/' not in n for n in changed)
      assert all('/worldgen/biome/' not in n for n in changed)
      for n in changed:
        if n.startswith('data/') and '/worldgen/structure/' in n:
          assert plat=='RUST'
          x=json.loads(a.read(n));y=json.loads(b.read(n))
          delta={k for k in (set(x)|set(y)) if x.get(k)!=y.get(k)}
          assert delta=={'horizon:start_checks'},(n,delta)
      # Check every JSON file parseable, and pack format is 121.
      for n in bn:
        if n.endswith(('.json','.mcmeta')):json.loads(b.read(n))
      pack=json.loads(b.read('pack.mcmeta'))['pack']
      assert pack['pack_format']==121 and 'V70' in pack['description']
      for n in meta[1:]:assert b'V70' in b.read(n)
      assert not any('stellarity' in n for n in bn)
      assert all(b'stellarity' not in b.read(n).lower() for n in bn if n.endswith(('.json','.mcmeta','.mcfunction')))
      # Confirm original dungeon set is not duplicated and partition is exactly correct.
      oldset=json.loads(a.read(SETU));deep=json.loads(b.read(SETU));surface=json.loads(b.read(SETS))
      allprev={e['structure']:e['weight'] for e in oldset['structures']}
      deeps={e['structure']:e['weight'] for e in deep['structures']}
      surfs={e['structure']:e['weight'] for e in surface['structures']}
      assert len(deeps)==11 and len(surfs)==15
      assert not set(deeps)&set(surfs)
      assert allprev==(deeps|surfs)
      for id in deeps:
        d=json.loads(b.read('data/'+id.replace(':','/worldgen/structure/')+'.json'))
        assert not d.get('project_start_to_heightmap'),id
      for id in surfs:
        d=json.loads(b.read('data/'+id.replace(':','/worldgen/structure/')+'.json'))
        assert d.get('project_start_to_heightmap'),id
      p=deep['placement'];q=surface['placement'];
      assert (p['spacing'],p['separation'],p['frequency'])==(48,19,0.47)
      assert (q['spacing'],q['separation'],q['frequency'])==(51,23,0.33)
      assert q['exclusion_zone']['other_set']=='thalvoryn:ecology/ow_settlements' and q['exclusion_zone']['chunk_count']==10
      # Validate spread sets, uniqueness and all active structure definitions.
      refs=collections.defaultdict(list)
      set_count=0
      for n in bn:
        if '/worldgen/structure_set/' not in n or not n.endswith('.json'):continue
        set_count+=1; d=json.loads(b.read(n));p=d.get('placement',{})
        if p.get('type')=='minecraft:random_spread':
          sp=p['spacing'];sep=p['separation']
          assert 0<=sep<sp<=4096,(n,sp,sep)
          assert 0<=p.get('frequency',1)<=1
          if 'exclusion_zone' in p:
            assert 1<=p['exclusion_zone']['chunk_count']<=16
        for entry in d.get('structures',[]):
          id=entry['structure'];refs[id].append(n)
          path='data/'+id.replace(':','/worldgen/structure/')+'.json'
          assert path in bn or id.startswith('minecraft:') and path not in an,('undefined structure',id,n)
      dups={id:n for id,n in refs.items() if len(n)!=1}
      assert not dups,dups
      boat_checks=[];land_checks=[]
      for n in changed:
        if '/worldgen/structure/' not in n:continue
        oldd=json.loads(a.read(n));d=json.loads(b.read(n))
        id=n.split('/')[1]+':'+n.split('/worldgen/structure/')[1][:-5]
        if '/ships_' in n or ('/trek/' in n and any(k in n for k in ('_trade','ship_pillager'))):
          oldradius=oldd['horizon:start_checks'].get('biome_radius')
          newradius=d['horizon:start_checks'].get('biome_radius')
          assert oldradius==2 and 3<=newradius<=9,(id,oldradius,newradius)
          boat_checks.append((id,newradius))
        else:
          assert d['horizon:start_checks']=={'terrain_radius':2,'terrain_range':24,'liquid':'top'},id
          assert oldd.get('horizon:start_checks') is None
          land_checks.append(id)
      if plat=='RUST': assert len(boat_checks)==33 and len(land_checks)==6,(len(boat_checks),len(land_checks))
      else:assert not boat_checks and not land_checks
      assert len(refs)==478,(len(refs),plat,height)
      summary={'zip_sha256':hashlib.sha256(new.read_bytes()).hexdigest(),'files':len(bn),'changed':len(changed),'added':added,'active_unique':len(refs),'structure_sets':set_count,'surface_dungeon_ids':len(surfs),'deep_dungeon_ids':len(deeps),'rust_ocean_checks':len(boat_checks),'rust_land_checks':len(land_checks),'nbt_count':sum(n.endswith('.nbt') for n in bn)}
      report['variants'][f'{plat}_{height}']=summary
      print(plat,height,summary,flush=True)
(DST/'V70_INDEPENDENT_QA.json').write_text(json.dumps(report,indent=2,sort_keys=True,ensure_ascii=False)+'\n',encoding='utf-8')