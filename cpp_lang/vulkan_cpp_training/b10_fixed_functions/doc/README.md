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
