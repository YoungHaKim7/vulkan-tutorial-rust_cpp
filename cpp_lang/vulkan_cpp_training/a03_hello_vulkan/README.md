# LinuxOS(openSUSE)
- You need to install GLFW development libraries. Run:

```bash
sudo zypper install libglfw-devel glm-devel libglfw-devel

# LinuxOS
sudo zypper in wayland-protocols-devel liblz4-devel
sudo zypper install 'pkgconfig(xcb-keysyms)'

# Vulkan Install
sudo zypper in libvulkan1 libvulkan1-32bit \
libvulkan_intel libvulkan_intel-32bit \
libvulkan_radeon libvulkan_radeon-32bit \
vulkan-tools
```

After installing, the build should work. The justfile and CMakeLists.txt have been updated to:
1. Use `/usr/include` and `/usr/lib` for Linux (instead of `/opt/homebrew`)
2. Link with Linux-specific libraries (`dl`, `X11`, `pthread`) instead of macOS frameworks
3. Skip `DYLD_LIBRARY_PATH` on Linux since system libraries are in standard paths

# justfile(macOS & LinuxOS)

```justfile
# project name
project_name := `basename "$(pwd)"`

# Detect OS
os := `uname`

# which g++
gpp_which := `which g++`

# Source and target directories
src_dir := "./src"
target_dir := "./target"

# Files
source := src_dir+"/main.cpp"
target := target_dir+"/main"

# Common flags
cflags := if os == "Linux" { \
    "-std=c++17 -O2 -I/usr/include" \
  } else if os == "Darwin" { \
    "-std=c++17 -O2 -I/opt/homebrew/include" \
  } else { \
    "-std=c++17 -O2 -I/usr/local/include" \
  }

ldflags := if os == "Linux" { \
    "-lglfw -lvulkan -ldl -lX11 -lpthread" \
  } else if os == "Darwin" { \
    "-L/opt/homebrew/lib -lglfw -lvulkan -framework Cocoa -framework IOKit -framework CoreVideo" \
  } else { \
    "-lglfw -lvulkan" \
  }
ldflags_emit_llvm := "-S -emit-llvm"
ldflags_assembly := "-Wall -save-temps"
ldflags_fsanitize_address := "-O1 -g -fsanitize=address -fno-omit-frame-pointer -c"
ldflags_fsanitize_object := "-g -fsanitize=address"
ldflags_fsanitize_valgrind := "-fsanitize=address -g3 -std=c++2b"

clang_format_basic := `which clang-format`
cmake := `which cmake`

# clang-format 21
clang_format := if os == "Linux" { \
    "clang-format-21" \
  } else if os == "Darwin" { \
    "/opt/homebrew/opt/llvm/bin/clang-format" \
  } else { \
    clang_format_basic \
  }

# fmt .clang-format(linuxOS / macOS)
fmt_flags := if os == "Linux" { \
    ". -regex '.*\\.\\(cpp\\|hpp\\|cc\\|cxx\\|ixx\\|cppm\\|c\\|h\\)' -exec " \
    +clang_format+ \
    " -style=file -i {} \\;" \
  } else if os == "Darwin" { \
    ". -iname '*.cpp' \
    -o -iname '*.hpp' \
    -o -iname '*.cc' \
    -o -iname '*.c' \
    -o -iname '*.cxx' \
    -o -iname '*.cppm' \
    -o -iname '*.ixx' \
    -o -iname '*.c' \
    -o -iname '*.h' | " \
    +clang_format+ \
    " -style=file -i --files=/dev/stdin" \
  } else { \
    ". -regex '.*\\.\\(cpp\\|hpp\\|cc\\|cxx\\|c\\|h\\)' -exec " \
    +clang_format+ \
    " -style=file -i {} \\;" \
  }  

# fast fmt(LinuxOS / macOS)(Install "cargo install fd-find")
fm_flags := "-e c \
  -e h \
  -e cpp \
  -e hpp \
  -e cppm \
  -e ixx \
  -e cc \
  -e cxx -x " \
  +clang_format+  \
  " -style=file -i {} \\;"
	
# g++ compile
r:
	just fm
	rm -rf target
	mkdir -p target
	g++ {{cflags}} -o target/{{project_name}} {{source}} {{ldflags}}
	if [ "{{os}}" = "Darwin" ]; then DYLD_LIBRARY_PATH=/opt/homebrew/lib target/{{project_name}}; else target/{{project_name}}; fi

# .clang-format fmt(LinuxOS/ macOS)
fmt:
	find {{fmt_flags}}

# (fast).clang-format fmt(cargo install fd-find)(LinuxOS / macOS)
fm:
	fd {{fm_flags}}

# clean files
clean:
	rm -rf {{target_dir}} *.out {{src_dir}}/*.out *.bc {{src_dir}}/target/ *.dSYM {{src_dir}}/*.dSYM *.i *.o *.s

# cmake compile(LinuxOS)
cr:
	just fm
	rm -rf build
	mkdir -p build
	export CXX={{gpp_which}}
	cmake -D CMAKE_CXX_COMPILER={{gpp_which}} -G Ninja .
	ninja
	mv build.ninja CMakeCache.txt CMakeFiles cmake_install.cmake target .ninja_deps .ninja_log build
	./build/target/{{project_name}}

# cmake compile(LinuxOS)
cro:
	rm -rf build
	mkdir -p build
	cmake -D CMAKE_BUILD_TYPE=RelWithDebInfo \
	      -D CMAKE_CXX_COMPILER={{gpp_which}} \
	      -D CMAKE_CXX_FLAGS_RELWITHDEBINFO_INIT="-O2 -g" \
	      -G Ninja .
	ninja
	mv build.ninja CMakeCache.txt CMakeFiles cmake_install.cmake target .ninja_deps .ninja_log build
	./build/{{target}}

# cmake compile(LinuxOS)
cro3:
	rm -rf build
	mkdir -p build
	cmake -D CMAKE_BUILD_TYPE=Release \
	      -D CMAKE_CXX_COMPILER={{gpp_which}} \
	      -D CMAKE_CXX_FLAGS_RELEASE_INIT="-O3 -DNDEBUG" \
	      -G Ninja .
	ninja
	mv build.ninja CMakeCache.txt CMakeFiles cmake_install.cmake target .ninja_deps .ninja_log build
	./build/{{target}}

# C++ init
init:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main() {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

# C++ init(int argc, char **argv)
init2:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main(int argc, char **argv) {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

# Debugging(VSCode)
vscode:
	rm -rf .vscode
	mkdir -p .vscode
	echo '{' > .vscode/launch.json
	echo '    "version": "0.2.0",' >> .vscode/launch.json
	echo '    "configurations": [' >> .vscode/launch.json
	echo '        {' >> .vscode/launch.json
	echo '            "type": "lldb",' >> .vscode/launch.json
	echo '            "request": "launch",' >> .vscode/launch.json
	echo '            "name": "Launch",' >> .vscode/launch.json
	echo '            "program": "${workspaceFolder}/target/${fileBasenameNoExtension}",' >> .vscode/launch.json
	echo '            "args": [],' >> .vscode/launch.json
	echo '            "cwd": "${workspaceFolder}",' >> .vscode/launch.json
	echo '            // "preLaunchTask": "C/C++: clang build active file"' >> .vscode/launch.json
	echo '        },' >> .vscode/launch.json
	echo '        {' >> .vscode/launch.json
	echo '            "name": "gcc - Build and debug active file",' >> .vscode/launch.json
	echo '            "type": "cppdbg",' >> .vscode/launch.json
	echo '            "request": "launch",' >> .vscode/launch.json
	echo '            "program": "${fileDirname}/target/${fileBasenameNoExtension}",' >> .vscode/launch.json
	echo '            "args": [],' >> .vscode/launch.json
	echo '            "stopAtEntry": false,' >> .vscode/launch.json
	echo '            "cwd": "${fileDirname}",' >> .vscode/launch.json
	echo '            "environment": [],' >> .vscode/launch.json
	echo '            "externalConsole": false,' >> .vscode/launch.json
	echo '            "MIMode": "lldb",' >> .vscode/launch.json
	echo '            // "tasks": "C/C++: clang build active file"' >> .vscode/launch.json
	echo '        }' >> .vscode/launch.json
	echo '    ]' >> .vscode/launch.json
	echo '}' >> .vscode/launch.json
	echo '{' > .vscode/tasks.json
	echo '    "tasks": [' >> .vscode/tasks.json
	echo '        {' >> .vscode/tasks.json
	echo '            "type": "cppbuild",' >> .vscode/tasks.json
	echo '            "label": "C/C++: clang build active file",' >> .vscode/tasks.json
	echo '            "command": "{{gpp_which}}",' >> .vscode/tasks.json
	echo '            "args": [' >> .vscode/tasks.json
	echo '                "-fcolor-diagnostics",' >> .vscode/tasks.json
	echo '                "-fansi-escape-codes",' >> .vscode/tasks.json
	echo '                "-g",' >> .vscode/tasks.json
	echo '                "${file}",' >> .vscode/tasks.json
	echo '                "-o",' >> .vscode/tasks.json
	echo '                "${fileDirname}/target/${fileBasenameNoExtension}"' >> .vscode/tasks.json
	echo '            ],' >> .vscode/tasks.json
	echo '            "options": {' >> .vscode/tasks.json
	echo '                "cwd": "${fileDirname}"' >> .vscode/tasks.json
	echo '            },' >> .vscode/tasks.json
	echo '            "problemMatcher": [' >> .vscode/tasks.json
	echo '                "$gcc"' >> .vscode/tasks.json
	echo '            ],' >> .vscode/tasks.json
	echo '            "group": {' >> .vscode/tasks.json
	echo '                "kind": "build",' >> .vscode/tasks.json
	echo '                "isDefault": true' >> .vscode/tasks.json
	echo '            },' >> .vscode/tasks.json
	echo '            "detail": "Task generated by Debugger."' >> .vscode/tasks.json
	echo '        }' >> .vscode/tasks.json
	echo '    ],' >> .vscode/tasks.json
	echo '    "version": "2.0.0"' >> .vscode/tasks.json
	echo '}' >> .vscode/tasks.json
```

