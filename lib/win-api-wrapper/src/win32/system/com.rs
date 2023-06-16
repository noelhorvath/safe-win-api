use crate::call;
use crate::core::Result;
use core::ptr;
use std::os::raw::c_void;
use windows_sys::Win32::System::Com::{CoTaskMemAlloc, CoTaskMemFree};

/// Allocates an uninitialized block of task memory.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * There's no sufficient memory available.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemalloc
///
pub fn mem_alloc(size: usize) -> Result<*mut c_void> {
    call! { CoTaskMemAlloc(size) != ptr::null_mut() }
}

/// Frees a block of task memory previously allocated by [`mem_alloc`].
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree
///
pub fn mem_free(ptr: *const c_void) {
    unsafe { CoTaskMemFree(ptr) }
}
