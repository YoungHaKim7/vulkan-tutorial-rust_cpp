cmake_minimum_required(VERSION 3.20)
project(gltf_loader VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Find tinygltf - using FetchContent for simplicity
include(FetchContent)

FetchContent_Declare(
    tinygltf
    GIT_REPOSITORY https://github.com/syoyo/tinygltf.git
    GIT_TAG v2.8.21
)

FetchContent_MakeAvailable(tinygltf)

# Create executable
add_executable(${PROJECT_NAME}
    src/main.cpp
)

# Include directories
target_include_directories(${PROJECT_NAME}
    PRIVATE
        ${tinygltf_SOURCE_DIR}
)

# Link libraries
target_link_libraries(${PROJECT_NAME}
    PRIVATE
        tinygltf
)

# Compiler warnings
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4 /WX-)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wno-unused-parameter)
endif()
