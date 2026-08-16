# fix 
Exit code 124 = the app was still running at timeout, in its render loop (the repeated "Copying old device" lines are its per-frame validation output). Fixed and verified.

## Root cause — two problems, not one

1. **Wrong path**: your justfile runs the binary from the **project root** (`./target/debug/a09_shader_modules`), so `"../shaders/..."` pointed *outside* the project. The `../` prefix comes from vulkan-tutorial.com, which runs the binary from inside `build/`.
2. **Wrong file type**: even if the file had opened, `vkCreateShaderModule` requires **SPIR-V bytecode** — loading raw GLSL text (`.vert`/`.frag`) would have failed next with `vkCreateShaderModule` errors (SPIR-V magic-number check).

## Changes

- `src/main.cpp:385` — now reads the compiled `shaders/vert.spv` / `shaders/frag.spv` (relative to project root, works for both `just r` and `just cr`)
- `src/main.cpp:618` — error message now includes the filename (`"failed to open file!: " + filename`), so this failure is diagnosable next time
- `CMakeLists.txt` — added a `glslc` custom command: `shaders/shader_base.{vert,frag}` → `shaders/{vert,frag}.spv`, only recompiled when the GLSL changes
- `justfile` — new `shaders` recipe (plain `glslc` calls), hooked into the g++ build via `just r`; the cmake recipes get it automatically through the custom target
- `.gitignore` — added `shaders/*.spv` since they're generated next to the sources

## Verified

```
[1/4] glslc: shaders/shader_base.vert -> shaders/vert.spv
[2/4] glslc: shaders/shader_base.frag -> shaders/frag.spv
[4/4] Linking CXX executable debug/a09_shader_modules
```
App ran the full 6s timeout in the render loop — no "failed to open file", no shader-module errors.

<hr />

Verified — the swap is clean (`libglfw3-dev` stays installed, nothing else affected).

## What was wrong

**Code bug:** `initWindow()` never checked `glfwInit()` or `glfwCreateWindow()`. When window creation fails, `window` stays `NULL` and gets passed to `glfwCreateWindowSurface()`, which hits GLFW's internal `assert(window != NULL)` → abort (exit 134).

**Why it failed on your machine:** your current session is **X11** (`XDG_SESSION_TYPE=x11`, `DISPLAY=:1`, no `WAYLAND_DISPLAY`), but the installed runtime library is `libglfw3-wayland` — Debian/Ubuntu's Wayland-*only* GLFW build (`nm` shows zero X11 symbols in it). So `glfwInit()` failed with `Wayland: Failed to connect to display` (error 65544), silently, since GLFW 3.3 swallows errors unless you install an error callback. Your previous chapters worked because you were in a Wayland session then.

## Fix applied to `src/main.cpp`

- `initWindow()` (`src/main.cpp:96`) now sets a `glfwErrorCallback`, checks `glfwInit()`, and throws if `glfwCreateWindow()` returns `NULL`
- Added the static `glfwErrorCallback` next to `debugCallback` (`src/main.cpp:568`) that prints the GLFW error code + description

Result: instead of a core dump you now get
```
GLFW error (65544): Wayland: Failed to connect to display
failed to initialize GLFW!
```

## To actually run it in this X11 session

The code is now correct, but it needs an X11-capable GLFW. Either install the X11 build:

```
! sudo apt install libglfw3
```

(replaces `libglfw3-wayland`; swap back the same way if you return to a Wayland session) — or log back into your Wayland session.

One heads-up for the other chapters: your justfile passes `-D VKB_WSI_SELECTION=WAYLAND`, which is a vk-bootstrap option (CMake warned it's unused here). If you stay on X11, you'll want `X11` there too.
