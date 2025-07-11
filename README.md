# vulkan-tutorial-rust_cpp

<p align="center">
  <img width=50px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" />
  <img width=35px src="https://user-images.githubusercontent.com/67513038/213403213-1b1b3efc-ce53-4825-9dfc-e9bf2956a7f4.svg" />
  <img width=40px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/02580529-b8e2-4aa9-b80e-dd1f56a08491" />
  <img width=70px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/1599aaad-3821-4abe-b40b-f7000f5ab0b7" />
</p>

# link

- 기초 개념 잡기
  - [Shader](#shader)


<hr />

- https://vulkan-tutorial.com/

- eBook
  - (Rust) Vulkan eBook
    - https://vulkano.rs/
  - Raytracing
    - https://github.com/RayTracing/raytracing.github.io
  - https://www.pbr-book.org/

- [VulkanAPI 알아보기](#vulkanapi)
  - [(외부링크)English ver. VulkanAPI 설명서_https://vkguide.dev/](https://vkguide.dev/)
  - [(외부링크)한글 ver. VulkanAPI 설명서_https://vkguide.dev/docs/ko](https://vkguide.dev/docs/ko)

<hr />

- [Vulkan VS. OpenGL 차이점 이해](#vulkan-vs-opengl)

<hr />

- WGSL Tutorial
  - https://google.github.io/tour-of-wgsl/
  - 여기에 정리중...
    - https://github.com/YoungHaKim7/WGSL_Training
      - Rust로 만든 WGSL-playground(cargo run 하고 뒤에 "파일명.wgsl"하면 화면으로 나옴 굿)
        - https://github.com/paulgb/wgsl-playground 

<hr />

- 내가 공부하려고 만든 영상 (모아보기)GlobalYoung
  - https://youtube.com/playlist?list=PLcMveqN_07mYLlQ72z9uktIcWF0kNLGxB&si=4JwpIMy4iZVYJsnk


<hr>

# Vulkan(API)[|🔝|](#link)

- [Vulkan API docs문서(API찾을때 굿](https://docs.vulkan.org/spec/latest/index.html)
  - [Vulkan Blog에 잘 정리됨](https://www.khronos.org/blog/an-introduction-to-vulkan-video)
  - [Vulkan 그룹에서 제공하는 Reference guide 및 다양한 정보가 많다The Khronos Group Reference Guides are available as downloadable PDFs, online viewable format using Slideshare and a hardcopy format from LuLu.
If you find a mistake in any of the reference guides, please submit a bug.](https://www.khronos.org/developers/reference-cards/)

- https://www.vulkan.org/
  - Vulkanised 2024 | Vulkan 공식 유튜브
    - https://youtube.com/playlist?list=PLMLurvdlOpWOh_nXIbXbBewWuERe-8xVW&si=qtukRIafDafmLA6j

- https://namu.wiki/w/Vulkan(API)

- Safe and rich Rust wrapper around the Vulkan API
  - https://github.com/vulkano-rs/vulkano

- Vulkan tutorial
  - Vulkan(Rust Tutorial)
    - https://kylemayes.github.io/vulkanalia/
  - https://vulkan.lunarg.com/doc/sdk/1.1.106.0/windows/getting_started.html
  - Vulkan-Guide
    - https://github.com/KhronosGroup/Vulkan-Guide

  - Vulkan vs directX 차이점 이해
    - https://techscene.tistory.com/entry/게이머-필독-Vulkan과-DirectX-비교-분석-발더스-게이트-3

- [Big Picture(step 8)](#big-picturestep-8)
  - [요약본 같이 보면 더 좋다 Summary)So in short, to draw the first triangle we need to:](#summaryso-in-short-to-draw-the-first-triangle-we-need-to)

<hr />

# Vulkan Install
- https://vulkan.lunarg.com/doc/view/latest/linux/getting_started_ubuntu.html

<hr />

# Vulkan vs OpenGL


|-|OpenGL | Vulkan|
|-|-|-|
|Thread|Single-threading|Multi-threading|
|global<br> state<br> machine|One single global state machine |	Object-based with no global state|
|state<br> concepts|State is tied to a single context |	All state concepts are localized to a command buffer|
||Operations can only be executed sequentially |	Multi-threaded programming is possible|
|memory management|GPU memory and synchronization are usually hidden |	Explicit control over memory management and synchronization|
|checking at runtime|Extensive error checking |	Vulkan drivers do no error checking at runtime;<br> there is a validation layer for developers |

- https://en.wikipedia.org/wiki/Vulkan

- 그림으로 이해
  - OpenGL and Vulkan are both rendering APIs. In both cases, the GPU executes shaders, while the CPU executes everything else.

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/Division_of_labor_cpu_and_gpu.svg/500px-Division_of_labor_cpu_and_gpu.svg.png" />

- Vulkan
  - https://vkguide.dev/docs/extra-chapter/multithreading/
  - https://en.wikipedia.org/wiki/Vulkan
    - 한글 설명서
      - https://vkguide.dev/docs/ko

- OpenGL
  - https://en.wikipedia.org/wiki/OpenGL


# Command line tool for building Rust shaders using rust-gpu 
- https://github.com/Rust-GPU/cargo-gpu

# Learning Vulkan with Rust | Tsoding Daily[|🔝|](#link)
- https://youtu.be/8iEN64bj3X4?si=45lau6cdCsHBQvkl

<hr>

# OpenGL-rs
- https://github.com/bwasty/learn-opengl-rs

<hr />

# NVIDIA[|🔝|](#link)

- https://www.nvidia.com/en-us/drivers/unix/

<hr />

<br>

# Metal(macOS)[|🔝|](#link)


<hr>

# DirectX(WindowsOS)[|🔝|](#link)

- DirectX3D 51강~55강
  - https://m.blog.naver.com/gp89ky/223027167276

<hr />

# Shader[|🔝|](#link)
- https://en.m.wikipedia.org/wiki/Shader

# Big Picture(step 8)[|🔝|](#link)
- Step1(Instance and physical device selection)
  - A Vulkan application start by setting up the Vulkan API through a `VkInstance`.
- Step 2 - Logical device and queue families
  - After selecting the right hardware device to use, you need to create a `VkDevice` (logical device), where you describe more specifically which `VkPhysicalDeviceFeatures` you will be using, like multi viewport rendering and 64 bit floats.
- Step 3 - Window surface and swap chain
  - Unless you’re only interested in offscreen rendering, you will need to create a window to present rendered images to.
    - We need two more components to actually render to a window: a window surface (`VkSurfaceKHR`) and a swap chain (`VkSwapchainKHR`)
- Step 4 - Image views and framebuffers
  - To draw to an image acquired from the swap chain, we have to wrap it into a `VkImageView` and `VkFramebuffer`.
- Step 5 - Render passes
  - Render passes in Vulkan describe the type of images that are used during rendering operations, how they will be used, and how their contents should be treated.
    - In our initial triangle rendering application, we’ll tell Vulkan that we will use a single image as color target and that we want it to be cleared to a solid color right before the drawing operation. Whereas a render pass only describes the type of images, a `VkFramebuffer` actually binds specific images to these slots.
- Step 6 - Graphics pipeline
  - The graphics pipeline in Vulkan is set up by creating a `VkPipeline` object. 
    - It describes the configurable state of the graphics card, like the viewport size and depth buffer operation and the programmable state using `VkShaderModule` objects.

- Step 7 - Command pools and command buffers
  - As mentioned earlier, many of the operations in Vulkan that we want to execute, like drawing operations, need to be submitted to a queue. These operations first need to be recorded into a `VkCommandBuffer` before they can be submitted. These command buffers are allocated from a `VkCommandPool` that is associated with a specific queue family. To draw a simple triangle, we need to record a command buffer with the following operations:
    - 1. Begin the render pass
    - 2. Bind the graphics pipeline
    - 3. Draw 3 vertices

- Step 8 - Main loop
  - Now that the drawing commands have been wrapped into a command buffer, the main loop is quite straightforward. We first acquire an image from the swap chain with `vkAcquireNextImageKHR`. We can then select the appropriate command buffer for that image and execute it with `vkQueueSubmit`. Finally, we return the image to the swap chain for presentation to the screen with `vkQueuePresentKHR`.

- 출처: 다음에서 발췌 Vulkan Tutorial | Alexander Overvoorde

<hr />

## (Summary)So in short, to draw the first triangle we need to:

- Create a `VkInstance`
- Select a supported graphics card (`VkPhysicalDevice`)
- Create a `VkDevice` and `VkQueue` for drawing and presentation
- Create a window, window surface and swap chain
- Wrap the swap chain images into `VkImageView`
- Create a render pass that specifies the render targets and usage
- Create framebuffers for the render pass
- Set up the graphics pipeline
- Allocate and record a command buffer with the draw commands for every possible swap chain image
- Draw frames by acquiring images, submitting the right draw command buffer and returning the images back to the swap chain

<hr />


# How to Use Glslang[|🔝|](#link)
- https://github.com/KhronosGroup/glslang

## Execution of Standalone Wrapper

- To use the standalone binary form, execute glslang, and it will print a usage statement. Basic operation is to give it a file containing a shader, and it will print out warnings/errors and optionally an AST.

- The applied stage-specific rules are based on the file extension:
  - .vert for a vertex shader
  - .tesc for a tessellation control shader
  - .tese for a tessellation evaluation shader
  - .geom for a geometry shader
  - .frag for a fragment shader
  - .comp for a compute shader

- For ray tracing pipeline shaders:
  - .rgen for a ray generation shader
  - .rint for a ray intersection shader
  - .rahit for a ray any-hit shader
  - .rchit for a ray closest-hit shader
  - .rmiss for a ray miss shader
  - .rcall for a callable shader

- There is also a non-shader extension:
  - .conf for a configuration file of limits, see usage statement for example


<hr />

# Vulkan main objects and their use[|🔝|](#link)

- `VkInstance` : The Vulkan context, used to access drivers.
- `VkPhysicalDevice` : A GPU. Used to query physical GPU details, like features, capabilities, memory size, etc.
- `VkDevice` : The “logical” GPU context that you actually execute things on.
- `VkBuffer` : A chunk of GPU visible memory.
- `VkImage` : A texture you can write to and read from.
- `VkPipeline` : Holds the state of the gpu needed to draw. For example: shaders, rasterization options, depth settings.
- `VkRenderPass` : Holds information about the images you are rendering into. All drawing commands have to be done inside a renderpass. Only used in legacy vkguide
- `VkFrameBuffer` : Holds the target images for a renderpass. Only used in legacy vkguide
- `VkCommandBuffer` : Encodes GPU commands. All execution that is performed on the GPU itself (not in the driver) has to be encoded in a VkCommandBuffer.
- `VkQueue` : Execution “port” for commands. GPUs will have a set of queues with different properties. Some allow only graphics commands, others only allow memory commands, etc. Command buffers are executed by submitting them into a queue, which will copy the rendering commands onto the GPU for execution.
- `VkDescriptorSet` : Holds the binding information that connects shader inputs to data such as VkBuffer resources and VkImage textures. Think of it as a set of gpu-side pointers that you bind once.
- `VkSwapchainKHR` : Holds the images for the screen. It allows you to render things into a visible window. The KHR suffix shows that it comes from an extension, which in this case is VK_KHR_swapchain
- `VkSemaphore` : Synchronizes GPU to GPU execution of commands. Used for syncing multiple command buffer submissions one after other.
- `VkFence` : Synchronizes GPU to CPU execution of commands. Used to know if a command buffer has finished being executed on the GPU.
