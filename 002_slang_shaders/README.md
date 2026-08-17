# slang
- https://shader-slang.org/slang/user-guide/

- https://shader-slang.org/slang/user-guide/get-started.html

# A collection of tools, libraries, and tests for Vulkan shader compilation.
- https://github.com/google/shaderc

# File Types.(`.vert`, `.frag`, `.spv` 차이점 이해)
- https://vulkan.lunarg.com/doc/view/1.4.321.0/mac/antora/tutorial/latest/03_Drawing_a_triangle/02_Graphics_pipeline_basics/01_Shader_modules.html

- `vert`: Vertex shader source code written in languages like GLSL or HLSL..
  - `vert`: GLSL이나 HLSL 같은 언어로 작성된 버텍스 셰이더 소스 코드..

- `frag`: Fragment shader source code for pixel-level color and lighting calculations.
  - `frag`: 픽셀 단위 색상 및 조명 계산을 위한 프래그먼트 셰이더 소스 코드.

- `.spv`: Standardized binary SPIR-V bytecode compiled from vertex or fragment files, used by APIs like Vulkan
  - `.spv`: 버텍스 또는 프래그먼트 파일에서 컴파일된 표준화된 바이너리 SPIR-V 바이트코드로, Vulkan 같은 API에서 사용됩니다


# 외국유튜브 영상
- [(220624)Vulkan with C++ 10 SPIR V Files | The Graphics Guy](https://youtu.be/5spoiJQo1O8?si=188kRaRBUd4MLcSr)
  - Youtube채널 깃허브 
    - https://github.com/amengede/getIntoGameDev

# `slang` & `slangc` version

```bash
$ slang version
slang version 2026.13.1-1-g84792eb15

$ slangc -version
2026.13.1-1-g84792eb15
```

# `glslc --version`

```bash
$ glslc --version
shaderc v2026.3 v2026.3
spirv-tools v2026.3 v2022.4-1283-gb707790a
glslang 11.1.0-1493-g168d452a

Target: SPIR-V 1.0
```

