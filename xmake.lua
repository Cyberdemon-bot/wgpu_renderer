add_rules("mode.debug", "mode.release")
add_requires("glfw")

target("rust_wgpu")
    set_kind("static")
    set_policy("build.fence", true)
    on_build(function (target)
        local args = {"build"}
        if is_mode("release") then
            table.insert(args, "--release")
        end
        os.execv("cargo", args, {curdir = "src/wgpu"})
        local rust_mode = is_mode("release") and "release" or "debug"
        local search_path = "src/wgpu/target/" .. rust_mode .. "/*.a"
        if is_plat("windows") then
            search_path = "src/wgpu/target/" .. rust_mode .. "/*.lib"
        end
        local match_files = os.files(search_path)
        if #match_files > 0 then
            os.cp(match_files[1], target:targetfile())
        else
            raise("Cannot find rust lib " .. search_path)
        end
    end)

target("native_window")
    set_kind("static")
    add_files("src/native_window/native_window.c")
    add_packages("glfw")
    if is_plat("macosx") then
        add_cxflags("-x objective-c")
        add_frameworks("Cocoa")
    end

target("App")
    set_kind("binary")
    add_files("src/main.cpp", {languages = "c++17"})
    add_packages("glfw")

    add_deps("rust_wgpu", "native_window")
    add_linkdirs("$(builddir)/$(plat)/$(arch)/$(mode)")
    add_links("rust_wgpu")

    if is_plat("windows") then
        add_syslinks(
            "kernel32",
            "user32",
            "gdi32",
            "shell32",
            "advapi32",
            "bcrypt",
            "ws2_32",
            "ntdll"
        )
    end

    if is_plat("windows") then
        add_syslinks("user32", "gdi32", "shell32")
    elseif is_plat("macosx") then
        add_frameworks("Cocoa", "IOKit", "CoreVideo", "Security", "SystemConfiguration", "Metal", "QuartzCore")
    elseif is_plat("linux") then
        add_syslinks("pthread", "dl", "X11", "Xrandr", "Xi")
    end