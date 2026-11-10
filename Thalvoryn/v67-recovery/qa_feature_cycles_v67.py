from pathlib import Path
import zipfile,json,collections
P=next(Path(__file__).resolve().parent.glob('*RUST-Nether1024-V67-INTEGRATION-TEST*.zip'))
with zipfile.ZipFile(P) as z:
 biomes={n:json.loads(z.read(n)) for n in z.namelist() if '/worldgen/biome/' in n and n.endswith('.json')}
 dims=[json.loads(z.read(f'data/minecraft/dimension/{dim}.json')) for dim in ('overworld','the_nether','the_end')]
 def resource_id(p):return p.split('/')[1]+':'+p.split('/worldgen/biome/')[1][:-5]
 byid={resource_id(p):d for p,d in biomes.items()}
 active=set()
 for d in dims:
  source=d['generator']['biome_source'];active.update(v['biome'] for v in source.get('biomes',[]))
 if not active:
  active.update(byid)
 print('active biome IDs',len(active),'present',sum(x in byid for x in active))
 adj=collections.defaultdict(set);sources=collections.defaultdict(set)
 for name in active:
  b=byid.get(name)
  if b is None:continue
  sections=b.get('features',[])
  flat=[v for layer in sections for v in layer if isinstance(v,str)]
  for a,c in zip(flat,flat[1:]):
   if a!=c:adj[a].add(c);sources[(a,c)].add(name)
 indeg=collections.Counter()
 for a,edges in adj.items():
  indeg.setdefault(a,0)
  for b in edges:indeg[b]+=1
 q=collections.deque(a for a in indeg if indeg[a]==0);n=0
 while q:
  a=q.popleft();n+=1
  for b in adj[a]:
   indeg[b]-=1
   if indeg[b]==0:q.append(b)
 remaining=[k for k,v in indeg.items() if v>0]
 print('order nodes',len(indeg),'traversed',n,'cycles',len(remaining),'examples',remaining[:30])
 output={'active_biomes':sorted(active),'nodes':len(indeg),'cycle_nodes':remaining[:100],'cycle_count':len(remaining)}
 if remaining:
  output['cycle_source_examples']={k:[{'target':b,'in_biomes':list(sources[k,b])[:10]} for b in adj[k] if b in remaining][:10] for k in remaining[:12]}
 (P.parent/'v67_feature_order_qa.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n')