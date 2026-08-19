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

# fix(gcc16)

- All fixed and verified. Only this chapter uses the CMake recipe, so no siblings need the same fix.

## What was wrong

The link error had nothing to do with library paths — it was the **driver binary**. Your justfile had:

```just
gpp_which := "/opt/gcc-16/bin/gcc"    # ← the C driver
```

passed as `CMAKE_CXX_COMPILER`. `gcc` compiles `.cpp` files fine but **never adds `-lstdc++` when linking** — only `g++` does. That's exactly what every one of those errors was: `operator new`, `std::__throw_bad_alloc`, `__cxa_begin_catch`, `__gxx_personality_v0` are all libstdc++/libgcc symbols.

Your fish config also had two real problems that would have bitten you later:

- **`CPLUS_INCLUDE_PATH` was forcing GCC 16 to compile against GCC 13's headers.** I verified the search order: `/usr/include/c++/13` comes *before* `/opt/gcc-16/include/c++/16.2.0` (that's why your error output shows `/usr/include/c++/13/bits/new_allocator.h`). Mixing GCC 13 headers with a GCC 16 link is an ABI hazard — and the line appended to itself, so it had already accumulated 3 duplicate copies.
- **`set -gx LD_LIBRARY_PATH /usr/local/lib` (the SDL3 line) overwrote** the `/opt/gcc-16/lib64` entry set 7 lines earlier, so your gcc16 runtime lib was never actually being found.

## Changes made

**`justfile`**
- `gpp_which` → `/opt/gcc-16/bin/g++` (this alone fixes your link error)
- Added `-D CMAKE_EXE_LINKER_FLAGS=-Wl,-rpath,/opt/gcc-16/lib64` to `cr`/`cro`/`cro3` — the binary now embeds the path to GCC 16's libstdc++, so it runs from any shell/IDE/debugger (system libstdc++ is only GLIBCXX 3.4.33)
- Fixed `r` recipe: `src_dir+"main.cpp"` concatenated to `srcmain.cpp` (missing `/`), and it now uses `gpp_which` too

**`~/.config/fish/config.fish`**
- Removed `CPLUS_INCLUDE_PATH` and `LIBRARY_PATH` — the g++ driver finds its own headers/libs relative to `/opt/gcc-16` automatically; no env vars needed
- Fixed the SDL3 line to *append* instead of overwrite, so `LD_LIBRARY_PATH` keeps `/opt/gcc-16/lib64`

## One thing you need to do

**Open a fresh terminal** (or run `exec fish`). Your already-open terminals still carry the old exported `CPLUS_INCLUDE_PATH`; new shells will be clean.

Verified end-to-end: configure + build + link with the exact `cr` flags succeeds, and the binary in `target/debug/` resolves `libstdc++.so.6 => /opt/gcc-16/lib64/libstdc++.so.6` even with a completely empty environment. `just cr` should now work — it will also launch the windowed app at the end, as before.

Side note while investigating: `just --dry-run` doesn't execute backticks, so it displays the *wrong* `if os == "Linux"` branch in its output — real runs pick the right one. Don't let a dry-run fool you the way it briefly fooled me.
