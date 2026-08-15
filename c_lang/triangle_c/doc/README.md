# fix
Done — exit 124 means the app ran until the timeout killed it (a missing shader or failed adapter request would have aborted immediately). The triangle window was up and rendering.

## What was wrong

The project was a mix of three incompatible things:

1. **`src/main.cpp` was actually C code** — byte-identical (modulo clang-format) to the [wgpu-native v29.0.1.1](https://github.com/gfx-rs/wgpu-native/tree/v29.0.1.1/examples/triangle) triangle example. With `project(... LANGUAGES C)` and a `.cpp` source, CMake had no enabled language for the file → *"can not determine linker language"*.
2. **The top-level `CMakeLists.txt` was a stale Vulkan C++ template** (Vulkan::Vulkan, GLM, VKB flags) that had nothing to do with the wgpu code, and none of the wgpu headers/library existed on the system.
3. **The justfile** compiled with `g++ -std=c++20` (compound literals in `main.c` won't compile as C++) and linked only Vulkan/GLFW.

## Changes

- **`src/main.cpp` → `src/main.c`** — it's the wgpu-native C example.
- **`include/vendor/ffi/`** — vendored from the wgpu-native **v29.0.1.1** release zip (matches your example code): `wgpu.h`, `webgpu-headers/webgpu.h`, and the prebuilt `lib/libwgpu_native.so` (that's what the empty `include/vendor/` was for).
- **`CMakeLists.txt`** — rewritten: builds `framework` (`include/framework/framework.c`) + the executable from `src/main.c`, links `libwgpu_native.so`, `PkgConfig::GLFW`, `PkgConfig::WAYLAND` (openSUSE puts `wayland-client.h` under `/usr/include/wayland`, which `glfw3native.h` needs), sets the platform `GLFW_EXPOSE_NATIVE_*` defines, adds an rpath to the vendored lib, and copies `shader.wgsl` next to the binary.
- **`justfile`** — `r` now uses `gcc -std=c11` with the wgpu include/link paths and compiles `framework.c` too; `cr`/`cro` drop the stale `VKB_WSI_SELECTION` flag and `cd` into the output dir (the app loads `shader.wgsl` from the cwd); `fm` now excludes `vendor/` so clang-format doesn't churn the vendored headers.

Verified: `cmake -G Ninja` configure + build clean (only upstream warnings), `ldd` resolves everything, and the app runs. `just cr` will now build and launch it.

The CMakeLists is fixed. Summary of what changed:

1. **`find_package(Vulkan REQUIRED)` moved before `add_executable`** and linked as `Vulkan::Vulkan` so include dirs come with the target.
2. **GLFW found via its CMake config** (`find_package(glfw3)`), with a pkg-config fallback — no more linking the bare `glfw` library name with no include path, which was the actual cause of your error.
3. **GLM found via `find_package(glm)` / `find_path`** — you're including `<glm/mat4x4.hpp>` but the old file never found GLM at all, so it would have been the next fatal error after GLFW.
4. Removed the pointless `include_directories(/usr/include)` / `link_directories(/usr/lib)` (the compiler/linker search those by default) and the duplicated platform link blocks. `X11` was dropped from the Linux link because the GLFW target/pkg-config already carries its own dependencies (and openSUSE's libglfw supports Wayland).

One thing still blocks the build, and CMake can't fix it — the packages genuinely aren't on your system (no `/usr/include/GLFW`, no pkg-config entry, no library in `ldconfig`). Run:

```
! sudo zypper in libglfw-devel glm-devel
```

then `just cr` and it should build. If you'd rather not install system-wide, the alternative is `FetchContent` for glfw/GLM in CMake — say the word and I'll switch it over.

# fix 2

- Fixed and verified. All artifacts now live only in `build/`, and the source directory stays clean — no more `mv` hack.

## What was wrong

The justfile ran `cmake .` **in-source** (polluting the repo with `build.ninja`, `CMakeCache.txt`, etc.), then patched it up afterwards with `mv build.ninja CMakeCache.txt ... build`. `cro`/`cro3` also pointed at a wrong binary path (`./build/target/main` — never existed), and `CMakeLists.txt:1` had a malformed version `VERSION 4.01` (CMake version components can't have leading zeros).

## Changes

**`CMakeLists.txt`:**
- `cmake_minimum_required(VERSION 4.01)` → `VERSION 3.16` (valid form, well above CMake 4.x's 3.5 floor)
- pkg-config GLFW fallback: `pkg_check_modules(GLFW REQUIRED IMPORTED_TARGET glfw3)` + link `PkgConfig::GLFW` instead of manually copying `GLFW_LINK_LIBRARIES`/`GLFW_INCLUDE_DIRS` (the imported target carries libs, includes, *and* cflags)
- Replaced the hard `set(CMAKE_CXX_FLAGS_RELEASE "-O2 -DNDEBUG")` override with `add_compile_options($<$<CONFIG:Release>:-O2>)` so CMake's own defaults (which already include `-DNDEBUG`) aren't clobbered

**`justfile`:**
- `cr`/`cro`/`cro3` now use out-of-source builds: `cmake -B build -G Ninja ...` + `cmake --build build` — no `mkdir`, no `mv` hack, and the executable is at `./build/target/<project_name>` (which `RUNTIME_OUTPUT_DIRECTORY` already handled)
- Fixed `cro`/`cro3` run paths to the actual binary location

## Verification

`just cr` ran clean: configure → build → app launched (printed via the render loop; it's a windowed Vulkan app, so the recipe blocks until you close the window — same as before). `build/` contains everything, and no in-source artifacts remain.

# Cmake fix
- Build verified — the binary is at `target/target/00_hello_vulkan_LinuxOS`, matching the fixed recipe. `just cr` should now work end to end.

- One minor note: the `VKB_WSI_SELECTION=WAYLAND` warning means your CMakeLists doesn't use that variable (it's for vulkan-bootstrap's WSI, and your project links GLFW instead). Harmless — you can drop that `-D` flag if you want a clean configure.
