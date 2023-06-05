use crate::call_num;
use crate::win32::core::Result;
use core::ffi::c_void;
use windows_sys::Win32::System::Memory::{LocalFree, LocalHandle};

/// Frees the local memory object specified by `local_handle` and invalidates it.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
pub fn local_free(local_handle: isize) -> Result<isize> {
    call_num! { LocalFree(local_handle) != 0 }
}

/// Gets the handle associated with the specified pointer to a local memory object.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
pub fn get_local_handle(mem_ptr: *const c_void) -> Result<isize> {
    call_num! { LocalHandle(mem_ptr) != 0 }
}
