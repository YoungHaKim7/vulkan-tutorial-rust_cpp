# compile

```bash
$ glslc ./src/shader_base.vert -o ./target/vert.spv

$ glslc ./src/shader_base.frag -o ./target/frag.spv


$ eza -la -TL2
drwxrwxr-x    - gygy 17 Aug 16:12 .
.rw-rw-r--    8 gygy 17 Aug 16:09 ├── .gitignore
.rw-rw-r--  507 gygy 17 Aug 16:12 ├── README.md
drwxrwxr-x    - gygy 17 Aug 16:10 ├── src
.rw-rw-r--  149 gygy 17 Aug 16:10 │  ├── shader_base.frag
.rw-rw-r--  369 gygy 17 Aug 16:10 │  └── shader_base.vert
drwxrwxr-x    - gygy 17 Aug 16:11 └── target
.rw-rw-r--  572 gygy 17 Aug 16:11    ├── frag.spv
.rw-rw-r-- 1.5k gygy 17 Aug 16:11    └── vert.spv
```

# `glslc --version`

```bash
$ glslc --version
shaderc v2026.3 v2026.3
spirv-tools v2026.3 v2022.4-1283-gb707790a
glslang 11.1.0-1493-g168d452a

Target: SPIR-V 1.0
```
