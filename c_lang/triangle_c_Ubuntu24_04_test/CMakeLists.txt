cmake_minimum_required(VERSION 3.20)

get_filename_component(ProjectId ${CMAKE_CURRENT_SOURCE_DIR} NAME)
string(REPLACE " " "_" ProjectId ${ProjectId})
project(${ProjectId} LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# GLFW via pkg-config (system package)
find_package(PkgConfig REQUIRED)
pkg_check_modules(GLFW REQUIRED IMPORTED_TARGET glfw3)

# Wayland client headers (openSUSE installs them under /usr/include/wayland)
pkg_check_modules(WAYLAND REQUIRED IMPORTED_TARGET wayland-client)

# Distro GLFW builds may omit the Wayland backend entirely (e.g. Ubuntu's
# libglfw is X11-only), in which case the Wayland native access symbols do
# not exist. Probe for them instead of using the GLFW 3.4-only
# glfwGetPlatform() API.
include(CheckCSourceCompiles)
set(CMAKE_REQUIRED_LIBRARIES PkgConfig::GLFW PkgConfig::WAYLAND)
check_c_source_compiles(
    "
    #define GLFW_EXPOSE_NATIVE_WAYLAND
    #include <GLFW/glfw3.h>
    #include <GLFW/glfw3native.h>
    int main(void) { return glfwGetWaylandDisplay() != 0; }
    "
    GLFW_HAS_WAYLAND_NATIVE)
unset(CMAKE_REQUIRED_LIBRARIES)

# Vendored wgpu-native ffi (headers + prebuilt library, wgpu-native v29.0.1.1)
set(WGPU_FFI ${CMAKE_CURRENT_SOURCE_DIR}/include/vendor/ffi)
set(WGPU_LIBRARY ${WGPU_FFI}/lib/libwgpu_native.so)

if (NOT EXISTS "${WGPU_LIBRARY}")
    message(FATAL_ERROR
        "libwgpu_native.so not found at ${WGPU_LIBRARY}\n"
        "The prebuilt library is not tracked by git (.gitignore: *.so). "
        "Fetch it from the wgpu-native release:\n"
        "  curl -sL https://github.com/gfx-rs/wgpu-native/releases/download/"
        "v29.0.1.1/wgpu-linux-x86_64-release.zip -o /tmp/wgpu.zip\n"
        "  unzip -o -j /tmp/wgpu.zip 'lib/libwgpu_native.so' -d include/vendor/ffi/lib")
endif()

if (MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# Example framework (shader loading, logging, adapter info printing)
add_library(framework include/framework/framework.c)
target_include_directories(framework PUBLIC
    ${WGPU_FFI}
    ${WGPU_FFI}/webgpu-headers
)
target_link_libraries(framework ${WGPU_LIBRARY})

# Main target
add_executable(${ProjectId} src/main.c)
target_include_directories(${ProjectId} PRIVATE
    include/framework
    ${WGPU_FFI}
    ${WGPU_FFI}/webgpu-headers
)

if (WIN32)
    target_compile_definitions(${ProjectId} PRIVATE GLFW_EXPOSE_NATIVE_WIN32)
    set(OS_LIBRARIES userenv ws2_32 bcrypt)
elseif(UNIX AND NOT APPLE)
    target_compile_definitions(${ProjectId} PRIVATE GLFW_EXPOSE_NATIVE_X11)
    if (GLFW_HAS_WAYLAND_NATIVE)
        target_compile_definitions(${ProjectId} PRIVATE GLFW_EXPOSE_NATIVE_WAYLAND)
    endif()
    set(OS_LIBRARIES m dl pthread)
elseif(APPLE)
    target_compile_definitions(${ProjectId} PRIVATE GLFW_EXPOSE_NATIVE_COCOA)
    set(OS_LIBRARIES "-framework CoreFoundation" "-framework QuartzCore" "-framework Metal")
endif()

target_link_libraries(${ProjectId} framework PkgConfig::GLFW ${OS_LIBRARIES})

if (UNIX AND NOT APPLE)
    # glfw3native.h needs the Wayland client headers
    target_link_libraries(${ProjectId} PkgConfig::WAYLAND)
endif()

# Keep libwgpu_native.so findable at runtime
set_target_properties(${ProjectId} PROPERTIES
    BUILD_RPATH "${WGPU_FFI}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/$<LOWER_CASE:$<CONFIG>>"
)

# The example loads "shader.wgsl" from the working directory
add_custom_command(TARGET ${ProjectId} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
            ${CMAKE_CURRENT_SOURCE_DIR}/src/shader.wgsl
            $<TARGET_FILE_DIR:${ProjectId}>/shader.wgsl
)
