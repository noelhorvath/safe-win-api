use crate::core::Result;
use crate::{call, call_HRESULT};
use core::ptr;
use std::os::raw::c_void;
use windows_sys::Win32::System::Com::{CoInitializeEx, CoTaskMemAlloc, CoTaskMemFree};

pub use windows_sys::Win32::System::Com::{
    COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, COINIT_MULTITHREADED,
    COINIT_SPEED_OVER_MEMORY,
};

/// Initializes the `COM` library for use by the calling thread,
/// sets the thread's concurrency model, and creates a new apartment
/// for the thread if one is required.
///
/// [`initialize`] must be called at least once, and is usually called only once,
/// for each thread that uses the `COM` library.
///
/// # Arguments
///
/// * `options`: The concurrency model and initialization options for the thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex
///
pub fn initialize(options: i32) -> Result<()> {
    call_HRESULT! { CoInitializeEx(ptr::null(), options) }
}

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
