#!/usr/bin/env python3
"""Thalvoryn V67 private recovery test build. NO GAME RUNTIME TEST CLAIMED.
Inputs: user's own 26.3 source packs and previous V67. Never publish third-party assets without clearance.
Preserve originals, avoid replacing root minecraft/overworld JSON wholesale, create auditable changes.
"""
from pathlib import Path
from collections import Counter,defaultdict
import zipfile,random,math,json,hashlib,gzip,copy,sys,time
from nbt_codec_v67 import basic_template,read_nbt,write_nbt,t
ROOT=Path('/mnt/data');WD=ROOT/'Thalvoryn_V67_work'
JSON=lambda obj:(json.dumps(obj,indent=2,sort_keys=True,ensure_ascii=False)+'\n').encode()
INPUTS={
 'dnt':ROOT/'Dungeons and Taverns v6.1.0.zip',
 'ancient':ROOT/'DnT Ancient City Overhaul v4.zip',
 'structures':ROOT/'Structures v4.4.zip',
 'structuresplus':ROOT/'Structures+ v3.0.zip',
 'explorify':ROOT/'Explorify v1.6.5.mod(1).jar',
 'hellish':ROOT/'HellishTrials-fabric-1.0.7(1).jar',
 'underground':ROOT/'UndergroundVillages-neoforge-26.3-13.0.1(1).jar',
 'stellarity':ROOT/'Stellarity-6.0.1.zip',
 'trek':ROOT/'Trek 26.3 B0.6.2.1(1).zip'
}
ARTIFACTS={}
MODULE_LOG={}

# Separate native-data source overlay. Source assets are staged for private testing only.
def external_overlays():
 overlay={};module_counts=Counter();conflicts=defaultdict(list);supported_ns={
 'dnt':{'nova_structures'},'structures':{'structures'},'structuresplus':{'structuresplus'},
 'explorify':{'explorify','cristellib'},'hellish':{'hellish_trials'},
 'underground':{'underground_villages'},
 'stellarity':{'stellarity','kohara','animated_java','awesomedungeonend','smithed'}
 }
 for label in ['dnt','ancient','structures','structuresplus','explorify','hellish','underground','stellarity','trek']:
  f=INPUTS[label]
  with zipfile.ZipFile(f) as z:
   for fn in z.namelist():
    if not fn.startswith('data/') or fn.endswith('/'):continue
    names=fn.split('/')
    if len(names)<4:continue
    ns=names[1]
    if label=='trek': # V67 already has TREK; take only missing under TREK namespace
     if ns!='trek':continue
    elif label=='ancient':
     # Only ancillary resources; do not replace vanilla ancient city without spawn/loot run tests.
     if ns!='nova_structures':continue
    elif ns not in supported_ns[label]:continue
    if label=='stellarity' and fn.endswith('.snbt'):continue
    raw=z.read(fn)
    if label=='underground' and fn=='data/underground_villages/worldgen/structure/underground_village.json':
     d=json.loads(raw)
     # Custom NeoForge height provider would be rejected by plain Horizon datapack.
     d['start_height']={'absolute':-38}
     d['project_start_to_heightmap']=None
     d.pop('project_start_to_heightmap',None)
     d['terrain_adaptation']='beard_thin'
     raw=JSON(d)
    if label=='hellish' and fn=='data/hellish_trials/worldgen/structure/nether_trial.json':
     d=json.loads(raw);d['terrain_adaptation']='beard_thin' if d.get('terrain_adaptation')=='encapsulate' else d.get('terrain_adaptation')
     raw=JSON(d)
    if fn in overlay:
     if raw!=overlay[fn]:conflicts[label].append(fn)
     continue
    overlay[fn]=raw;module_counts[label]+=1
 # MC 26.3 root Stellarity files use material_rule / 26.3 biome attributes.
 # Do NOT apply the older 26.2 overlay: it uses obsolete noise and biome fields.
 with zipfile.ZipFile(INPUTS['stellarity']) as z:
  for fn in z.namelist():
   if not fn.startswith('data/minecraft/') or fn.endswith('/'):continue
   if (fn.startswith('data/minecraft/worldgen/placed_feature/') or
       fn.startswith('data/minecraft/worldgen/configured_feature/') or
       fn.startswith('data/minecraft/worldgen/template_pool/') or
       fn.startswith('data/minecraft/tags/') or
       fn.startswith('data/minecraft/loot_table/')):
    overlay.setdefault(fn,z.read(fn))
    module_counts['stellarity_minecraft_support']+=1
 MODULE_LOG.update({'external_files_by_module':dict(module_counts),'conflicts':dict(conflicts)})
 return overlay

EXTERNAL=external_overlays()

# Procedural rock island, with pointed irregular underside, not flat slabs.
def make_island(i):
 rng=random.Random(676700+i*3747)
 xs,zs=[(45,47),(47,43),(49,45),(43,49),(49,49),(41,45)][i]
 sy=50;cx=(xs-1)/2;cz=(zs-1)/2
 palettes=[
 ['minecraft:stone','minecraft:andesite','minecraft:dirt','minecraft:grass_block','minecraft:moss_block','minecraft:oak_log','minecraft:oak_leaves','minecraft:flowering_azalea_leaves','minecraft:coarse_dirt'],
 ['minecraft:stone','minecraft:tuff','minecraft:dirt','minecraft:grass_block','minecraft:calcite','minecraft:cherry_log','minecraft:cherry_leaves','minecraft:moss_block','minecraft:rooted_dirt'],
 ['minecraft:deepslate','minecraft:andesite','minecraft:dirt','minecraft:grass_block','minecraft:moss_block','minecraft:spruce_log','minecraft:spruce_leaves','minecraft:tuff','minecraft:gravel'],
 ['minecraft:stone','minecraft:granite','minecraft:dirt','minecraft:grass_block','minecraft:moss_block','minecraft:oak_log','minecraft:oak_leaves','minecraft:flowering_azalea_leaves','minecraft:coarse_dirt'],
 ['minecraft:stone','minecraft:calcite','minecraft:dirt','minecraft:grass_block','minecraft:moss_block','minecraft:cherry_log','minecraft:cherry_leaves','minecraft:tuff','minecraft:rooted_dirt'],
 ['minecraft:stone','minecraft:andesite','minecraft:dirt','minecraft:grass_block','minecraft:moss_block','minecraft:oak_log','minecraft:oak_leaves','minecraft:flowering_azalea_leaves','minecraft:rooted_dirt']
 ][i]
 vox={};surf={}
 for x in range(2,xs-2):
  for z in range(2,zs-2):
   nx=(x-cx)/(cx-2);nz=(z-cz)/(cz-2)
   angle=math.atan2(nz,nx);r=math.hypot(nx,nz)
   wiggle=.045*math.sin(angle*(6+i)+.6*i)+.055*math.cos(angle*(11-i)+i)
   if i in (1,3):r=min(math.hypot(nx+.35,nz)*.85,math.hypot(nx-.32,nz)*.92)
   if i==4 and nx>.18 and nz<-.24:r+=.20
   if r>0.93+wiggle:continue
   variation=math.sin(x*.51+i)*1.4+math.cos(z*.37-i)*1.7
   top=min(38,max(23,int(30+variation+(1-r)*4)))
   thickness=4+int(23*(max(0,1-r)**.87))+int(3*abs(math.sin(2.2*angle+i)))
   bottom=max(2,top-thickness)
   for y in range(bottom,top+1):
    u=rng.random()
    if y==top:state=3 if u<.76 else 4 if u<.89 else 8
    elif y>=top-3:state=2 if u<.82 else 0
    else:state=0 if u<.75 else 1 if u<.93 else 7
    vox[(x,y,z)]=state
   surf[(x,z)]=top
 seeds=[[(17,20),(29,29)],[(15,22),(28,25)],[(18,19),(34,25)],[(20,18),(31,28)],[(22,25),(34,32)],[(15,20),(27,27)]][i]
 for tx,tz in seeds:
  pts=sorted(surf,key=lambda p:(p[0]-tx)**2+(p[1]-tz)**2)
  x,z=pts[0];y=surf[x,z]+1;h=4+(i%3)
  if y+h+3>=sy:continue
  for dy in range(h):vox[(x,y+dy,z)]=5
  for dy in range(h-2,h+3):
   rr=2 if dy<=h else 1
   for xx in range(-rr,rr+1):
    for zz in range(-rr,rr+1):
     if (xx*xx+zz*zz)<=(rr*rr+1) and (x+xx,y+dy,z+zz) not in vox and 0<x+xx<xs and 0<z+zz<zs:vox[(x+xx,y+dy,z+zz)]=6 if rng.random()<.9 else 7
 return basic_template((xs,sy,zs),palettes,vox),len(vox),[xs,sy,zs]

# A steep, winding tunnel to a handcrafted cavern (does not falsely claim intersection with natural carvers).
def make_tunnel(i):
 rng=random.Random(6700+i*961);size=(35,73,82)
 p=['minecraft:stone','minecraft:mossy_stone_bricks','minecraft:cobblestone','minecraft:stone_bricks',
    'minecraft:air','minecraft:tuff','minecraft:andesite','minecraft:oak_planks',
    'minecraft:polished_andesite','minecraft:lantern','minecraft:calcite','minecraft:dripstone_block',
    'minecraft:moss_block','minecraft:gravel','minecraft:oak_log','minecraft:glow_lichen']
 vox={}
 color=[1,3,5,11,12,6][i]
 for z in range(1,51):
  # 1 vertical block lost for every full z step, steep and traversable.
  y=65-z
  center=17+round(4*math.sin(z/10.0+i*.71))
  for x in range(center-5,center+6):
   for yy in range(y-2,y+5):
    if not (0<x<size[0]-1 and 0<yy<size[1]-1):continue
    dx=abs(x-center)
    if (dx<=2 and y<=yy<=y+3):v=4
    elif (dx<=2 and yy==y-1):v=3 if z%5 else 7
    elif dx==3 and yy in (y,y+1,y+2):v=color if rng.random()<.65 else 0
    elif dx<=4 and yy>=y+4:v=0 if rng.random()<.75 else 5
    elif dx<=4 and yy==y-2:v=0 if rng.random()<.8 else 6
    else:continue
    vox[x,yy,z]=v
  if z%11==0:
   # Wall lanterns are separate from passage floor.
   for dx in (-3,3):vox[(center+dx,y+2,z)]=9
 # Cave chamber at the tunnel's deepest end. Stone shell blends into surrounding underground.
 c=(17,14,63);rx,ry,rz=14,10,16
 for x in range(2,33):
  for z in range(46,80):
   for y in range(2,30):
    metric=((x-c[0])/rx)**2+((y-c[1])/ry)**2+((z-c[2])/rz)**2
    rough=.03*math.sin(x*.9+z*.43+i)+.045*math.cos(z*.37+i*.81)
    if metric<1+rough:
     if y==5 and 7<x<27 and 53<z<75:state=8 if rng.random()<.7 else color
     elif metric>.80 and rng.random()<.21:state=color
     else:state=4
     vox[(x,y,z)]=state
    elif 1+rough<=metric<1.17 and rng.random()<.68:
     vox[(x,y,z)]=0 if rng.random()<.7 else color
 # rubble, vertical stakes, imperfect walls, small stairs to cavern edge
 for x in range(10,25,3):
  for z in range(56,73,4):
   if rng.random()<.18:vox[(x,6,z)]=color
 # Guaranteed open tunnel-to-cavern breach near entrance.
 for z in range(46,59):
  y=max(8,65-z)
  for x in range(15,20):
   for yy in range(y,y+5):vox[(x,yy,z)]=4
 # Vary the entrance facade by biome/variant.
 for z in range(0,5):
  for x in range(12,23):
   for y in range(60,70):
    edge=(x in (12,13,21,22) or y in (68,69))
    if edge and rng.random()<.71:vox[(x,y,z)]=color if rng.random()<.66 else 2
 # no spawners: user reported overspawn
 return basic_template(size,p,vox),len(vox),list(size)

# Small End floating island variants with tapered underside.
def make_end_island(i):
 rng=random.Random(800+i*456)
 sx,sz=39+(i%2)*8,41+((i+1)%2)*6;sy=39;cx=(sx-1)/2;cz=(sz-1)/2
 palette=['minecraft:end_stone','minecraft:purpur_block','minecraft:obsidian','minecraft:calcite','minecraft:amethyst_block','minecraft:chorus_plant','minecraft:chorus_flower']
 vox={}
 for x in range(1,sx-1):
  for z in range(1,sz-1):
   nx=(x-cx)/cx;nz=(z-cz)/cz;r=math.hypot(nx,nz)+.09*math.sin(math.atan2(nz,nx)*5+i)
   if r>0.94:continue
   top=max(19,min(29,24+int(2*math.sin(x*.41)+3*math.cos(z*.27))))
   bottom=max(1,top-(5+round(17*((1-r)**.85))))
   for y in range(bottom,top+1):
    voxel=0 if rng.random()<.90 else (3 if i%2 else 2)
    vox[(x,y,z)]=voxel
   if i%2 and rng.random()<.012 and top+4<sy:
    for dy in range(1,4):vox[(x,top+dy,z)]=5
    vox[(x,top+4,z)]=6
 return basic_template((sx,sy,sz),palette,vox),len(vox),[sx,sy,sz]

GENERATED={};GEOMETRY={}
for i in range(6):
 for family,creator in [('sky',make_island),('underground',make_tunnel)]:
  nbt,size,dims=creator(i)
  name=f'data/thalvoryn/structure/{family}/v67_{"sky_isle" if family=="sky" else "cave_tunnel"}_{i+1:02d}.nbt'
  GENERATED[name]=nbt;GEOMETRY[name]={'blocks':size,'size':dims}
for i in range(3):
 raw,num,dims=make_end_island(i)
 name=f'data/thalvoryn/structure/end/v67_void_rock_{i+1:02d}.nbt'
 GENERATED[name]=raw;GEOMETRY[name]={'blocks':num,'size':dims}

# Helpers

def jigsaw(pool,biomes,step='surface_structures',height=None,project=None,terrain='none',size=1):
 d={'type':'minecraft:jigsaw','biomes':biomes,'start_pool':pool,'step':step,'size':size,
    'start_height':height or {'absolute':0},'max_distance_from_center':128,
    'terrain_adaptation':terrain,'use_expansion_hack':False,'spawn_overrides':{}}
 if project:d['project_start_to_heightmap']=project
 return d

def pool(template):return {'fallback':'minecraft:empty','elements':[{'weight':1,'element':{'element_type':'minecraft:single_pool_element','location':template,'projection':'rigid','processors':'minecraft:empty'}}]}

def random_set(items,spacing,separation,frequency,salt,exclusion=None):
 if spacing<=separation or not 0<frequency<=1:raise ValueError('bad spread')
 d={'structures':[{'structure':item,'weight':w} for item,w in items],
    'placement':{'type':'minecraft:random_spread','salt':salt,'spacing':spacing,'separation':separation,'frequency':frequency,'spread_type':'triangular'}}
 if exclusion:d['placement']['exclusion_zone']={'other_set':exclusion[0],'chunk_count':exclusion[1]}
 return d

# Add active references for previously dormant source definitions, preserve ocean-biome / End structure semantics.
def categorize_inactive(files):
 structures={n.replace('data/','',1).replace('/worldgen/structure/',':').replace('.json',''):n
   for n in files if n.startswith('data/') and '/worldgen/structure/' in n and n.endswith('.json')}
 linked=set()
 for n,raw in files.items():
  if '/worldgen/structure_set/' not in n or not n.endswith('.json'):continue
  d=json.loads(raw)
  for o in d.get('structures',[]):linked.add(o.get('structure') if isinstance(o,dict) else o)
 out=defaultdict(list)
 for sid,path in structures.items():
  if sid in linked or not sid.startswith('thalvoryn:'):continue
  d=json.loads(files[path]);bio=d.get('biomes',[])
  if path.startswith('data/thalvoryn/worldgen/structure/nether/'):
   category='nether_ships' if 'ships_' in path or 'frigate' in path else 'nether_structures'
  elif path.startswith('data/thalvoryn/worldgen/structure/end/'):
   category='end_ships' if 'ships_' in path or 'sky_caravan' in path else 'end_ruins'
  elif path.startswith('data/thalvoryn/worldgen/structure/overworld/'):
   category='overworld_ships' if 'ships_' in path else 'overworld_large'
  elif 'small_island' in path:category='end_islands'
  else:category='other'
  out[category].append((sid,path))
 return out

# Pure string references in pool -> structure, safe for set creation.

def repair_missing_nbt_chests(files):
 target=['data/thalvoryn/structure/overworld/imported/buildings_02.nbt',
         'data/thalvoryn/structure/overworld/imported/buildings_03.nbt',
         'data/thalvoryn/structure/overworld/imported/buildings_08.nbt',
         'data/thalvoryn/structure/overworld/imported/buildings_12.nbt',
         'data/thalvoryn/structure/nether/imported/buildings_06.nbt']
 result={}
 for path in target:
  if path not in files:continue
  nam,root=read_nbt(files[path]);pal=root['palette'].value[1]
  palids=[(a.get('id') or a.get('Name')).value for a in pal]
  loot='thalvoryn:chests/rc37/nether/generic_uncommon_wastes' if '/nether/' in path else 'thalvoryn:chests/rc37/overworld/generic_uncommon_plains'
  fixed=0
  for blk in root['blocks'].value[1]:
   state=blk['state'].value
   if 'chest' in palids[state] and blk.get('nbt') is None:
    blk['nbt']=t(10,{'id':t(8,'minecraft:chest'),'LootTable':t(8,loot)})
    fixed+=1
  if fixed:files[path]=write_nbt(nam,root)
  result[path]=fixed
 return result

def repair_external_missing_templates(files):
 # Source Dungeons & Taverns 6.1.0 itself includes 8 missing NBT links.
 # Avoid worldgen crash by replacing only those unresolved entries with nearby valid assets.
 table={
  'nova_structures:witch_huts/mangrove/mangrove_witch_hut_8': 'nova_structures:witch_huts/mangrove/mangrove_witch_hut_9',
  'nova_structures:mangrove_witch_hut/mangrove_witch_hut_suport':'nova_structures:witch_huts/mangrove/mangrove_witch_hut_1',
 }
 paths=['data/nova_structures/worldgen/template_pool/mangrove_witch_hut_brewing.json',
        'data/nova_structures/worldgen/template_pool/mangrove_witch_hut_suport.json',
        'data/nova_structures/worldgen/template_pool/witch_huts/mangrove_witch_hut.json']
 changes=[]
 for path in paths:
  if path not in files:continue
  obj=json.loads(files[path]);ne=[]
  for entry in obj.get('elements',[]):
   elem=entry.get('element',{});loc=elem.get('location','')
   if loc in table:
    elem['location']=table[loc];changes.append({'path':path,'from':loc,'to':table[loc]})
   elif loc.startswith('nova_structures:mangrove_witch_hut/mangrove_witch_hut_brewingstand_'):
    # Missing six source templates: use a harmless empty element; do not invent layouts.
    entry['element']={'element_type':'minecraft:empty_pool_element'}
    changes.append({'path':path,'from':loc,'to':'minecraft:empty_pool_element'})
   ne.append(entry)
  obj['elements']=ne;files[path]=JSON(obj)
 path='data/stellarity/worldgen/template_pool/desert_ruin.json'
 if path in files:
  v=json.loads(files[path]);v['fallback']='minecraft:empty';files[path]=JSON(v)
 path='data/stellarity/worldgen/structure/floating_treasure.json'
 if path in files:
  v=json.loads(files[path]);v['type']='minecraft:jigsaw';files[path]=JSON(v)
 return changes

# Programmatic changes per platform.
def changed_files(files,platform,height):
 notes=defaultdict(list)
 def write(path,obj):files[path]=JSON(obj);notes['updated'].append(path)
 def get(path):return json.loads(files[path])
 # Restore pre-RC31 mountain factor: prioritize more gradual valleys/land patches.
 for leaf,restore in [('extreme_mountains_factor',.25),('extreme_mountains_jagged',.055)]:
  path='data/thunderfusion/worldgen/density_function/overworld_overworld_'+leaf+'.json'
  d=get(path);old=d['left']['right'];d['left']['right']=restore
  write(path,d);notes['terrain'].append((path,old,restore))
 # Increase deep natural caves; prior world had only 2–2.5% cave probability.
 for carver,probability,max_y in [('cave',.085,480),('cave_extra_underground',.075,180),('canyon',.013,300)]:
  carver_path=f'data/minecraft/worldgen/carver/{carver}.json'
  if carver_path in files:
   cv=get(carver_path)
   cv['probability']=probability
   cv['y']['max_inclusive']={'absolute':max_y}
   write(carver_path,cv)
 for carver,probability in [('alpine_cliff_passage',.027),('highland_cave_passage',.055),('woodland_cave_passage',.062)]:
  carver_path=f'data/thalvoryn/worldgen/carver/{carver}.json'
  if carver_path in files:
   cv=get(carver_path);cv['probability']=probability;write(carver_path,cv)
 for room,chance in [('monster_room',13),('monster_room_deep',17)]:
  f=f'data/minecraft/worldgen/placed_feature/{room}.json'
  if f in files:
   d=get(f)
   for placement in d.get('placement',[]):
    if placement.get('type')=='minecraft:rarity_filter':placement['chance']=chance
   write(f,d)
 # Existing rare sets to a more sensible balance / End large minimum separation
 for name,spacing,sep,freq in [
  ('data/minecraft/worldgen/structure_set/end_cities.json',58,26,.42),
  ('data/thalvoryn/worldgen/structure_set/end/landmarks.json',85,38,.27),
  ('data/thalvoryn/worldgen/structure_set/end/handmade/sky_caravan.json',100,42,.18),
  ('data/thalvoryn/worldgen/structure_set/end/sky_sanctuary.json',80,34,.18),
  ('data/thalvoryn/worldgen/structure_set/end/rc21_highland_ruins.json',64,24,.27),
  ('data/thalvoryn/worldgen/structure_set/end/rc35_micro_landmarks.json',52,22,.32),
  ('data/thalvoryn/worldgen/structure_set/sky/sky_garden.json',37,14,.77),
  ('data/thalvoryn/worldgen/structure_set/underground/spiral_hamlet.json',35,14,.46),
  ('data/thalvoryn/worldgen/structure_set/overworld/frontier_mix_rc36.json',41,17,.61),
  ('data/thalvoryn/worldgen/structure_set/overworld/imported/small_landmarks_rc36.json',42,16,.62),
 ]:
  if name in files:
   d=get(name);p=d['placement'];p.update({'spacing':spacing,'separation':sep,'frequency':freq});write(name,d)
 # Preserve old sky templates but prefer new organic, deep underside islands.
 sky_set='data/thalvoryn/worldgen/structure_set/sky/sky_garden.json';d=get(sky_set)
 d['structures']=[{'structure':f'thalvoryn:sky/v67_sky_isle_{i+1:02d}','weight':3} for i in range(6)]+[
 {'structure':x,'weight':1} for x in ['thalvoryn:sky/sky_garden','thalvoryn:sky/verdant_terrace','thalvoryn:sky/crescent_crag','thalvoryn:sky/twin_isles','thalvoryn:sky/amber_ridge']]
 write(sky_set,d)
 for i in range(6):
  name=f'v67_sky_isle_{i+1:02d}';id='thalvoryn:sky/'+name
  write(f'data/thalvoryn/worldgen/template_pool/sky/{name}.json',pool(id))
  write(f'data/thalvoryn/worldgen/structure/sky/{name}.json',jigsaw(id,['minecraft:plains','minecraft:meadow','minecraft:forest','minecraft:flower_forest','minecraft:cherry_grove','minecraft:birch_forest','minecraft:taiga','thalvoryn:crystal_meadows','thalvoryn:ancient_woodlands'],height={'type':'minecraft:uniform','min_inclusive':{'absolute':85},'max_inclusive':{'absolute':130}},project='WORLD_SURFACE_WG'))
 # Air vessel: make actually spawn, but rare and elevated above current relief.
 av='data/thalvoryn/worldgen/structure/overworld/handmade/air_vessel.json'
 if av in files:
  a=get(av);a['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':135},'max_inclusive':{'absolute':195}};write(av,a)
  write('data/thalvoryn/worldgen/structure_set/overworld/v67_giant_air_vessel.json',random_set([('thalvoryn:overworld/handmade/air_vessel',1)],72,30,.62,678001,('thalvoryn:sky/sky_garden',9)))
 # 6 natural steep cave entrances end in a custom deep chamber (not random staircase stubs)
 citems=[]
 for i in range(6):
  name=f'v67_cave_tunnel_{i+1:02d}';id='thalvoryn:underground/'+name
  write('data/thalvoryn/worldgen/template_pool/underground/'+name+'.json',pool(id))
  write('data/thalvoryn/worldgen/structure/underground/'+name+'.json',jigsaw(id,['minecraft:plains','minecraft:forest','minecraft:meadow','minecraft:cherry_grove','minecraft:taiga','minecraft:old_growth_pine_taiga','minecraft:stony_peaks','minecraft:windswept_hills','thalvoryn:crystal_meadows','thalvoryn:ancient_woodlands'],step='underground_structures',height={'type':'minecraft:uniform','min_inclusive':{'absolute':-66},'max_inclusive':{'absolute':-62}},project='WORLD_SURFACE_WG',terrain='beard_thin'))
  citems.append((id,1))
 write('data/thalvoryn/worldgen/structure_set/underground/v67_cave_tunnels.json',random_set(citems,28,12,.70,678002,('thalvoryn:underground/spiral_hamlet',4)))
 # End organic archipelago, not constant tiny flat fragments.
 eitems=[]
 for i in range(3):
  name=f'v67_void_rock_{i+1:02d}';id='thalvoryn:end/'+name
  write('data/thalvoryn/worldgen/template_pool/end/'+name+'.json',pool(id))
  write('data/thalvoryn/worldgen/structure/end/'+name+'.json',jigsaw(id,['minecraft:end_highlands','minecraft:end_midlands','minecraft:small_end_islands'],height={'type':'minecraft:uniform','min_inclusive':{'absolute':57},'max_inclusive':{'absolute':95}}))
  eitems.append((id,1))
 write('data/thalvoryn/worldgen/structure_set/end/v67_organic_void_rocks.json',random_set(eitems,39,16,.60,678003,('thalvoryn:end/landmarks',6)))
 # Do not allow vanilla Nether and End big ships stacked right next to End cities.
 old=categorize_inactive(files)
 groups={
  'overworld_large':('overworld/v67_large_landmarks',43,18,.62,678010,('minecraft:villages',12)),
  'overworld_ships':('overworld/v67_ocean_fleet',34,14,.56,678011,('trek:overworld/rc52_deep_ocean_vessels',9)),
  'nether_structures':('nether/v67_old_ruins',37,17,.68,678012,('thalvoryn:nether/highland_ruins',7)),
  'nether_ships':('nether/v67_aerial_fleet',55,24,.44,678013,('thalvoryn:nether/rc63_forest_sites',8)),
  'end_ruins':('end/v67_imported_ruins',75,31,.25,678014,('minecraft:end_cities',14)),
  'end_ships':('end/v67_imported_vessels',105,46,.17,678015,('minecraft:end_cities',18)),
  'end_islands':('end/v67_small_islands',43,15,.48,678016,('thalvoryn:end/landmarks',7))
 }
 for group,items in old.items():
  if group not in groups or not items:continue
  name,spacing,sep,freq,salt,excl=groups[group]
  write('data/thalvoryn/worldgen/structure_set/'+name+'.json',random_set([(id,1) for id,_ in items],spacing,sep,freq,salt,excl))
  notes['activated_groups'].append({'group':group,'ids':len(items),'set':name})
  for id,path in items:
   d=get(path)
   if group=='nether_structures' and not d.get('project_start_to_heightmap'):
    d['project_start_to_heightmap']='WORLD_SURFACE_WG'
    d['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':-13},'max_inclusive':{'absolute':-5}}
    d['terrain_adaptation']='beard_thin'
   if group=='nether_ships':
    d['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':160},'max_inclusive':{'absolute':740 if height==1024 else 1480}}
   if group=='end_ships':
    d['start_height']={'type':'minecraft:uniform','min_inclusive':{'absolute':75},'max_inclusive':{'absolute':150}}
    d['biomes']=['minecraft:end_highlands','minecraft:end_midlands']
   if group=='overworld_large' and d.get('terrain_adaptation')=='none' and d.get('project_start_to_heightmap'):
    d['terrain_adaptation']='beard_thin'
   if platform=='RUST' and group.startswith('nether'):
    # Native verified keys against exact Horizon 2ebf00a parser.
    d['horizon:start_checks']=({'liquid':'top','max_y':height-80} if group=='nether_ships' else {'liquid':'top','terrain_radius':1,'terrain_range':32,'max_y':height-80})
   else:d.pop('horizon:start_checks',None)
   write(path,d)
 # Prevent unreasonably dense duplicate spawner-style sites with 84 TREK structures in one pool.
 trek_set='data/trek/worldgen/structure_set/overworld/ecology_mix_rc36.json'
 if trek_set in files:
  d=get(trek_set);d['placement'].update({'spacing':22,'separation':9,'frequency':.79});write(trek_set,d)
 # End biome+noise activation runs after native file overlays; worldgen remain experimental.
 # External native sets throttled to avoid flooding the world with 35 extra DNT sets.
 for path in sorted(files):
  if '/worldgen/structure_set/' not in path or not path.endswith('.json'):continue
  if not any(path.startswith('data/'+ns+'/') for ns in ('nova_structures','explorify','structures','structuresplus','hellish_trials','underground_villages','stellarity')):continue
  d=get(path)
  p=d.get('placement',{})
  if p.get('type')!='minecraft:random_spread':continue
  if 'spacing' in p:
   oldsp=int(p['spacing'])
   # Huge landmark structures and skyships should be found but remain rare.
   p['spacing']=max(oldsp,28 if path.startswith('data/structures/') else 32)
   p['separation']=min(int(p.get('separation',max(3,p['spacing']//3))),p['spacing']-1)
  p['frequency']=min(float(p.get('frequency',1.0)),.45 if path.startswith('data/nova_structures/') else .50)
  if '/end_' in path or path.startswith('data/stellarity/') or 'ship' in path:
   p['spacing']=max(int(p['spacing']),66);p['separation']=max(int(p['separation']),24)
  if path.startswith('data/hellish_trials/'):
   p['spacing']=48;p['separation']=20;p['frequency']=.34
  write(path,d)
 # Patch user-provided 26.3 UndergroundVillages to be independent of custom NeoForge height provider.
 uv='data/underground_villages/worldgen/structure/underground_village.json'
 if uv in files:
  d=get(uv);d['start_height']={'absolute':-38};write(uv,d)
 # Stella End: enable custom End biomes only in experimental RUST while preserving original 26.3 dragon-island noise.
 # To avoid unstable overlay formats on a running world, source dimension is NOT replaced by raw 26.2 dimension file.
 return notes

def end_monument_spacing(files):
 # One End landmark lattice: no two large structures of this lattice start too close.
 # Independent End cities stay in a distinct lattice and are excluded by 18 chunks.
 sources=[
  'data/thalvoryn/worldgen/structure_set/end/landmarks.json',
  'data/thalvoryn/worldgen/structure_set/end/handmade/sky_caravan.json',
  'data/thalvoryn/worldgen/structure_set/end/sky_sanctuary.json',
  'data/thalvoryn/worldgen/structure_set/end/rc21_highland_ruins.json',
  'data/thalvoryn/worldgen/structure_set/end/v67_imported_ruins.json',
  'data/thalvoryn/worldgen/structure_set/end/v67_imported_vessels.json',
  'data/nova_structures/worldgen/structure_set/end_castle.json',
  'data/nova_structures/worldgen/structure_set/end_lighthouses.json',
  'data/nova_structures/worldgen/structure_set/end_ship.json',
  'data/explorify/worldgen/structure_set/end_shipwrecks.json',
  'data/structures/worldgen/structure_set/end_ship.json',
  'data/stellarity/worldgen/structure_set/chapel.json',
  'data/stellarity/worldgen/structure_set/shipwreck.json'
 ]
 weights=Counter();removed=[]
 for fn in sources:
  if fn not in files:continue
  src=json.loads(files.pop(fn))
  removed.append(fn)
  for entry in src.get('structures',[]):
   sid=entry['structure'] if isinstance(entry,dict) else entry
   score=1 if 'ship' in sid or 'vessel' in sid or 'caravan' in sid else 3
   if 'citadel' in sid or 'castle' in sid or 'sanctuary' in sid:score=5
   weights[sid]=max(weights[sid],score)
 main='data/thalvoryn/worldgen/structure_set/end/v67_spaced_monuments.json'
 files[main]=JSON(random_set(sorted(weights.items()),68,28,.57,678171,('minecraft:end_cities',18)))
 return {'active_large_structures':len(weights),'removed_separate_sets':removed,'single_set':main,
         'minimum_grid_separation_blocks':28*16,'end_city_exclusion_radius_blocks':18*16}

def build_one(platform,height):
 pattern=f'Thalvoryn-{platform}-Nether{height}-V67-NETHER-REBALANCED-UNRELEASED.zip'
 path=WD/pattern
 if not path.exists():raise FileNotFoundError(path)
 with zipfile.ZipFile(path) as src:
  files={n:src.read(n) for n in src.namelist() if not n.endswith('/')}
  preexisting=set(files)
  ext_add=Counter();ext_collisions=[]
  for fn,raw in EXTERNAL.items():
   if fn in files:
    if raw!=files[fn]:ext_collisions.append(fn)
   else:
    files[fn]=raw;ext_add[fn.split('/')[1]]+=1
  files.update(GENERATED)
  repaired=repair_missing_nbt_chests(files)
  missing_fixed=repair_external_missing_templates(files)
  notes=changed_files(files,platform,height)
  notes['missing_source_templates_fixed']=missing_fixed
  # Activate 26.3-native Stellarity End worldgen without replacing the Overworld or Nether.
  with zipfile.ZipFile(INPUTS['stellarity']) as se:
   end_paths=['data/minecraft/dimension/the_end.json',
              'data/minecraft/worldgen/noise_settings/end.json',
              'data/minecraft/worldgen/biome/end_barrens.json',
              'data/minecraft/worldgen/biome/end_highlands.json',
              'data/minecraft/worldgen/biome/end_midlands.json']
   for ep in end_paths:
    obj=json.loads(se.read(ep))
    if ep.endswith('/noise_settings/end.json'):
     obj['noise']['height']=json.loads(files['data/minecraft/dimension_type/the_end.json'])['height']
    files[ep]=JSON(obj)
   notes['end_overhaul']=['Stellarity 26.3 root End dimension multi_noise, 24 biomes',
                          'End noise from Stellarity (height matched to Thalvoryn 512)',
                          'Overworld and Nether generation otherwise unaffected by this optional End replacement']
  # Private 26.3 Dungeons & Taverns Ancient City overhaul (native jigsaw + city pools).
  with zipfile.ZipFile(INPUTS['ancient']) as az:
   ancient_add=[]
   for ep in az.namelist():
    if not ep.startswith('data/minecraft/') or ep.endswith('/'):
     continue
    if ep.startswith(('data/minecraft/structure/ancient_city/',
                      'data/minecraft/worldgen/template_pool/ancient_city/',
                      'data/minecraft/worldgen/processor_list/ancient_city',
                      'data/minecraft/loot_table/archaelogy/ancient_city',
                      'data/minecraft/loot_table/chests/ancient_city',
                      'data/minecraft/worldgen/structure/ancient_city.json',
                      'data/minecraft/worldgen/structure_set/ancient_cities.json')):
     files[ep]=az.read(ep)
     ancient_add.append(ep)
   notes['ancient_city_overhaul']={'native_files':len(ancient_add),'original_worldgen_overridden':True}
  notes['unified_end_monuments']=end_monument_spacing(files)
  # Preserve native load/tick function tags in the base and additive dependency packs.
  for tag_name in ['data/minecraft/tags/function/load.json','data/minecraft/tags/function/tick.json']:
   sources=[]
   for f in [INPUTS['stellarity'],INPUTS['dnt']]:
    with zipfile.ZipFile(f) as sec:
     if tag_name in sec.namelist():sources.append(json.loads(sec.read(tag_name)))
   if sources:
    acc=json.loads(files[tag_name]) if tag_name in files else {'values':[]}
    vals=acc.get('values',[])
    for obj in sources:
     for v in obj.get('values',[]):
      if v not in vals:vals.append(v)
    acc['values']=vals;acc.pop('replace',None)
    files[tag_name]=JSON(acc)

  # Never override old dimension heights / metadata.
  meta=json.loads(src.read('pack.mcmeta'))
  meta['pack']['description']=f'Thalvoryn V67 test: source structures + tunnel & sky repair {platform} Nether{height} (not game-tested)'
  files['pack.mcmeta']=JSON(meta)
  out=WD/f'Thalvoryn-{platform}-Nether{height}-V67-INTEGRATION-TEST-UNRELEASED.zip'
  with zipfile.ZipFile(out,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True) as z:
   for n,raw in sorted(files.items()):z.writestr(n,raw)
  with zipfile.ZipFile(out) as ver:
   assert ver.testzip() is None
   assert len(ver.namelist())==len(set(ver.namelist()))
   dim=json.loads(ver.read('data/minecraft/dimension_type/the_nether.json'))
   assert dim['height']==height
  return {'path':str(out),'bytes':out.stat().st_size,'sha256':hashlib.sha256(out.read_bytes()).hexdigest(),
          'entry_count':len(files),'new_entries':len(files)-len(preexisting),
          'external_additions':dict(ext_add),'external_conflicts_skipped':len(ext_collisions),
          'external_conflict_sample':ext_collisions[:40],
          'repaired_legacy_chests':repaired,'notes':dict(notes)}

if __name__=='__main__':
 tic=time.time();results={}
 for platform in ('JAVA','RUST'):
  for h in (1024,2032):
   key=f'{platform}_{h}';print('BUILD',key,flush=True)
   results[key]=build_one(platform,h)
   print('  OK',results[key]['entry_count'],results[key]['bytes'],'SHA256',results[key]['sha256'][:16],flush=True)
 report={'status':'PRIVATE STATIC INTEGRATION CANDIDATE; NO HORIZON/JAVA RUNTIME TEST',
         'input_archive_versions':'Thalvoryn V67 Nether Rebalanced',
         'external_sources':{k:str(v) for k,v in INPUTS.items()},'module_counts':MODULE_LOG,
         'generated_template_geometry':GEOMETRY,'variants':results,'wall_clock_seconds':round(time.time()-tic,2),
         'known_exclusions':['abridged:bridge uses lithostitched:delegating and requires a non-datapack mod',
                             'end-castle and end-island-only-50-done are Minecraft world saves with MCA region files, not datapacks',
                             'Stellarity modern-root End biome and density now activated as EXPERIMENTAL; actual dragon arena untested',
                             'Dungeons & Taverns native Ancient City Overhaul 26.3 applied as experiment; actual placement untested',
                             'No runtime test; some imported structures could still have placement or version errors']}
 (WD/'v67_integration_manifest.json').write_bytes(JSON(report))
 print('DONE',round(time.time()-tic,1),flush=True)