pub use windows_sys::core::GUID;

/// Contains the error type.
pub mod error;

pub type Result<T> = core::result::Result<T, super::core::error::Error>;

#[inline]
/// Creates a [`GUID`] from an array of `16` bytes.
pub fn guid_from_array(bytes: [u8; 16]) -> GUID {
    GUID {
        data1: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        data2: u16::from_le_bytes([bytes[4], bytes[5]]),
        data3: u16::from_le_bytes([bytes[6], bytes[7]]),
        data4: [
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ],
    }
}

/// Gets the length from the bytes of an UTF-16 encoded string.
pub fn get_utf16_string_len_from_bytes(bytes: &[u8]) -> usize {
    let mut len = 0;
    if bytes.len() >= 2 && bytes[0] != 0 {
        len += 1;
    }

    if bytes.len() <= 2 {
        return len;
    }

    let bytes_slice = if bytes.len() % 2 == 0 {
        bytes
    } else {
        &bytes[..bytes.len() - 1]
    };
    let mut i = 1; // == index of last checked u16 byte's lower byte => [0x11, 0x00] -> 1
    while i < bytes.len() {
        i += 1;
        if bytes_slice[i] == 0 {
            break;
        }

        i += 1;
        len += 1;
    }
    len
}
