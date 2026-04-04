cmake_minimum_required(VERSION 3.20)
project(vulkan001 VERSION 1.0.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Graphics source path - set this if graphics folder is elsewhere
set(RAPTOR_GRAPHICS_PATH "${CMAKE_CURRENT_SOURCE_DIR}/raptor/graphics" CACHE PATH "Path to raptor-graphics source folder")

# Find packages
find_package(Vulkan REQUIRED)

# SDL2
find_package(SDL2 REQUIRED)

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/raptor
    ${CMAKE_CURRENT_SOURCE_DIR}/external
    ${CMAKE_CURRENT_SOURCE_DIR}/external/cglm
    ${CMAKE_CURRENT_SOURCE_DIR}/external/cglm/struct
    ${CMAKE_CURRENT_SOURCE_DIR}/external/enkiTS
    ${CMAKE_CURRENT_SOURCE_DIR}/external/imgui
    ${Vulkan_INCLUDE_DIRS}
    ${SDL2_INCLUDE_DIRS}
)

# Graphics include (if folder exists)
if(EXISTS ${RAPTOR_GRAPHICS_PATH})
    include_directories(${RAPTOR_GRAPHICS_PATH})
    message(STATUS "Graphics folder found at: ${RAPTOR_GRAPHICS_PATH}")
else()
    message(WARNING "Graphics folder NOT found at: ${RAPTOR_GRAPHICS_PATH}")
    message(WARNING "Please set RAPTOR_GRAPHICS_PATH to the correct location using: cmake -DRAPTOR_GRAPHICS_PATH=/path/to/graphics ..")
endif()

# Source files
set(SOURCES
    # Main
    src/main.cpp

    # Raptor Foundation
    raptor/foundation/assert.cpp
    raptor/foundation/bit.cpp
    raptor/foundation/blob_serialization.cpp
    raptor/foundation/camera.cpp
    raptor/foundation/color.cpp
    raptor/foundation/data_structures.cpp
    raptor/foundation/file.cpp
    raptor/foundation/gltf.cpp
    raptor/foundation/log.cpp
    raptor/foundation/memory.cpp
    raptor/foundation/numerics.cpp
    raptor/foundation/process.cpp
    raptor/foundation/resource_manager.cpp
    raptor/foundation/serialization.cpp
    raptor/foundation/service.cpp
    raptor/foundation/service_manager.cpp
    raptor/foundation/string.cpp
    raptor/foundation/time.cpp

    # Raptor Application
    raptor/application/application.cpp
    raptor/application/game_application.cpp
    raptor/application/game_camera.cpp
    raptor/application/input.cpp
    raptor/application/keys.cpp
    raptor/application/window.cpp

    # External: Tracy profiler
    external/tracy/TracyClient.cpp
)

# Graphics source files (if folder exists)
if(EXISTS ${RAPTOR_GRAPHICS_PATH})
    file(GLOB GRAPHICS_SOURCES
        ${RAPTOR_GRAPHICS_PATH}/*.cpp
        ${RAPTOR_GRAPHICS_PATH}/vulkan/*.cpp
    )
    list(APPEND SOURCES ${GRAPHICS_SOURCES})
    message(STATUS "Added ${GRAPHICS_SOURCES} graphics source files")
endif()

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES})

# Link libraries
target_link_libraries(${PROJECT_NAME}
    PRIVATE
        Vulkan::Vulkan
        SDL2::SDL2
        SDL2::SDL2main
)

# Platform-specific libraries
if(APPLE)
    # macOS frameworks
    find_library(COCOA_LIBRARY Cocoa)
    find_library(IOKIT_LIBRARY IOKit)
    find_library(COREVIDEO_LIBRARY CoreVideo)
    find_library(QUARTZCORE_LIBRARY QuartzCore)
    find_library(METAL_LIBRARY Metal)
    find_library(APPSERVICES_LIBRARY ApplicationServices)

    target_link_libraries(${PROJECT_NAME}
        PRIVATE
            ${COCOA_LIBRARY}
            ${IOKIT_LIBRARY}
            ${COREVIDEO_LIBRARY}
            ${QUARTZCORE_LIBRARY}
            ${METAL_LIBRARY}
            ${APPSERVICES_LIBRARY}
    )
endif()

# Compiler warnings
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4 /WX-)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wno-unused-parameter)
endif()

# Copy assets (if any)
file(GLOB ASSETS "${CMAKE_CURRENT_SOURCE_DIR}/assets/*")
if(ASSETS)
    file(COPY ${ASSETS} DESTINATION ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/assets)
endif()
