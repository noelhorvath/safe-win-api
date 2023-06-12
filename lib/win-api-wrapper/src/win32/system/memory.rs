use crate::call_num;
use crate::core::Result;
use core::ffi::c_void;
use windows_sys::Win32::System::Memory::{LocalFree, LocalHandle};

/// Gets the handle associated with the specified pointer to a local memory object.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocesses
///
pub fn get_local_handle(mem_ptr: *const c_void) -> Result<isize> {
    call_num! { LocalHandle(mem_ptr) != 0 }
}

/// Frees the local memory object specified by `local_handle` and invalidates it.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree
///
pub fn local_free(local_handle: isize) -> Result<()> {
    call_num! { LocalFree(local_handle) == 0 }
}
