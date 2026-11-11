import zipfile,sys,json,collections
z=zipfile.ZipFile(sys.argv[1]); names=set(z.namelist())
errors=collections.defaultdict(list)
for f in sorted(n for n in names if '/worldgen/template_pool/' in n and n.endswith('.json')):
 v=json.loads(z.read(f))
 def walk(e):
  if not isinstance(e,dict):return
  t=e.get('element_type')
  if t=='minecraft:list_pool_element':
   for el in e.get('elements',[]):walk(el)
  elif t in ('minecraft:single_pool_element','minecraft:legacy_single_pool_element'):
   pro=e.get('processors')
   if isinstance(pro,str) and pro and not pro.startswith('minecraft:'):
    ns,p=pro.split(':',1);path=f'data/{ns}/worldgen/processor_list/{p}.json'
    if path not in names:errors['missing_processor'].append((f,pro))
  elif t=='minecraft:feature_pool_element':
   p=e.get('feature')
   if isinstance(p,str) and not p.startswith('minecraft:'):
    ns,x=p.split(':',1);path=f'data/{ns}/worldgen/placed_feature/{x}.json'
    if path not in names:errors['missing_feature'].append((f,p))
 for el in v.get('elements',[]):walk(el.get('element'))
print('errors', {k:len(v) for k,v in errors.items()})
for k,v in errors.items():print(k, v[:25])