# LLGL
- Low Level Graphics Library (LLGL) is a thin abstraction layer for the modern graphics APIs OpenGL, Direct3D, Vulkan, and Metal
  - https://github.com/LukasBanana/LLGL

```bash
  # this is heuristically generated, and may not be correct
  find_package(LLGL CONFIG REQUIRED)
  target_link_libraries(main PRIVATE LLGL::LLGL LLGL::LLGL_Null)

Completed submission of vcpkg-cmake-config:x64-linux@2026-07-21 to 1 binary cache(s) in 2.49 ms
Waiting for 1 remaining binary cache submissions...
Completed submission of llgl:x64-linux@2023-03-05#1 to 1 binary cache(s) in 374 ms (1/1)
All requested installations completed successfully in: 9.2 s
```

`Example.cpp` is the `Example_Animation` CMake target — it links the shared `ExampleBase` library and needs [GaussianLib](external/GaussianLib) (it uses `Gs::Vector2f`/`Gs::Matrix4f`). Your submodule isn't initialized yet (empty `external/GaussianLib/`), so do that first.

**Option 1 — convenience script (does everything, incl. fetching GaussianLib if the submodule is missing):**

```fish
cd /home/gy/my_projects/Cpp_Lang/LLGL
./BuildLinux.sh
```

This configures `build_linux/` with `LLGL_BUILD_EXAMPLES=ON` + OpenGL renderer and builds. To build just this example afterwards:

```fish
cmake --build build_linux --target Example_Animation
```

**Option 2 — plain CMake:**

```fish
cd /home/gy/my_projects/Cpp_Lang/LLGL
git submodule update --init --recursive
cmake -S . -B build_local -DLLGL_BUILD_EXAMPLES=ON \
      -DGaussLib_INCLUDE_DIR=$PWD/external/GaussianLib/include
cmake --build build_local --target Example_Animation
```

(The `GaussLib_INCLUDE_DIR` is required — `examples/Cpp/CMakeLists.txt:94` fatals without it. `BuildLinux.sh` passes it for you.)

Notes:
- Linux needs the X11/Xrandr/OpenGL dev packages installed (the script validates this; skip with `-S`).
- Run it from its own folder so it finds its shaders: `cd examples/Cpp/Animation && ../../../build_linux/Example_Animation` — or use `./RunExamplesLinux.sh Animation` from the repo root, which handles that.
