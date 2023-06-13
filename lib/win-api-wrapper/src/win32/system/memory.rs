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
pub fn free_local(local_mem_handle: isize) -> Result<()> {
    call_num! { LocalFree(local_mem_handle) == 0 }
}

#[doc(hidden)]
#[macro_export]
macro_rules! free {
    (Local: $mem_ptr:expr) => {
        $crate::win32::system::memory::free_local(
            $crate::win32::system::memory::get_local_handle($mem_ptr.cast())?,
        )?;
    };
}
