```fish
set -gx LD_LIBRARY_PATH "$VULKAN_SDK/lib" $LD_LIBRARY_PATH
set -gx VK_ADD_LAYER_PATH "$VULKAN_SDK/share/vulkan/explicit_layer.d" $VK_ADD_LAYER_PATH
set -gx PKG_CONFIG_PATH "$VULKAN_SDK/lib/VulkanLoader/lib/pkgconfig" "$VULKAN_SDK/share/pkgconfig" "$VULKAN_SDK/lib/pkgconfig" $PKG_CONFIG_PATH
set -gx CMAKE_PREFIX_PATH "$VULKAN_SDK" "$VULKAN_SDK/lib/VulkanLoader"
```

- `.fish/config.fish`
  - https://vulkan.lunarg.com/doc/sdk/1.4.357.1/linux/getting_started.html

```fish
fish_add_path "$HOME/vulkan/vulkansdk-linux-x86_64-1.4.357.1/1.4.357.1/x86_64/bin"

# Vulkan SDK PATH
set -gx VULKAN_SDK $HOME/vulkan/vulkansdk-linux-x86_64-1.4.357.1/1.4.357.1/x86_64
# set -gx LD_LIBRARY_PATH "$VULKAN_SDK/lib:" $LD_LIBRARY_PATH
set -gx LD_LIBRARY_PATH "$VULKAN_SDK/lib" $LD_LIBRARY_PATH
set -gx VK_ADD_LAYER_PATH "$VULKAN_SDK/share/vulkan/explicit_layer.d" $VK_ADD_LAYER_PATH
set -gx PKG_CONFIG_PATH "$VULKAN_SDK/lib/VulkanLoader/lib/pkgconfig" "$VULKAN_SDK/share/pkgconfig" "$VULKAN_SDK/lib/pkgconfig" $PKG_CONFIG_PATH
set -gx CMAKE_PREFIX_PATH "$VULKAN_SDK" "$VULKAN_SDK/lib/VulkanLoader"

```


Notes on the translation:

- `set -gx VAR val` replaces `export VAR=val` (global + exported).
- The `${VAR:+:$VAR}` idiom ("append `:$VAR` only if set, no dangling colon") needs no fish equivalent: an unset `$VAR` expands to **zero list elements**, and fish joins list elements with `:` on export. So trailing `$VAR` on the `set` line gives identical semantics.
- Your first line `export LD_LIBRARY_PATH` had no assignment — in fish that's just `set -gx LD_LIBRARY_PATH` (a no-op for vars inherited from the environment, which are already exported). I assumed the assignment from the Vulkan SDK's `setup-env.sh`; drop the first line if you set `LD_LIBRARY_PATH` elsewhere.
- `CMAKE_PREFIX_PATH` wasn't appended to in your original, so it's a plain overwrite here too.
