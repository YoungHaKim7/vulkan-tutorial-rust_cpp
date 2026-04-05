cmake_minimum_required(VERSION 3.10)

get_filename_component(ProjectId ${CMAKE_CURRENT_SOURCE_DIR} NAME)
string(REPLACE " " "_" ProjectId ${ProjectId})
project(${ProjectId} LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Platform-specific settings
if(${CMAKE_SYSTEM_NAME} STREQUAL "Linux")
    # Linux: use system paths
    include_directories(/usr/include)
    link_directories(/usr/lib)
elseif(${CMAKE_SYSTEM_NAME} STREQUAL "Darwin")
    # macOS: use Homebrew paths
    include_directories(/opt/homebrew/include)
    link_directories(/opt/homebrew/lib)
endif()

# Source files
add_executable(main src/main.cpp)

# Link libraries
if(${CMAKE_SYSTEM_NAME} STREQUAL "Linux")
    target_link_libraries(main
        glfw
        vulkan
        dl
        X11
        pthread
    )
elseif(${CMAKE_SYSTEM_NAME} STREQUAL "Darwin")
    target_link_libraries(main
        glfw
        vulkan
        "-framework Cocoa"
        "-framework IOKit"
        "-framework CoreVideo"
    )
endif()

# Set output directory
set_target_properties(main PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/target"
)

# Optimization flags for Release build
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -DNDEBUG")
