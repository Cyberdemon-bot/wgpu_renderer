use std::ptr::NonNull;
use std::num::NonZeroIsize;

use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle,
    DisplayHandle, HandleError,
    HasDisplayHandle, HasWindowHandle,
    RawDisplayHandle, RawWindowHandle,
    WindowHandle,
    Win32WindowHandle, WindowsDisplayHandle,
};

use crate::{NativePlatform, NativeWindowHandle};

pub struct NativeWindow(pub NativeWindowHandle);

unsafe impl Send for NativeWindow {}
unsafe impl Sync for NativeWindow {}

impl HasWindowHandle for NativeWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.0.platform {
            NativePlatform::Macos => {
                let view = NonNull::new(self.0.window_handle)
                    .ok_or(HandleError::Unavailable)?;

                let handle = AppKitWindowHandle::new(view);

                Ok(unsafe {
                    WindowHandle::borrow_raw(
                        RawWindowHandle::AppKit(handle)
                    )
                })
            }

            NativePlatform::Windows => {
                let hwnd = NonZeroIsize::new(self.0.window_handle as isize)
                    .ok_or(HandleError::Unavailable)?;

                let handle = Win32WindowHandle::new(hwnd);

                Ok(unsafe {
                    WindowHandle::borrow_raw(
                        RawWindowHandle::Win32(handle)
                    )
                })
            }

            _ => Err(HandleError::NotSupported),
        }
    }
}

impl HasDisplayHandle for NativeWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match self.0.platform {
            NativePlatform::Macos => Ok(unsafe {
                DisplayHandle::borrow_raw(
                    RawDisplayHandle::AppKit(
                        AppKitDisplayHandle::new()
                    )
                )
            }),

            NativePlatform::Windows => Ok(unsafe {
                DisplayHandle::borrow_raw(
                    RawDisplayHandle::Windows(
                        WindowsDisplayHandle::new()
                    )
                )
            }),

            _ => Err(HandleError::NotSupported),
        }
    }
}