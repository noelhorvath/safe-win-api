use crate::{call, call_BOOL, core::Result};
use core::ptr;
use widestring::U16CStr;
use windows_sys::Win32::System::LibraryLoader::{
    FreeLibrary, GetModuleHandleW, LoadLibraryExW, LoadLibraryW, LOAD_LIBRARY_FLAGS,
};

pub fn free_library(module_handle: isize) -> Result<()> {
    call_BOOL! { FreeLibrary(module_handle) }
}

pub fn get_module_handle(module_name: Option<&U16CStr>) -> Result<isize> {
    call! { GetModuleHandleW(module_name.map_or(ptr::null(), |name| name.as_ptr())) != 0 }
}

pub fn load_library(file_name: &U16CStr) -> Result<isize> {
    call! { LoadLibraryW(file_name.as_ptr()) != 0 }
}

pub fn load_library_with_flags(file_name: &U16CStr, flags: LOAD_LIBRARY_FLAGS) -> Result<isize> {
    call! { LoadLibraryExW(file_name.as_ptr(), 0, flags) != 0 }
}
