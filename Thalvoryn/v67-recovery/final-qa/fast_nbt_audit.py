#!/usr/bin/env python3
"""Fast streaming NBT audit for Minecraft structure containers. Static; no game tests."""
from __future__ import annotations
import gzip, zlib, zipfile, pathlib, struct, collections, json, time, sys
BASE=pathlib.Path(__file__).resolve().parent
SOURCE=next(BASE.glob('*RUST*1024*ECOLOGY*'))
class Scanner:
 def __init__(self,blob):
  if blob.startswith(b'\x1f\x8b'): blob=gzip.decompress(blob)
  elif len(blob)>2 and blob[0]==0x78:blob=zlib.decompress(blob)
  self.b=memoryview(blob);self.i=0;self.length=len(blob)
 def U8(self):
  k=self.b[self.i];self.i+=1;return k
 def S16(self):
  n=struct.unpack_from('>H',self.b,self.i)[0];self.i+=2;return n
 def I32(self):
  n=struct.unpack_from('>i',self.b,self.i)[0];self.i+=4;return n
 def STRING(self):
  n=self.S16();v=self.b[self.i:self.i+n].tobytes().decode('utf-8','replace');self.i+=n;return v
 def SKIP(self,t):
  if t in (1,2,3,4,5,6):self.i+={1:1,2:2,3:4,4:8,5:4,6:8}[t];return
  if t==7:
   n=self.I32();self.i+=n;return
  if t==8:
   n=self.S16();self.i+=n;return
  if t==9:
   sub=self.U8();n=self.I32();
   if sub in (1,2,3,4,5,6):self.i+=n*{1:1,2:2,3:4,4:8,5:4,6:8}[sub]
   else:
    for _ in range(n):self.SKIP(sub)
   return
  if t==10:
   while True:
    tp=self.U8()
    if not tp:break
    self.SKIP(8);self.SKIP(tp)
   return
  if t in (11,12):
   n=self.I32();self.i+=n*(4 if t==11 else 8);return
  raise ValueError('Invalid tag '+str(t))
 def read_palette(self):
  et=self.U8();n=self.I32();items=[]
  if et!=10:raise ValueError('palette wrong type '+str(et))
  for _ in range(n):
   block=''
   while (t:=self.U8())!=0:
    key=self.STRING()
    if key in ('Name','id') and t==8:block=self.STRING()
    else:self.SKIP(t)
   items.append(block)
  return items
 def read_blocks(self):
  et=self.U8();n=self.I32();counts=collections.Counter(); inventories=collections.Counter(); bad=[];states=collections.Counter(); t0=time.monotonic()
  if et!=10:raise ValueError('blocks wrong type '+str(et))
  for _ in range(n):
   state=None;loot='';has_items=False;item_len=0;block_nbt=False;pos=(0,0,0)
   while (t:=self.U8())!=0:
    key=self.STRING()
    if key=='state' and t==3:state=self.I32()
    elif key=='nbt' and t==10:
     block_nbt=True
     while (t2:=self.U8())!=0:
      key2=self.STRING()
      if key2=='LootTable' and t2==8:loot=self.STRING()
      elif key2=='Items' and t2==9:
       elem=self.U8();item_len=self.I32();has_items=True
       for _ in range(item_len):self.SKIP(elem)
      else:self.SKIP(t2)
    else:self.SKIP(t)
   if state is not None:
    states[state]+=1
    if loot:inventories[(state,'loot',loot)]+=1
    elif has_items:inventories[(state,'items',item_len)]+=1
    elif block_nbt:inventories[(state,'bare_nbt',None)]+=1
    else:inventories[(state,'no_nbt',None)]+=1
  return states,inventories,n
 def RUN(self):
  if self.U8()!=10:raise ValueError('root not compound')
  self.STRING()
  pal=[];states=collections.Counter();inv=collections.Counter();size=[];block_count=0
  while (t:=self.U8())!=0:
   name=self.STRING()
   if name=='palette' and t==9:pal=self.read_palette()
   elif name=='blocks' and t==9:states,inv,block_count=self.read_blocks()
   elif name=='size' and t==9:
    sub=self.U8();n=self.I32()
    if sub==3:size=[self.I32() for _ in range(n)]
    else:
     for _ in range(n):self.SKIP(sub)
   else:self.SKIP(t)
  if self.i!=self.length:raise ValueError(f'NBT trailing bytes {self.length-self.i}')
  return pal,states,inv,size,block_count

def go():
 with zipfile.ZipFile(SOURCE) as z:
  names=sorted(x for x in z.namelist() if x.endswith('.nbt') and '/structure/' in x)
  names=names[int(sys.argv[1]):int(sys.argv[2])] if len(sys.argv)>2 else names
  loot={x.split('/')[1]+':'+x.split('/loot_table/')[1][:-5] for x in z.namelist() if x.endswith('.json') and '/loot_table/' in x}
  results={'source':str(SOURCE),'static_only':True,'scanned_templates':0,'decoded_blocks':0,'container_types':collections.Counter(),'container_inventory':collections.Counter(),'loot_id_frequency':collections.Counter(),'missing_custom_loot':[],'bad_nbt':[],'bad_state':[],'unfilled_containers_by_template':[],'container_scenarios':{},'interesting_shapes':{}}
  t0=time.monotonic()
  CONTAINER={'minecraft:chest','minecraft:trapped_chest','minecraft:barrel','minecraft:dispenser','minecraft:dropper','minecraft:hopper','minecraft:shulker_box','minecraft:decorated_pot'}
  for idx,n in enumerate(names):
   try:
    pal,states,inv,size,block_count=Scanner(z.read(n)).RUN()
    if states and max(states)>=len(pal):results['bad_state'].append([n,len(pal),max(states)])
    counts=collections.Counter();missing=collections.Counter();examples=[]
    for (s,status,data),num in inv.items():
     if s>=len(pal):continue
     typ=pal[s]
     if typ in CONTAINER:
      results['container_types'][typ]+=num
      results['container_inventory'][status]+=num
      counts[(typ,status)]+=num
      if status=='loot':
       results['loot_id_frequency'][data]+=num
       if data not in loot and not data.startswith('minecraft:'):results['missing_custom_loot'].append([n,typ,data,num])
      elif status in ('no_nbt','bare_nbt','items') and (status!='items' or data==0):missing[typ]+=num
    results['decoded_blocks']+=block_count
    if missing:results['unfilled_containers_by_template'].append([n,dict(missing)])
    if 'air_vessel' in n or 'v67_cave_tunnel_' in n or 'v67_sky_isle_' in n or 'end_castle' in n:results['interesting_shapes'][n]={'size':size,'blocks':block_count,'palette':len(pal),'containers':{str(x):v for x,v in counts.items()}}
    results['scanned_templates']+=1
   except Exception as e:
    results['bad_nbt'].append([n,str(e)])
   if idx%600==0:
    print('progress',idx+1,'/',len(names),'in',round(time.monotonic()-t0,1),'sec',flush=True)
  results['seconds']=round(time.monotonic()-t0,1)
  out=BASE/(f'V67_FAST_NBT_AUDIT_{sys.argv[1]}_{sys.argv[2]}.json' if len(sys.argv)>2 else 'V67_FAST_NBT_AUDIT.json');out.write_text(json.dumps(results,indent=2,ensure_ascii=False,default=lambda x:dict(x) if isinstance(x,collections.Counter) else x)+'\n')
  print('REPORT',out)
  print(json.dumps({k:results[k] for k in ['scanned_templates','decoded_blocks','container_types','container_inventory','missing_custom_loot','bad_nbt','bad_state','seconds']},indent=2,default=dict))
if __name__=='__main__':go()