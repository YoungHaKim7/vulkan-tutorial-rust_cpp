# fix
Fixed and verified — the app now runs cleanly and renders the engine model. Three changes:

## Root cause of the SIGSEGV

`2Cylinder.gltf` references `2CylinderEngine0.bin` (1,794,612 bytes), but **that file was never in `assets/`** — only `2Cylinder.gltf` was copied. So:

1. `file_read_binary()` returned `{nullptr, 0}` (`raptor/foundation/file.cpp:385` — null on failure, no error printed)
2. `main.cpp` pushed that `nullptr` into `buffers_data`
3. `get_buffer_data()` computed `nullptr + byteOffset(1340232)` → `GpuDevice::create_buffer()` did `memcpy(dst, invalid_ptr, 454380)` → SIGSEGV (backtrace confirmed: `__memcpy_avx_unaligned_erms` ← `GpuDevice::create_buffer` ← `main`)

## Fixes

| File                                  | Change                                                                                                                                |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `assets/2CylinderEngine0.bin`         | Downloaded from the archived Khronos `glTF-Sample-Models` repo — size matches the glTF's declared `byteLength` exactly                |
| `src/main.cpp:277`                    | Null-check after `file_read_binary` → prints `Error: cannot read binary file '...' referenced by '...'` and exits instead of crashing |
| `raptor/graphics/gpu_device.cpp:1797` | Removed the init-time manual layout transition of all swapchain images in `vulkan_create_swapchain_pass`                              |

The **validation warnings** (`UNASSIGNED-non-acquired-swapchain-image-used`) came from that third item: the spec forbids touching presentable images outside the acquire→present window. The transition was redundant anyway — the swapchain render pass already declares `initialLayout = UNDEFINED` / `finalLayout = PRESENT_SRC_KHR` and performs the transition per-frame, legally.

## Verification

- Ran ~6 s under gdb-free run: **zero validation errors**, window opened, clean shutdown (`HeapAllocator Shutdown - all memory free!`)
- Screenshot confirms the **2CylinderEngine model is rendering** (visible engine geometry against the green clear color) with the ImGui panels
- Missing-file error path tested: clear message, clean exit, no crash
- Rebuilt with `make -j`; note CMake only copies `assets/` at configure time, so I `cp`'d the `.bin` into `build/bin/assets/` manually

You already committed these changes yourself (`8c30ad9`) — heads up that commit also picked up `imgui.ini` from the repo root, which the app regenerated when I ran it from there.

# debugging

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


   python3 - <<'EOF'
   from PIL import Image
   img = Image.open('/tmp/vk_shot.png')
   # Raptor Test window on left monitor: approx x 100..1400, y 130..980 of full image
   crop = img.crop((80, 120, 1450, 1000))
   crop.save('/tmp/vk_window.png')
   print(crop.size)
   EOF
   Crop the Raptor Test window


```



