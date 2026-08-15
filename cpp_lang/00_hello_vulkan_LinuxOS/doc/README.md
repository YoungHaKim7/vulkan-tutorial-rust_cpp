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
