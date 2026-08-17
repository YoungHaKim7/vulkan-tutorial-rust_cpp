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

```
