use core::mem::size_of;
use widestring::U16String;
use windows_sys::core::PCWSTR;

/// This trait defines a `to` method for `Borrowed` to `Owned` conversion between two types.
pub trait To<T> {
    /// Converts the borrowed type to an owned type of `T`.
    fn to(&self) -> T;
}

/// Defines a method for `Borrowed` to debug [`String`] conversion.
pub trait ToDebugString {
    /// Creates a debug [`String`] from `self`.
    fn to_debug_string(&self) -> String;
}

/// Returns a subslice of `slice` that doesn't contain any trailing whitespaces.
pub(crate) fn trim_wide_end(mut slice: &[u16]) -> &[u16] {
    while let Some(char) = slice.last() {
        match char {
            32 | 9..=13 => slice = &slice[..slice.len() - 1],
            _ => break,
        }
    }

    slice
}

/// Gets the length of a [`PCWSTR`][`windows_sys::core::PCWSTR`].
/// The ending null character is not included in the result.
///
/// # Safety
///
/// If any of the following conditions are violated, the result is Undefined Behavior:
///
/// * `pwstr` must point to an element of a wide string that is null-terminated.
///
pub(crate) unsafe fn get_pcwstr_len(pwstr: PCWSTR) -> usize {
    let mut len = 0;
    let type_size = size_of::<u16>();
    unsafe {
        while *pwstr.offset((len * type_size) as isize) != 0 {
            len += 1;
        }
    };
    len
}

/// Copies the elements of a `len` long [`PCWSTR`][`windows_sys::core::PCWSTR`] to a newly allocated [`U16String`].
/// If `trim_end` is `true` the end of the result string will be trimmed.
///
/// # Safety
///
/// If any of the following conditions are violated, the result is Undefined Behavior:
///
/// * `pwstr` must point to a `u16` value.
/// * `pwstr` must be valid for `len` number of consecutive values of type `u16`.
/// * `len * core::mem::size_of::<u16>()` must be no larger than `isize::MAX`.
///
pub(crate) unsafe fn pcwstr_to_u16_string(pwstr: PCWSTR, len: usize, trim_end: bool) -> U16String {
    let mut message_slice = unsafe { core::slice::from_raw_parts(pwstr, len) };
    if trim_end {
        message_slice = trim_wide_end(message_slice);
    }

    unsafe { U16String::from_ptr(message_slice.as_ptr(), message_slice.len()) }
}

#[doc(hidden)]
#[macro_export]
macro_rules! check_null_terminator {
    ($string_slice:expr, $error_code:expr) => {
        if *$string_slice.as_slice().last().unwrap_or(&1) != 0 {
            return Err($crate::win32::core::Win32Error::from_code($error_code));
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! check_param {
    ($str:ident as U16Str) => {
        $crate::check_null_terminator!(
            $str,
            windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER
        )
    };
}
