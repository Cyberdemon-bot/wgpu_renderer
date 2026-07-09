use std::sync::Mutex;

mod state;
mod texture;
pub mod native_window;

use state::State;

// ---------------------------------------------------------------------
// C ABI mirror of native_window.h's NativeWindowHandle / NativePlatform.
// Field order, types, and repr(C) must stay in sync with the C header.
// ---------------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum NativePlatform {
    Macos = 1,
    X11 = 2,
    Wayland = 3,
    Windows = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NativeWindowHandle {
    pub platform: NativePlatform,
    pub window_handle: *mut std::ffi::c_void,
    pub display_handle: *mut std::ffi::c_void,
}

// ---------------------------------------------------------------------
// Global renderer state.
//
// C++ owns the window and the event loop; Rust just owns the GPU state
// and is driven by explicit calls (init / resize / render / shutdown).
// A Mutex is used defensively in case C++ ever calls in from more than
// one thread; it is not expected to be contended in the common case.
// ---------------------------------------------------------------------
static STATE: Mutex<Option<State>> = Mutex::new(None);

fn log_panic_payload(payload: Box<dyn std::any::Any + Send>) {
    if let Some(s) = payload.downcast_ref::<&str>() {
        eprintln!("[rust_wgpu] panic: {s}");
    } else if let Some(s) = payload.downcast_ref::<String>() {
        eprintln!("[rust_wgpu] panic: {s}");
    } else {
        eprintln!("[rust_wgpu] panic: <non-string payload>");
    }
}

/// Initialize the GPU state from a native window handle provided by C++.
/// Returns true on success, false on failure. Never unwinds across FFI.
#[unsafe(no_mangle)]
pub extern "C" fn wgpu_init(handle: NativeWindowHandle, width: u32, height: u32) -> bool {
    let result = std::panic::catch_unwind(|| {
        pollster::block_on(State::new(handle, width, height))
    });

    match result {
        Ok(Ok(state)) => {
            *STATE.lock().unwrap() = Some(state);
            true
        }
        Ok(Err(e)) => {
            eprintln!("[rust_wgpu] init failed: {e}");
            false
        }
        Err(payload) => {
            log_panic_payload(payload);
            false
        }
    }
}

/// Notify Rust of a window resize. Safe to call every frame; it's cheap
/// to skip internally if the size hasn't changed, but currently always
/// reconfigures the surface for simplicity.
#[unsafe(no_mangle)]
pub extern "C" fn wgpu_resize(width: u32, height: u32) -> bool {
    let result = std::panic::catch_unwind(|| {
        if let Some(state) = STATE.lock().unwrap().as_mut() {
            state.resize(width, height);
        }
    });
    result.is_ok()
}

/// Render one frame. Returns false if rendering failed or state isn't
/// initialized, in which case C++ should treat it as non-fatal unless
/// it happens repeatedly.
#[unsafe(no_mangle)]
pub extern "C" fn wgpu_render() -> bool {
    let result = std::panic::catch_unwind(|| -> anyhow::Result<()> {
        let mut guard = STATE.lock().unwrap();
        if let Some(state) = guard.as_mut() {
            state.update();
            state.render()?;
        }
        Ok(())
    });

    match result {
        Ok(Ok(())) => true,
        Ok(Err(e)) => {
            eprintln!("[rust_wgpu] render error: {e}");
            false
        }
        Err(payload) => {
            log_panic_payload(payload);
            false
        }
    }
}

/// Tear down the GPU state. Safe to call multiple times.
#[unsafe(no_mangle)]
pub extern "C" fn wgpu_shutdown() {
    let _ = std::panic::catch_unwind(|| {
        *STATE.lock().unwrap() = None;
    });
}