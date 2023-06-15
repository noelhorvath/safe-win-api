use crate::core::Result;
use core::ptr;
use std::os::raw::c_void;
use windows_sys::Win32::System::Com::{CoTaskMemAlloc, CoTaskMemFree};

use crate::call;

pub fn mem_alloc(size: usize) -> Result<*mut c_void> {
    call! { CoTaskMemAlloc(size) != ptr::null_mut() }
}

pub fn mem_free(ptr: *const c_void) {
    unsafe { CoTaskMemFree(ptr) }
}
