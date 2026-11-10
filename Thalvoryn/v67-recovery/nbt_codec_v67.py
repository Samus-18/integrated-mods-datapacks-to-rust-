"""Pure-Python Java-edition NBT codec for private Thalvoryn static generation.
Retains binary field types; only edits selected structure templates.
"""
from __future__ import annotations
from dataclasses import dataclass
import struct, gzip
@dataclass
class Tag:
    typ:int
    value:object

def t(typ,v): return Tag(typ,v)

def read_nbt(blob:bytes):
    b=memoryview(gzip.decompress(blob) if blob[:2]==b'\x1f\x8b' else blob);offset=0
    def rd(n):
        nonlocal offset
        d=b[offset:offset+n].tobytes(); offset+=n
        if len(d)!=n:raise ValueError('truncated NBT')
        return d
    def val(tp):
        if tp==1:return struct.unpack('>b',rd(1))[0]
        if tp==2:return struct.unpack('>h',rd(2))[0]
        if tp==3:return struct.unpack('>i',rd(4))[0]
        if tp==4:return struct.unpack('>q',rd(8))[0]
        if tp==5:return struct.unpack('>f',rd(4))[0]
        if tp==6:return struct.unpack('>d',rd(8))[0]
        if tp==7:return rd(struct.unpack('>i',rd(4))[0])
        if tp==8:
            length=struct.unpack('>H',rd(2))[0];return rd(length).decode('utf-8')
        if tp==9:
            subtype=rd(1)[0];n=struct.unpack('>i',rd(4))[0]
            if n<0 or n>10_000_000:raise ValueError('bad list length')
            return subtype,[val(subtype) for _ in range(n)]
        if tp==10:
            d={}
            while (kind:=rd(1)[0])!=0:
                key=val(8);d[key]=Tag(kind,val(kind))
            return d
        if tp==11:
            n=struct.unpack('>i',rd(4))[0];return list(struct.unpack(f'>{n}i',rd(n*4))) if n else []
        if tp==12:
            n=struct.unpack('>i',rd(4))[0];return list(struct.unpack(f'>{n}q',rd(n*8))) if n else []
        raise ValueError('bad NBT tag '+str(tp))
    if rd(1)[0]!=10:raise ValueError('NBT root not compound')
    root_name=val(8);root=val(10)
    if offset!=len(b):raise ValueError('Trailing data '+str(len(b)-offset))
    return root_name,root

def write_nbt(name:str,root:dict,compressed=True):
    def s(txt):
        data=txt.encode('utf-8');return struct.pack('>H',len(data))+data
    def enc(tp, v):
        if tp in (1,2,3,4,5,6):return struct.pack({1:'>b',2:'>h',3:'>i',4:'>q',5:'>f',6:'>d'}[tp],v)
        if tp==7:return struct.pack('>i',len(v))+bytes(v)
        if tp==8:return s(v)
        if tp==9:
            sub,els=v;return bytes([sub])+struct.pack('>i',len(els))+b''.join(enc(sub,e) for e in els)
        if tp==10:return b''.join(bytes([item.typ])+s(key)+enc(item.typ,item.value) for key,item in v.items())+b'\x00'
        if tp==11:return struct.pack('>i',len(v))+struct.pack('>'+str(len(v))+'i',*v)
        if tp==12:return struct.pack('>i',len(v))+struct.pack('>'+str(len(v))+'q',*v)
        raise ValueError('bad NBT tag '+str(tp))
    raw=b'\x0a'+s(name)+enc(10,root)
    return gzip.compress(raw,compresslevel=6,mtime=0) if compressed else raw

def basic_template(size,palette,voxels,entities=None,dataversion=4435):
    pentries=[{'id':t(8,x)} for x in palette]
    blocks=[{'pos':t(9,(3,list(pos))),'state':t(3,state)} for pos,state in sorted(voxels.items())]
    root={'DataVersion':t(3,dataversion),'size':t(9,(3,list(size))),'palette':t(9,(10,pentries)),'blocks':t(9,(10,blocks)),'entities':t(9,(10,entities or []))}
    return write_nbt('',root)

if __name__=='__main__':
    import glob,zipfile
    z=zipfile.ZipFile(glob.glob('/mnt/data/Thalvoryn_V67_work/*JAVA*1024*REBALANCED*')[0]);n='data/thalvoryn/structure/overworld/imported/buildings_02.nbt';raw=z.read(n)
    nm,r=read_nbt(raw);out=write_nbt(nm,r);_,re=read_nbt(out)
    assert re==r
    print('roundtrip',len(raw),len(out),'size',r['size'].value,'blocks',len(r['blocks'].value[1]),'palette',len(r['palette'].value[1]))