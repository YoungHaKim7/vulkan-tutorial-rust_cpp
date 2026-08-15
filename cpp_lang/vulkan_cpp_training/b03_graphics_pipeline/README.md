# Result
- https://vulkan-tutorial.com/code/08_graphics_pipeline.cpp

```bash
validation layer: Searching for ICD drivers named /usr/lib64/libGLX_nvidia.so.0
validation layer: Searching for ICD drivers named /usr/lib64/libvulkan_radeon.so
validation layer: Searching for ICD drivers named /usr/lib64/libvulkan_lvp.so
validation layer: Searching for ICD drivers named /usr/lib64/libvulkan_intel_hasvk.so
validation layer: Searching for ICD drivers named /usr/lib64/libvulkan_intel.so
validation layer: Searching for ICD drivers named /usr/lib/libvulkan_intel_hasvk.so
validation layer: Searching for ICD drivers named /usr/lib/libvulkan_intel.so
validation layer: Searching for ICD drivers named /usr/lib/libvulkan_radeon.so
validation layer: Searching for ICD drivers named /usr/lib/libGLX_nvidia.so.0
validation layer: Loading layer library libVkLayer_khronos_validation.so
validation layer: Loading layer library libVkLayer_MESA_device_select.so
validation layer: Copying old device 0 into new device 0
validation layer: Copying old device 1 into new device 1
validation layer: Copying old device 2 into new device 2
validation layer: Copying old device 0 into new device 0
validation layer: Copying old device 1 into new device 1
validation layer: Copying old device 2 into new device 2
validation layer: Copying old device 0 into new device 0
validation layer: Copying old device 1 into new device 1
validation layer: Copying old device 2 into new device 2
validation layer: Copying old device 0 into new device 0
validation layer: Copying old device 1 into new device 1
validation layer: Copying old device 2 into new device 2
validation layer: Copying old device 0 into new device 0
validation layer: Copying old device 1 into new device 1
validation layer: Copying old device 2 into new device 2
validation layer: Copying old device 0 into new device 0
validation layer: Copying old device 1 into new device 1
validation layer: Copying old device 2 into new device 2
validation layer: Copying old device 0 into new device 0
validation layer: Copying old device 1 into new device 1
validation layer: Copying old device 2 into new device 2
validation layer: vkCreateDevice(): pCreateInfo->enabledLayerCount is 1 (not zero).
Device Layers have never worked since Vulkan 1.0 and only Instance Layers should be used instead: https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-devicelayers.
The Vulkan spec states: enabledLayerCount must be 0 (https://docs.vulkan.org/spec/latest/chapters/devsandqueues.html#VUID-VkDeviceCreateInfo-enabledLayerCount-12384)
```
