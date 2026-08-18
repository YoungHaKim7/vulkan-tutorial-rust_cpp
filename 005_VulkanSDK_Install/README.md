# openSUSE Install error

Fixed. The problem wasn't your setup — it's a GCC 16 false positive that SPIRV-Tools promotes to an error.

## What happened

- Your compiler is **GCC 16.2.0**, far newer than what this SDK's DXC/SPIRV-Tools sources are tested against.
- In `timer.h`, `ScopedTimer<Timer>` allocates exactly a `Timer` (384 bytes), then calls the virtual `Stop()`. GCC 16 speculatively inlines the `CumulativeTimer::Stop()` override and warns that its extra members would be past the end of that 384-byte allocation — a path that can never actually execute, i.e. a bogus `-Warray-bounds` warning.
- SPIRV-Tools' CMake defaults `SPIRV_WERROR=ON` (`external/SPIRV-Tools/CMakeLists.txt:87`), turning that warning into the error that killed your build.

## The fix (already applied and verified)

I reconfigured the existing DXC build with warnings-as-errors off for SPIRV-Tools:

```
cmake -S source/DirectXShaderCompiler -B source/DirectXShaderCompiler/build -DSPIRV_WERROR=OFF
```

Then rebuilt the previously failing `pass_manager.cpp.o` — it compiles cleanly now (warnings only, exit 0). Since the `vulkansdk` script never sets `SPIRV_WERROR` itself, the cached `OFF` survives its re-configure on re-run.

## Resume the build

```sh
./vulkansdk --no-deps --maxjobs
```

- `--no-deps` — skips the sudo package-install step (it only knows Ubuntu/Fedora/Arch and would just print "skipping" on your system anyway).
- `--maxjobs` — important: the script defaults to **1 job**, and DXC is huge. This uses `nproc - 1`.

Everything before DXC already built, so those components will no-op quickly, and the run continues with DXC, then SPIRV-Reflect, profiles, VMA, VulkanCapsViewer, CrashDiagnosticLayer, and slang.

One heads-up: DXC (LLVM-based) isn't routinely built with GCC 16, so if a *different* warning-as-error or GCC incompatibility appears later in the DXC part itself, the clean fallback is building with clang: `./vulkansdk --clean` followed by `CC=clang CXX=clang++ ./vulkansdk --no-deps --maxjobs` — though that rebuilds everything from scratch. If you'd rather not disable `-Werror` wholesale for SPIRV-Tools, the narrower alternative is adding `#pragma GCC diagnostic ignored "-Warray-bounds"` near the top of `source/DirectXShaderCompiler/external/SPIRV-Tools/source/util/timer.h`.


# Vulkan version check

```bash
vulkaninfo | grep "version"
```
