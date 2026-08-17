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


```

