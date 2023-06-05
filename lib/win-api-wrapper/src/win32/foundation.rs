use crate::call_BOOL;
use crate::win32::core::Result;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError};

pub use windows_sys::Win32::Foundation::{ERROR_INVALID_HANDLE, ERROR_INVALID_WINDOW_HANDLE};

/// Closes the specified handle.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
/// # Possible errors
///
/// - `handle` has been already invalidated.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
pub fn close_handle(handle: isize) -> Result<()> {
    call_BOOL! { CloseHandle(handle) }
}

#[inline]
#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets the calling thread's last Win32 error code.
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror
pub fn get_last_error() -> u32 {
    unsafe { GetLastError() }
}
