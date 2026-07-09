#pragma once
#include <GLFW/glfw3.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    NATIVE_PLATFORM_MACOS = 1,
    NATIVE_PLATFORM_X11 = 2,
    NATIVE_PLATFORM_WAYLAND = 3,
    NATIVE_PLATFORM_WINDOWS = 4,
} NativePlatform;

typedef struct {
    NativePlatform platform;
    void* window_handle;   
    void* display_handle;  
} NativeWindowHandle;

NativeWindowHandle native_get_window_handle(GLFWwindow* window);

#ifdef __cplusplus
}
#endif