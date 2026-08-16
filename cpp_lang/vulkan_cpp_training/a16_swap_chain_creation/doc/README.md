No. The **compile/build succeeded** — the **run crashed**:

```
glfwCreateWindowSurface: Assertion `window != NULL' failed.
Aborted (core dumped)
```

## What actually happened

In `src/main.cpp:188-193`, `createSurface()` calls `glfwCreateWindowSurface(instance, window, ...)`, but `window` is `NULL`. That means `glfwCreateWindow` at `src/main.cpp:84` failed and returned `NULL` — and its return value (like `glfwInit()`'s at line 79) is never checked, so the failure went silent until the assert inside GLFW killed the process.

The supporting evidence: the Vulkan instance **did** get created (you can see the validation layer's ICD-loading messages), so the crash is purely on the GLFW/window side.

Also note: `-D VKB_WSI_SELECTION=WAYLAND` did nothing — CMake explicitly warned `Manually-specified variables were not used by the project`. That variable belongs to the Vulkan SDK's `vulkan-bootstrap`-style projects, not this one, and GLFW picks its platform (X11/Wayland) itself.

## Likely causes

Since `vkCreateInstance` worked but the window didn't, the usual suspects are:

1. `glfwInit()` failed (no usable display server connection — e.g. neither `$DISPLAY` nor `$WAYLAND_DISPLAY` is set in the environment you launched from)
2. `glfwCreateWindow()` failed on the NVIDIA + Wayland/X11 combination

## How to find out which

Add a GLFW error callback and check the returns — this will tell you the exact reason to stderr:

```cpp
void initWindow() {
    glfwSetErrorCallback([](int code, const char *desc) {
        std::cerr << "GLFW error " << code << ": " << desc << std::endl;
    });

    if (!glfwInit()) {
        throw std::runtime_error("glfwInit failed!");
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

    window = glfwCreateWindow(WIDTH, HEIGHT, "Vulkan", nullptr, nullptr);
    if (window == nullptr) {
        throw std::runtime_error("glfwCreateWindow failed!");
    }
}
```

You can also quickly sanity-check the environment you're launching from:

```fish
echo $DISPLAY $WAYLAND_DISPLAY
```

If both are empty (e.g. launching from an SSH session or a clean environment), that's your answer. Want me to apply the error-checking patch to `main.cpp`?

# fix
Fixed. The problem was a stray `}` at `src/main.cpp:109` — it closed the `HelloTriangleApplication` class right after `initWindow()`, so `initVulkan`, `mainLoop`, `cleanup`, etc. ended up at namespace scope where they couldn't see the member variables or each other. That one stray brace caused all the cascading "not declared in this scope" errors.

Changes:
- Removed the stray `}` after `initWindow()` so all functions stay inside the class
- Joined the dangling `}` + `;` at the end into a proper `};` class terminator

`just fm` + `cmake --build` now pass cleanly. `just cr` will also launch the windowed app, so run that when you want to see it live.
