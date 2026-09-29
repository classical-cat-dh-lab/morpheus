#!/usr/bin/env python3
"""Rasterize the product's simple geometric M mark for PWA icon sizes."""
from pathlib import Path
import struct
import zlib
ROOT = Path(__file__).resolve().parents[1]
POLYGON = [(35,146),(35,46),(55,46),(96,110),(137,46),(157,46),(157,146),(138,146),(138,79),(96,141),(54,79),(54,146)]
def inside(x, y):
    result = False
    for (ax,ay),(bx,by) in zip(POLYGON, POLYGON[1:]+POLYGON[:1]):
        if (ay>y)!=(by>y) and x<(bx-ax)*(y-ay)/(by-ay)+ax:
            result=not result
    return result
def chunk(name, data):
    return struct.pack('!I',len(data))+name+data+struct.pack('!I',zlib.crc32(name+data))
for size in (180,192,512):
    rows=[]
    for y in range(size):
        row=bytearray([0])
        for x in range(size):
            row.extend((244,245,240) if inside((x+.5)*192/size,(y+.5)*192/size) else (102,2,60))
        rows.append(row)
    png=b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('!2I5B',size,size,8,2,0,0,0))+chunk(b'IDAT',zlib.compress(b''.join(rows)))+chunk(b'IEND',b'')
    (ROOT/'browser'/f'icon-{size}.png').write_bytes(png)
