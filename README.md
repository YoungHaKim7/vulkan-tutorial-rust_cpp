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
  - Raytracing
    - https://github.com/RayTracing/raytracing.github.io
  - https://www.pbr-book.org/

<hr />

- 내가 공부하려고 만든 영상 (모아보기)GlobalYoung
  - https://youtube.com/playlist?list=PLcMveqN_07mYLlQ72z9uktIcWF0kNLGxB&si=4JwpIMy4iZVYJsnk


<hr>

# Vulkan(API)[|🔝|](#link)

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

# Learning Vulkan with Rust | Tsoding Daily[|🔝|](#link)
- https://youtu.be/8iEN64bj3X4?si=45lau6cdCsHBQvkl

<hr>

# OpenGL-rs
- https://github.com/bwasty/learn-opengl-rs

<hr />

# NVIDIA[|🔝|](#link)

- https://www.nvidia.com/en-us/drivers/unix/

# Comparison(Vulkan 하고 다른것 들과 비교)[|🔝|](#link)

Comparison to other well-known Graphics APIs in Rust ecosystem.

| Name | Open-sourced Since | API Level | Notable Features |
| ---- | ------------------ | --------- | ------------- |
| Vulkano | March, 2016 | High-level Rust API wrapping Vulkan APIs. | Type-safe compile-time shaders. Transparent interoperability with glsl and spir-v shader code types in Rust code. Automatically generated types for shader's Layout. |
| [Wgpu](https://github.com/gfx-rs/wgpu) | May, 2019 | High-level Rust API with multiple backends. | Supports multiple backends: Vulkan, Metal, DirectX, WebGPU, and other. Follows WebGPU specification. With async/await API. |
| [Miniquad](https://github.com/not-fl3/miniquad) | March, 2020 | High-level minimalistic Rust API with multiple backends. | Relatively minimalistic API well suited for small to medium graphics projects. Supports multiple backends, including browser target. |
| [Sierra](https://github.com/zakarumych/sierra) | March, 2021 | High-level Rust API for Vulkan/Metal APIs. | Layouts, Descriptors and shader Types construction in Rust code through the macro system. Built on top of [Erupt](https://gitlab.com/Friz64/erupt). Supports Ray Tracing Pipeline. |
| [Glium](https://github.com/glium/glium) | October, 2014 | High-level Rust API wrapping OpenGL | OpenGL only. |
| [Ash](https://github.com/MaikKlein/ash) | August, 2016 | Low-level API for Vulkan. | Unsafe Vulkan API bindings. |
| [Erupt](https://gitlab.com/Friz64/erupt) | April, 2020 | Low-level API for Vulkan. | Unsafe Vulkan API bindings. |

- 출처 : https://github.com/vulkano-rs/vulkano

<hr>

<hr>

<br>

# Metal(macOS)[|🔝|](#link)


<hr>

# DirectX(WindowsOS)[|🔝|](#link)

- DirectX3D 51강~55강
  - https://m.blog.naver.com/gp89ky/223027167276

<hr />

# Shader[|🔝|](#link)
- https://en.m.wikipedia.org/wiki/Shader

# Big Picture
- Step1(Instance and physical device selection)
  - A Vulkan application start by setting up the Vulkan API through a `VkInstance`.