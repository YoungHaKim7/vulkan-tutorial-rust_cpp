# Build

```bash
mkdir build && cd build
cmake -DRAPTOR_GRAPHICS_PATH=/path/to/graphics ..
cmake --build .

```

# Fix

- Build successful! The `vulkan001` executable has been created at `build/bin/vulkan001`.

## Summary of fixes applied:

1. **Missing graphics library** - Cloned `PacktPublishing/Mastering-Graphics-Programming-with-Vulkan` repo and copied the `graphics` folder from `source/chapter1/` to `raptor/graphics/`

2. **Platform-specific Vulkan headers** - Modified `raptor/graphics/gpu_device.hpp` to use `VK_USE_PLATFORM_MACOS_MVK` and `VK_USE_PLATFORM_METAL_EXT` on macOS instead of X11 headers

3. **ARM64/x86 SIMD compatibility** - Fixed `external/wyhash.h` to only include x86 intrinsics (`immintrin.h`) on x86 platforms, avoiding conflicts on ARM64

4. **ARM64 NEON intrinsics** - Added ARM NEON SIMD support to `raptor/foundation/hash_map.hpp` as an alternative to SSE2 for Apple Silicon

5. **Missing sources** - Added `external/tlsf.c` and ImGui sources to `CMakeLists.txt`
