# debug

```bash
$ gdb -batch -ex run -ex "bt 25" -ex "info registers rip" --args ./build/bin/vulkan001 ./build/bin/assets/2Cylinder.gltf 2>&1 | tail -60
```
