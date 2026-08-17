# debug

```bash
$ gdb -batch -ex run -ex "bt 25" -ex "info registers rip" --args ./build/bin/vulkan001 ./build/bin/assets/2Cylinder.gltf 2>&1 | tail -60
```

# bytes buffer check

```bash
 python3 -c "
   import json
   with open('build/bin/assets/2Cylinder.gltf') as f:
       d = json.load(f)
   print('buffers:', d.get('buffers'))
   print('bufferViews:')
   for i, bv in enumerate(d.get('bufferViews', [])):
       print(' ', i, bv)
   "

grep -rn "InjectDefault3DModel" --include="*.cpp" --include="*.hpp" . | grep -v build; python3 -c "
   import json
   d = json.load(open('assets/2Cylinder.gltf'))
   # check index accessor component types and that all primitives have materials
   bad = 0
   for m in d['meshes']:
       for p in m['primitives']:
           if 'material' not in p: bad += 1
   print('primitives without material:', bad)
   a = d['accessors']
   import collections
   icts = collections.Counter(d['accessors'][p['indices']]['componentType'] for m in d['meshes'] for p in m['primitives'])
   print('index componentTypes:', dict(icts))
   print('scene default:', d.get('scene'))
   "
   Check InjectDefault3DModel and glTF index types

python3 - <<'EOF'
   import struct
   with open('/tmp/vk.xwd','rb') as f:
       data = f.read()
   hdr = struct.unpack('>25I', data[:100])
   header_size, version, fmt, depth, w, h, xoff, byte_order, unit, bit_order, pad, bpp, bpl = hdr[:13]
   print('size', w, h, 'depth', depth, 'bpp', bpp, 'bpl', bpl, 'header_size', header_size)
   from PIL import Image
   off = header_size
   px = data[off:off + bpl*h]
   img = Image.frombytes('RGBX' if bpp==32 else 'RGB', (w,h), px, 'raw', 'BGRX' if bpp==32 else 'BGR', bpl)
   img = img.convert('RGB')
   # crop center-ish where window likely is (1280x800 window)
   img.save('/tmp/vk_shot.png')
   print('saved /tmp/vk_shot.png')
   EOF
   Decode XWD screenshot to PNG

  python3 - <<'EOF'
   from PIL import Image
   img = Image.open('/tmp/vk_shot.png')
   w, h = img.size
   # left quarter, mid, right
   img.crop((0, 0, w//3, h)).save('/tmp/vk_left.png')
   img.crop((w//3, 0, 2*w//3, h)).save('/tmp/vk_mid.png')
   img.crop((2*w//3, 0, w, h)).save('/tmp/vk_right.png')
   print('done')
   EOF
   Crop screenshot into three sections

```



