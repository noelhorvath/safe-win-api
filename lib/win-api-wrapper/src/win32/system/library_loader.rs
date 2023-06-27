use crate::{call, call_BOOL, core::Result};
use core::ptr;
use widestring::U16CStr;
use windows_sys::Win32::System::LibraryLoader::{
    FreeLibrary, GetModuleHandleExW, GetModuleHandleW, LoadLibraryExW, LoadLibraryW,
    LOAD_LIBRARY_FLAGS,
};

pub use windows_sys::Win32::System::LibraryLoader::{
    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_PIN,
    GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, LOAD_LIBRARY_AS_DATAFILE,
    LOAD_LIBRARY_AS_IMAGE_RESOURCE, LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY,
    LOAD_LIBRARY_REQUIRE_SIGNED_TARGET, LOAD_LIBRARY_SAFE_CURRENT_DIRS,
    LOAD_LIBRARY_SEARCH_APPLICATION_DIR, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
    LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR, LOAD_LIBRARY_SEARCH_SYSTEM32,
    LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER, LOAD_LIBRARY_SEARCH_USER_DIRS,
};

/// Frees the loaded dynamic-link library (DLL) module and, if necessary, decrements
/// its reference count. When the reference count reaches zero, the module is unloaded
/// from the address space of the calling process and the handle is no longer valid.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `lib_module_handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary
///
pub fn free_library(lib_module_handle: isize) -> Result<()> {
    call_BOOL! { FreeLibrary(lib_module_handle) }
}

/// Gets a module handle for the specified module. The module must have been loaded by the calling process.
///
/// This function must be used carefully in a multithreaded application.
/// There is no guarantee that the module handle remains valid between
/// the time this function returns the handle and the time it is used.
/// To avoid such race conditions use [`get_module_handle_with_flags`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew
///
pub fn get_module_handle(module_name: Option<&U16CStr>) -> Result<isize> {
    call! { GetModuleHandleW(module_name.map_or(ptr::null(), |name| name.as_ptr())) != 0 }
}

/// Gets a module handle for the specified module and increments the module's reference count
/// unless [`GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT`] is specified.
/// The module must have been loaded by the calling process.
///
/// The GetModuleHandleEx function does not retrieve handles
/// for modules that were loaded using the [`LOAD_LIBRARY_AS_DATAFILE`] flag.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexa
///
pub fn get_module_handle_with_flags(module_name: Option<&U16CStr>, flags: u32) -> Result<isize> {
    call_BOOL! {
        GetModuleHandleExW(
            flags,
            module_name.map_or(ptr::null(), |name| name.as_ptr()),
            &mut module_handle,
        ) -> mut module_handle: isize
    }
}

/// Loads the module specified by `module_file_name` into the address space of the calling process.
/// The specified module may cause other modules to be loaded.
///
/// For additional options, use [`load_library_with_flags`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw
///
pub fn load_library(module_file_name: &U16CStr) -> Result<isize> {
    call! { LoadLibraryW(module_file_name.as_ptr()) != 0 }
}

/// Loads the module specified by `file_name` into the address space of the calling process.
/// The specified module may cause other modules to be loaded.
///
///
/// # Arguments
///
/// * `module_file_name`: A string that specifies the file name of the module to load.
/// * `flags`: The action to be taken when loading the module.
///
/// If no flags are specified (`falgs` is `0`), the behaviour of this function is identical to that of
/// [`load_library`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw
///
pub fn load_library_with_flags(
    module_file_name: &U16CStr,
    flags: LOAD_LIBRARY_FLAGS,
) -> Result<isize> {
    call! { LoadLibraryExW(module_file_name.as_ptr(), 0, flags) != 0 }
}
