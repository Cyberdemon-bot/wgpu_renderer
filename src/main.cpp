#include <GLFW/glfw3.h>
#include "native_window/native_window.h"
#include "rust_header.h"
#include <iostream>

void error_callback(int error, const char* description)
{
    std::cerr << "GLFW Error [" << error << "]: " << description << std::endl;
}

int main()
{
    glfwSetErrorCallback(error_callback);
    if (!glfwInit())
    {
        std::cerr << "Failed to initialize GLFW!" << std::endl;
        return -1;
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

    GLFWwindow* window = glfwCreateWindow(800, 600, "App", nullptr, nullptr);
    if (!window)
    {
        std::cerr << "Failed to create GLFW window!" << std::endl;
        glfwTerminate();
        return -1;
    }

    NativeWindowHandle handle = native_get_window_handle(window);

    if (!wgpu_init(handle, 800, 600))
    {
        std::cerr << "Failed to initialize wgpu!" << std::endl;
        glfwDestroyWindow(window);
        glfwTerminate();
        return -1;
    }

    while (!glfwWindowShouldClose(window))
    {
        glfwPollEvents();

        int width, height;
        glfwGetFramebufferSize(window, &width, &height);
        wgpu_resize((unsigned int)width, (unsigned int)height);

        wgpu_render();
    }

    wgpu_shutdown();
    glfwDestroyWindow(window);
    glfwTerminate();

    return 0;
}