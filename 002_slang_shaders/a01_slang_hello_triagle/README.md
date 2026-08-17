# compile

```bash
slangc ./src/main.slang -profile glsl_450 -target spirv -o ./target/hello-world.spv -entry computeMain

$ eza -la -TL2
drwxrwxr-x   - gygy 17 Aug 16:07 .
.rw-rw-r--   8 gygy 17 Aug 16:03 ├── .gitignore
.rw-rw-r-- 126 gygy 17 Aug 16:07 ├── README.md
drwxrwxr-x   - gygy 17 Aug 16:02 ├── src
.rw-rw-r-- 303 gygy 17 Aug 16:02 │  └── main.slang
drwxrwxr-x   - gygy 17 Aug 16:03 └── target
.rw-rw-r-- 972 gygy 17 Aug 16:07    └── hello-world.spv

```

# `slang` & `slangc` version

```bash
$ slang version
slang version 2026.13.1-1-g84792eb15

$ slangc -version
2026.13.1-1-g84792eb15
```

