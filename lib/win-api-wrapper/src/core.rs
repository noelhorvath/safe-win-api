pub use windows_sys::core::GUID;

pub mod error;

pub type Result<T> = core::result::Result<T, super::core::error::Error>;

#[inline]
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

pub fn get_string_len_from_bytes(buffer: &[u8]) -> usize {
    let mut len = 0;
    if buffer.len() >= 2 && buffer[0] != 0 {
        len += 1;
    }

    if buffer.len() <= 2 {
        return len;
    }

    let buffer_slice = if buffer.len() % 2 == 0 {
        buffer
    } else {
        &buffer[..buffer.len() - 1]
    };
    let mut i = 1; // == index of last checked u16 byte's lower byte => [0x11, 0x00] -> 1
    while i < buffer.len() {
        i += 1;
        if buffer_slice[i] == 0 {
            break;
        }

        i += 1;
        len += 1;
    }
    len
}
