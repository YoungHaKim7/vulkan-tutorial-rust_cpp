# compile

- SPIR-V 어셈블리 보기: 터미널이나 명령 프롬프트에서 `spirv-dis shader.frag.spv -o shader.spvasm` 명령어를 입력하면 바이트코드를 사람이 읽을 수 있는 SPIR-V 어셈블리 텍스트 파일로 바꿀 수 있습니다.

```bash
$ glslc ./src/shader_base.frag -o ./target/frag.spv

$ spirv-dis target/frag.spv -o shader.spvasm
```
