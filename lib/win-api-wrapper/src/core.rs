pub use crate::U16CString;
pub use core::mem::size_of;
pub use windows_sys::core::GUID;

/// Contains the error type.
pub mod error;

pub type Result<T> = core::result::Result<T, super::core::error::Error>;

/// This trait defines a `to` method for `Borrowed` to `Owned` conversion between two types.
pub trait To<T> {
    /// Converts the borrowed type to an owned type of `T`.
    fn to(&self) -> T;
}

#[inline]
/// Creates a [`GUID`] from an array of `16` bytes.
pub fn guid_from_array(bytes: &[u8; 16]) -> GUID {
    GUID {
        data1: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        data2: u16::from_le_bytes([bytes[4], bytes[5]]),
        data3: u16::from_le_bytes([bytes[6], bytes[7]]),
        data4: [
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ],
    }
}

/// Gets the length from the bytes of a wide string in characters.
/// The returned length does not include the null terminator (`\0`).
pub fn get_wide_string_len_from_bytes(bytes: &[u8]) -> usize {
    let mut len = 0;
    if bytes.len() >= 2 && bytes[0] != 0 {
        len += 1;
    }

    if bytes.len() <= 2 {
        return len;
    }

    // The slice should contain even number of bytes, because each wide character consists of 2 bytes.
    let bytes_slice = if bytes.len() % 2 == 0 {
        bytes
    } else {
        // If the slice has odd number of bytes, just exclude the last byte of the byte sequence.
        // It is safe to exclude the last byte, beacuse the slice contains atleast 3 bytes.
        &bytes[..bytes.len() - 1]
    };
    let mut i = 2; // index of the first byte of the next u16 character to be checked
    while i < bytes.len() {
        // if both bytes of the wide character are 0, the character is a null terminator ([0, 0])
        if bytes_slice[i] == 0 && bytes_slice[i + 1] == 0 {
            break;
        }

        i += 2;
        len += 1;
    }
    len
}

/// Gets the number of strings that the multi-string byte sequence contains.
pub fn get_multi_string_bytes_count(mut bytes: &[u8]) -> usize {
    let mut count = 0;
    loop {
        let next_string_bytes_len = get_wide_string_len_from_bytes(bytes) * size_of::<u16>();
        if next_string_bytes_len == 0 {
            break;
        }

        let start_index_of_next_string = next_string_bytes_len + 2;
        bytes = &bytes[start_index_of_next_string..];
        count += 1;
    }

    count
}

/// Extracts each string from the multi-string byte sequence and returns
/// them in a [vector][alloc::vec::Vec].
pub fn multi_string_bytes_to_strings(mut bytes: &[u8]) -> Vec<U16CString> {
    let string_count = get_multi_string_bytes_count(bytes);
    let mut strings = Vec::with_capacity(string_count);
    let mut i = 0;
    while i < string_count {
        let len = get_wide_string_len_from_bytes(bytes);
        let string = unsafe { U16CString::from_ptr_unchecked(bytes.as_ptr().cast(), len) };
        strings.push(string);

        let start_index_of_next_string = (len + 1) * size_of::<u16>();
        bytes = &bytes[start_index_of_next_string..];
        i += 1;
    }

    strings
}
