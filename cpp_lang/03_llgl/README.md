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

# Install LLGL

```bash
  mkdir build && cd build
  cmake -DCMAKE_BUILD_TYPE=Release \
        -DLLGL_BUILD_RENDERER_OPENGL=ON \
        -DLLGL_BUILD_WRAPPER_C99=ON \
        -DGaussLib_INCLUDE_DIR:STRING=/path/to/GaussianLib/include \
        ..
  cmake --build .
```

# example  보기

Your examples-enabled build is in `build_local/` (the plain `build/` only has the library). So far only one example binary exists there:

**Run the one that's built:**
```fish
./RunExamplesLinux.sh build_local/build
```
That gives you an interactive menu (currently just `Animation`). Equivalently, run it directly:
```fish
cd examples/Cpp/Animation
../../../build_local/build/Example_Animation
```
The `cd` matters — examples load their media (textures, meshes) via relative paths.

**Build the rest** — your cmake config has `LLGL_BUILD_EXAMPLES=ON`, so everything in `examples/Cpp/` (HelloTriangle, Texturing, PBR, ShadowMapping, PostProcessing, …) is available as a target:
```fish
cmake --build build_local -j
```
That builds all of them (`MultiRenderer` is excluded from the Linux runner script; `HelloUWP`/`HelloOpenXR` are Windows/OpenXR-specific). Then re-run `RunExamplesLinux.sh build_local/build` and the menu will list all of them.

A good first one to try once built: `HelloTriangle` — it's the minimal "is my renderer working" sanity check.

# 모든 예시 다 빌드

Examples are already included by default — nothing extra needed:

```bash
./BuildLinux.sh
```

The script sets `LLGL_BUILD_EXAMPLES=ON` unless you pass `--no-examples`, so this builds all ~24 C++ examples (`HelloTriangle`, `PBR`, `ShadowMapping`, `Texturing`, …) along with the library. Each becomes a binary named `Example_<Name>` in `build_linux/build/` (Debug builds get a `D` suffix, e.g. `Example_HelloTriangleD`).

Your existing `build_local/` dir also has examples enabled (`LLGL_BUILD_EXAMPLES=ON`, tests off), so to just build there:

```bash
cmake --build build_local -j$(nproc)
```

If configuring CMake manually instead, the flag is on you — it defaults to OFF in raw CMake:

```bash
cmake -DLLGL_BUILD_EXAMPLES=ON ...
```

Two caveats:

- **Run them from their source directory** — examples load shaders/textures via relative paths. Easiest is the included picker, which handles the `cd` for you:
  ```bash
  ./RunExamplesLinux.sh          # interactive menu
  ./RunExamplesLinux.sh -- ...   # pass args to the example
  ```
  Or manually: `cd examples/Cpp/HelloTriangle && ../../build_linux/build/Example_HelloTriangle`
- `MultiRenderer` is excluded from the Linux run list, and `HelloUWP` is Windows/UWP-only — that's expected, not a build failure.
