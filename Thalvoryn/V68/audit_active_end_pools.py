import zipfile,glob,json,sys,collections,time
from nbt_codec_v67 import read_nbt
p=glob.glob('/mnt/data/v68_work/*RUST*1024*.zip')[0]; z=zipfile.ZipFile(p); names=set(z.namelist())
J=lambda path:json.loads(z.read(path))
def mkpath(x,t):
  ns,_,name=x.partition(':');name=name or ns;ns=ns if _ else 'minecraft';return f'data/{ns}/{t}/{name}'
active=[]
for x in names:
 if '/worldgen/structure_set/' not in x or not x.endswith('.json'):continue
 setdef=J(x)
 if '/ecology/end_' not in x:continue
 active.extend(e['structure'] for e in setdef['structures'])
roots=[]
for s in active:
 p=mkpath(s,'worldgen/structure')+'.json'
 if p not in names:continue
 v=J(p)
 if v.get('type')=='minecraft:jigsaw':roots.append((s,v.get('start_pool','')))
print('roots',len(roots))
queue=collections.deque([(id,pool,0) for id,pool in roots]);visited=set();adj=collections.defaultdict(set);hits=[];missing=[];tcount=0;connect=0
start=time.time()
while queue and len(visited)<1200 and time.time()-start<45:
 origin,pool,depth=queue.popleft()
 if (origin,pool) in visited or depth>20:continue
 visited.add((origin,pool))
 if any(x in pool.lower() for x in ['ship','boat','pirate','galleon','frigate','vessel','dock']):hits.append((origin,depth,pool,'pool'))
 path=mkpath(pool,'worldgen/template_pool')+'.json'
 if path not in names:
  if pool not in ('minecraft:empty',''):
   missing.append((origin,pool))
  continue
 doc=J(path)
 fallback=doc.get('fallback')
 if fallback not in (None,'minecraft:empty','') :queue.append((origin,fallback,depth+1))
 for e in doc.get('elements',[]):
  obj=e.get('element',{})
  sublist=obj.get('elements') if obj.get('element_type','').endswith('list_pool_element') else [obj]
  for ob in (sublist or []):
   loc=ob.get('location') or ob.get('name')
   if not isinstance(loc,str):continue
   if any(x in loc.lower() for x in ['ship','boat','pirate','galleon','frigate','vessel','dock']):hits.append((origin,depth,pool,loc))
   np=mkpath(loc,'structure')+'.nbt'
   if np not in names:continue
   try:_,nbt=read_nbt(z.read(np))
   except Exception as ex:print('decode fail',np,str(ex));continue
   tcount+=1
   for b in nbt.get('blocks',[]).value[1]:
    ent=b.get('nbt');
    if ent and isinstance(ent.value,dict):
     nxt=ent.value.get('pool')
     if nxt is not None and isinstance(nxt.value,str) and nxt.value not in ('minecraft:empty',''):
      connect+=1;queue.append((origin,nxt.value,depth+1))
print('visited',len(visited),'templates',tcount,'connect',connect,'missing',len(missing),'hits',len(hits),'seconds',round(time.time()-start,1))
print('missing sample:',missing[:20]);print('hits sample:',hits[:80]);
print('active with suspect boats:',collections.Counter(x[0] for x in hits))