use widestring::U16String;
use windows_sys::core::PCWSTR;

/// This trait defines a `to` method for Borrowed -> Owned conversion between two types.
pub trait To<T> {
    /// Converts the borrowed type to an owned type of `T`.
    fn to(&self) -> T;
}

/// Returns a subslice of `slice` that doesn't contain any trailing whitespaces or null characters at the end.
pub(crate) fn trim_wide_end(mut slice: &[u16]) -> &[u16] {
    while let Some(char) = slice.last() {
        match char {
            0 | 32 | 9..=13 => slice = &slice[..slice.len() - 1],
            _ => break,
        }
    }

    slice
}

/// Copies the elements of a `len` long [`PCWSTR`][`windows_sys::core::PCWSTR`] to a newly allocated [`U16String`].
/// If `trim_end` is `true` the end of the result string will be trimmed.
///
/// # Safety:
///
/// * `pwstr` must point to a `u16` value.
/// * `pwstr` must be valid for `len` number of consecutive values of type `u16`.
/// * `len * core::mem::size_of::<u16>()` must be no larger than `isize::MAX`.
///
pub(crate) unsafe fn pcwstr_to_u16_string(pwstr: PCWSTR, len: usize, trim_end: bool) -> U16String {
    let mut message_slice = unsafe { core::slice::from_raw_parts(pwstr, len) }; // '\0' is ignored
    if trim_end {
        message_slice = trim_wide_end(message_slice);
    }

    unsafe { U16String::from_ptr(message_slice.as_ptr(), message_slice.len()) }
}
