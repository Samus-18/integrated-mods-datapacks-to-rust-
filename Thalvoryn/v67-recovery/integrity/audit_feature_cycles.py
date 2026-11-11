import zipfile,json,glob,collections,sys
f=sys.argv[1] if len(sys.argv)>1 else glob.glob('/mnt/data/thalvoryn_v67_hardfix/V67_INTEGRITY_TEST/Thalvoryn-RUST-Nether1024*.zip')[0]
z=zipfile.ZipFile(f);names=set(z.namelist())
biomefiles={}
for path in (n for n in names if '/worldgen/biome/' in n and n.endswith('.json')):
 b=path.split('/')[1]+':'+path.split('/worldgen/biome/')[1][:-5];biomefiles[b]=json.loads(z.read(path))
def src_biomes(dim):
 src=json.loads(z.read('data/minecraft/dimension/'+dim+'.json'))['generator']['biome_source']
 if src['type']=='minecraft:multi_noise' and 'biomes' in src:return {b['biome'] for b in src['biomes']}
 if src['type']=='minecraft:the_end':return {'minecraft:the_end','minecraft:end_highlands','minecraft:end_midlands','minecraft:end_barrens','minecraft:small_end_islands'}
 return set(biomefiles)
for dim in ('overworld','the_nether','the_end'):
 edges=[collections.defaultdict(set) for _ in range(12)];active=src_biomes(dim);known=0
 for biome in active:
  v=biomefiles.get(biome)
  if v is None:continue
  known+=1
  for step,order in enumerate(v.get('features',[])):
   if step>=len(edges):continue
   for a,b in zip(order,order[1:]):edges[step][a].add(b)
 cycles=[]
 for i,graph in enumerate(edges):
  colors={}
  def dfs(n):
   if colors.get(n)==1:cycles.append((i,n));return
   if colors.get(n)==2:return
   colors[n]=1
   for k in graph.get(n,[]):dfs(k)
   colors[n]=2
  for node in list(graph):dfs(node)
 print(dim,'source_biomes',len(active),'defined_in_pack',known,'cycles',cycles[:10]);assert not cycles
print('PASS where biome definitions bundled; vanilla built-in default biome order not exhaustively checked')