use crate::core::Result;
use crate::{call_BOOL, call_HRESULT};
use core::ptr;
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::UI::Shell::{AssocQueryStringW, ShellExecuteExW, ASSOCF, ASSOCSTR};

pub use windows_sys::Win32::UI::Shell::{
    SEE_MASK_ASYNCOK, SEE_MASK_CLASSKEY, SEE_MASK_CLASSNAME, SEE_MASK_CONNECTNETDRV,
    SHELLEXECUTEINFOW,
};

pub fn get_assoc_string_len(
    search_flags: ASSOCF,
    string_type: ASSOCSTR,
    association_string: &U16CStr,
) -> Result<u32> {
    call_HRESULT! {
        AssocQueryStringW(
            search_flags,
            string_type,
            association_string.as_ptr(),
            ptr::null(),
            ptr::null_mut(),
            &mut char_count,
        ) -> mut char_count: u32
    }
}

pub fn get_asssoc_string(
    search_flags: ASSOCF,
    string_type: ASSOCSTR,
    association_string: &U16CStr,
    extra: Option<Box<&U16CStr>>,
) -> Result<U16CString> {
    let mut char_count = get_assoc_string_len(search_flags, string_type, association_string)?;
    let mut buffer = vec![0_u16; char_count as usize];
    call_HRESULT! {
        AssocQueryStringW(
            search_flags,
            string_type,
            association_string.as_ptr(),
            extra.map_or(ptr::null(), |ext| ext.as_ptr()),
            buffer.as_mut_ptr(),
            &mut char_count,
        ) return Error;
    }
    // Safety: `buffer` contains no interior null values
    Ok(unsafe { U16CString::from_vec_unchecked(buffer) })
}

pub fn execute(execute_info: &mut SHELLEXECUTEINFOW) -> Result<()> {
    call_BOOL! { ShellExecuteExW(execute_info) }
}
