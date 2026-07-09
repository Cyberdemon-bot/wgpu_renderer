#include "native_window.h"

#define NATIVE_TARGET_MACOS 1
#define NATIVE_TARGET_X11 2
#define NATIVE_TARGET_WAYLAND 3
#define NATIVE_TARGET_WINDOWS 4

#if defined(_WIN32)
    #define NATIVE_TARGET NATIVE_TARGET_WINDOWS
#elif defined(__APPLE__)
    #define NATIVE_TARGET NATIVE_TARGET_MACOS
#elif defined(_GLFW_WAYLAND)
    #define NATIVE_TARGET NATIVE_TARGET_WAYLAND
#else
    #define NATIVE_TARGET NATIVE_TARGET_X11
#endif

#if NATIVE_TARGET == NATIVE_TARGET_MACOS
    #define GLFW_EXPOSE_NATIVE_COCOA
    #import <Cocoa/Cocoa.h>
#elif NATIVE_TARGET == NATIVE_TARGET_X11
    #define GLFW_EXPOSE_NATIVE_X11
#elif NATIVE_TARGET == NATIVE_TARGET_WAYLAND
    #define GLFW_EXPOSE_NATIVE_WAYLAND
#elif NATIVE_TARGET == NATIVE_TARGET_WINDOWS
    #define GLFW_EXPOSE_NATIVE_WIN32
#endif

#include <GLFW/glfw3native.h>

NativeWindowHandle native_get_window_handle(GLFWwindow* window) 
{
    NativeWindowHandle handle = {0};

#if NATIVE_TARGET == NATIVE_TARGET_MACOS
    NSWindow* ns_window = glfwGetCocoaWindow(window);
    handle.platform = NATIVE_PLATFORM_MACOS;
    handle.window_handle = (__bridge void*)ns_window.contentView;
    handle.display_handle = NULL;

#elif NATIVE_TARGET == NATIVE_TARGET_X11
    handle.platform = NATIVE_PLATFORM_X11;
    handle.window_handle = (void*)(uintptr_t)glfwGetX11Window(window);
    handle.display_handle = (void*)glfwGetX11Display();

#elif NATIVE_TARGET == NATIVE_TARGET_WAYLAND
    handle.platform = NATIVE_PLATFORM_WAYLAND;
    handle.window_handle = (void*)glfwGetWaylandWindow(window);
    handle.display_handle = (void*)glfwGetWaylandDisplay();

#elif NATIVE_TARGET == NATIVE_TARGET_WINDOWS
    handle.platform = NATIVE_PLATFORM_WINDOWS;
    handle.window_handle = (void*)glfwGetWin32Window(window);
    handle.display_handle = NULL;
#endif

    return handle;
}