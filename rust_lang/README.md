# vulkano

- https://github.com/vulkano-rs/vulkano/tree/master/examples
  - https://github.com/vulkano-rs/vulkano

- eBooks
  - https://vulkano.rs/

<hr />

# Install
- macOS
  - macOS Install
    - https://vulkan.lunarg.com/doc/view/1.3.268.1/mac/getting_started.html
  - Developing with Vulkan on Apple iOS
    - https://www.khronos.org/blog/developing-with-vulkan-on-apple-ios
    - https://github.com/khronosGroup/Vulkan-samples
    - https://registry.khronos.org/vulkan/specs/1.3-khr-extensions/html/

# cargo wgsl(wgsl문법확인 빠르게 하기)
- Validate wgsl in rust projects
  - https://crates.io/crates/cargo-wgsl

```
# wgsl있는 폴더에서 누르면 문법확인 빠르게 해줌
cargo wgsl
```

# Vulkano관련 종류별 정리

- 종류별 찾아보기 좋다.(https://areweguiyet.com/)

<hr />

- vulkano[![crates.io](https://img.shields.io/crates/v/vulkano.svg)](https://crates.io/crates/vulkano)
![Crates.io](https://img.shields.io/crates/l/vulkano)
![Downloads](https://img.shields.io/crates/d/vulkano.svg)<a href="https://github.com/vulkano-rs/vulkano"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![icedstar](https://img.shields.io/github/stars/vulkano-rs/vulkano.svg)

- Safe and rich Rust wrapper around the Vulkan API

  - https://crates.io/crates/vulkano
  - https://github.com/vulkano-rs/vulkano

<hr />

- winit[![crates.io](https://img.shields.io/crates/v/winit.svg)](https://crates.io/crates/winit)
![Crates.io](https://img.shields.io/crates/l/winit)
![Downloads](https://img.shields.io/crates/d/winit.svg)<a href="https://github.com/rust-windowing/winit"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![Star](https://img.shields.io/github/stars/rust-windowing/winit.svg)

- winit - Cross-platform window creation and management in Rust
  - https://crates.io/crates/winit
  - https://github.com/rust-windowing/winit

<hr />

- wgpu[![crates.io](https://img.shields.io/crates/v/wgpu.svg)](https://crates.io/crates/wgpu)
![Crates.io](https://img.shields.io/crates/l/wgpu)
![Downloads](https://img.shields.io/crates/d/wgpu.svg)<a href="https://github.com/gfx-rs/wgpu"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![star](https://img.shields.io/github/stars/gfx-rs/wgpu.svg)

- wgpu is a cross-platform, safe, pure-rust graphics API. It runs natively on Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on wasm.
  - The API is based on the WebGPU standard. It serves as the core of the WebGPU integration in Firefox and Deno.
    - https://crates.io/crates/wgpu
    - https://github.com/gfx-rs/wgpu

<hr />

- ash[![crates.io](https://img.shields.io/crates/v/ash.svg)](https://crates.io/crates/ash)
![Crates.io](https://img.shields.io/crates/l/ash)
![Downloads](https://img.shields.io/crates/d/ash.svg)<a href="https://github.com/ash-rs/ash"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![Star](https://img.shields.io/github/stars/ash-rs/ash.svg)

- Vulkan bindings for Rust. 
  - https://crates.io/crates/ash
  - https://github.com/ash-rs/ash

<hr />

- vulkanalia[![crates.io](https://img.shields.io/crates/v/vulkanalia.svg)](https://crates.io/crates/vulkanalia)
![Crates.io](https://img.shields.io/crates/l/vulkanalia)
![Downloads](https://img.shields.io/crates/d/vulkanalia.svg)<a href="https://github.com/KyleMayes/vulkanalia"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![Star](https://img.shields.io/github/stars/KyleMayes/vulkanalia.svg)

- Vulkan bindings for Rust. 
  - https://crates.io/crates/vulkanalia
  - https://github.com/KyleMayes/vulkanalia

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
