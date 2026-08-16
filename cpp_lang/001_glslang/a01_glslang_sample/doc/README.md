Everything is fixed and verified except one thing I can't do myself: installing the missing package requires your sudo password.

## What was wrong

1. **Missing package** — only `glslang-tools` (the CLI binaries) is installed; the library headers/libs come from `glslang-dev`, which isn't installed. That's the direct cause of the `ShaderLang.h: No such file or directory` error.
2. **CMakeLists.txt never linked glslang** — the `GLSLANGLIBS` variable was defined but never used. Also stale: `HLSL` and `OGLCompiler` no longer exist as separate libs in glslang 15.
3. **Latent runtime bug in main.cpp** (found by testing) — the zero-initialized `TBuiltInResource` sets every limit to 0, so the shader would have failed to parse even after fixing the build:
   ```
   ERROR: 0:3: 'location' : too large for fragment output
   ```

## Changes made

**`CMakeLists.txt`** — replaced the stale list with a real find/link (the package ships a proper CMake config):
```cmake
find_package(glslang REQUIRED CONFIG)
...
target_link_libraries(${ProjectId}
    Vulkan::Vulkan
    glslang::glslang
    glslang::SPIRV
    glslang::glslang-default-resource-limits
    dl pthread)
```

**`src/main.cpp`** — replaced the hand-rolled partial resource struct with `GetDefaultResources()` (src/main.cpp:41).

I verified the fix end-to-end by downloading the exact `glslang-dev` package to /tmp and compiling/running your `main.cpp` against it: `Success! Compiled SPIR-V word count: 92`.

## One step for you

Type this in the prompt so it runs in-session (sudo needs your password):

```
! sudo apt install glslang-dev
```

Then I'll run `just cr` to confirm the full build.


# glslang

Everything checks out — this isn't a CMake bug. The `CMakeLists.txt` is correct; the machine is just missing the `glslang-dev` package (only `glslang-tools`, the CLI validator, is installed).

I verified by downloading the `glslang-dev` deb from your apt repo: it ships `/usr/lib/x86_64-linux-gnu/cmake/glslang/glslang-config.cmake` and exports exactly the targets you link (`glslang::glslang`, `glslang::SPIRV`, `glslang::glslang-default-resource-limits`). Its SPIRV target also needs the SPIRV-Tools static libs and headers, which you already have.

Sudo needs a password, so please run this yourself (the `!` prefix runs it in this session):

```
! sudo apt install glslang-dev
```

Then I'll re-run the build to confirm. If you'd rather not install the system package, the alternative is building glslang via `FetchContent`/submodule in CMake — but since Ubuntu 24.04's package provides a proper config, `apt install` is the clean fix.
