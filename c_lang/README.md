# wgpu(c언어 test예정)
- https://github.com/gfx-rs/wgpu-native/tree/trunk/examples/triangle

<hr />

# vkQuake
- https://github.com/Novum/vkQuake

vkQuake uses **Meson** as its build system. Since you're on Linux, from the repo root:

```console
meson setup build -Ddebug=true -Dstrip=false && ninja -C build
```

The binary ends up at `build/Quake/vkquake` (or similar path under `build/`).

**Dependencies first:**

```console
# Ubuntu/Debian
apt-get install git meson gcc glslang-tools spirv-tools libsdl3-dev libvulkan-dev libvorbis-dev libmpg123-dev libx11-xcb-dev

# Arch
pacman -S git meson flac glibc libgl mpg123 libvorbis libx11 sdl3 vulkan-headers glslang spirv-tools

# Fedora
dnf install git meson gcc glslang spirv-tools vulkan-loader-devel SDL3-devel mpg123-devel libvorbis-devel flac-devel opusfile-devel
```

Notes (from `readme.md:81-181`):

- **Meson ≥ 1.3.0** is required, and you need a Vulkan SDK ≥ 1.2.162. If your distro's SDK is older, install the LunarG SDK.
- Meson prefers **SDL3** and falls back to SDL2 automatically (force with `-Duse_sdl3=disabled` / `=enabled`).
- Older distro without new enough Meson? Fall back to make: `cd Quake && make -j`.
- To run, you'll need game data (`id1/pak0.pak` + `pak1.pak`) next to the binary.

Want me to check whether your system already has the needed deps and kick off the build?
