# Rust 관련 SPIR-V 여기에 정리중(260817)

- https://github.com/YoungHaKim7/rust_gui_vulkan_trainning/tree/main/005_rSPIR-V_Module

<hr />

# C++관련된거 여기에 정리중(260817)

# 명령어 도구 사용 (`spirv-dis`)

- Fragment Shader(`.frag`) 파일을 컴파일하여 생성된 `.spv`(SPIR-V) 바이너리 파일 내부를 확인하려면 Vulkan SDK에 포함된 디스어셈블러인 `spirv-dis` 명령어나 온라인 뷰어를 사용해 텍스트 형태의 어셈블리 또는 GLSL 코드로 변환해 볼 수 있습니다.

## 명령어 도구 사용 (`spirv-dis`)

- SPIR-V 어셈블리 보기: 터미널이나 명령 프롬프트에서 `spirv-dis shader.frag.spv -o shader.spvasm` 명령어를 입력하면 바이트코드를 사람이 읽을 수 있는 SPIR-V 어셈블리 텍스트 파일로 바꿀 수 있습니다.

- Vulkan SDK 확인: `spirv-dis` 명령어는 Vulkan SDK 설치 시 기본적으로 포함되어 경로에 등록됩니다.

## 코드 변환 및 리플렉션

- GLSL로 역참조: [SPIRV-Cross](https://github.com/khronosgroup/spirv-cross) 라이브러리를 사용하면 `.spv` 파일을 다시 가독성 높은 GLSL이나 타 언어로 디컴파일할 수 있습니다.

- 변수 정보 확인: [SPIRV-Reflect](https://github.com/KhronosGroup/SPIRV-Reflect)를 이용해 셰이더 내부의 유니폼, 바인딩, 속성 등의 메타데이터(리플렉션)를 코드로 추출할 수 있습니다.
