use crate::core::Result;
use crate::{call_BOOL, common::ToDebugString};
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, SetLastError, FILETIME};

pub use windows_sys::Win32::Foundation::{ERROR_INVALID_HANDLE, ERROR_INVALID_WINDOW_HANDLE};

impl ToDebugString for FILETIME {
    fn to_debug_string(&self) -> String {
        format!(
            "high_date_time: {}, low_date_time: {}",
            self.dwHighDateTime, self.dwLowDateTime
        )
    }
}

/// Closes the specified handle.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` has been already invalidated.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
///
pub fn close_handle(handle: isize) -> Result<()> {
    call_BOOL! { CloseHandle(handle) }
}

#[inline]
#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets the calling thread's last error code.
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror
///
pub fn get_last_error() -> u32 {
    unsafe { GetLastError() }
}

#[inline]
#[allow(clippy::undocumented_unsafe_blocks)]
/// Sets the calling thread's last error code.
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror
///
pub fn set_last_error(error_code: u32) {
    unsafe { SetLastError(error_code) }
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_BOOL {
    ($val:expr) => {
        if $val {
            1
        } else {
            0
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! from_BOOL {
    (! $val:expr) => {
        $val == 0
    };
    ($val:expr) => {
        $val != 0
    };
}
