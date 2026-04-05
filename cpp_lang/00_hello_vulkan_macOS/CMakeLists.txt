cmake_minimum_required(VERSION 3.10)
project(HelloVulkan)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include directories
include_directories(/opt/homebrew/include)

# Link directories
link_directories(/opt/homebrew/lib)

# Source files
add_executable(main src/main.cpp)

# Link libraries
target_link_libraries(main
    glfw
    vulkan
    "-framework Cocoa"
    "-framework IOKit"
    "-framework CoreVideo"
)

# Set output directory
set_target_properties(main PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/target"
)

# Optimization flags for Release build
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -DNDEBUG")
