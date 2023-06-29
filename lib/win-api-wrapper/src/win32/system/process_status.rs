use crate::alloc::vec::Vec;
use crate::alloc::{self};
use crate::call_BOOL;
use crate::core::Result;
use core::mem::size_of;
use windows_sys::Win32::System::ProcessStatus::EnumProcesses;

/// The recommended value for `initial_buffer_size` in [`get_pids`].
pub const RECOMMENDED_INITIAL_PID_BUFFER_LENGTH: usize = 1024;
/// Maximum number of processes that [`EnumProcesses`] could enumerate.
pub const MAX_PID_BUFFER_LEN: usize = (u32::MAX >> 1) as usize + 1;

/// Gets the process identifier for each process in the system.
///
/// If `initial_buffer_len` is too small, the buffer is resized by `buffer.len() * 2` until
/// it is large enough to hold the process identifiers.
/// The maximum number of process identifiers that could be returned is [`MAX_PID_BUFFER_LEN`].
///
/// If the specidied length is larger than [`MAX_PID_BUFFER_LEN`], then [`MAX_PID_BUFFER_LEN`]
/// is used instead.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
pub fn get_pids(initial_buffer_len: u32) -> Result<Vec<u32>> {
    let mut buffer = alloc::vec![0_u32; initial_buffer_len as usize];
    let mut bytes_written = 0;
    while buffer.len() < MAX_PID_BUFFER_LEN {
        bytes_written = 0;
        call_BOOL! {
            EnumProcesses(
                buffer.as_mut_ptr(),
                (buffer.len() * size_of::<u32>()) as u32,
                &mut bytes_written,
            ) return Error;
        };
        if bytes_written as usize / size_of::<u32>() < buffer.len() {
            break;
        }

        buffer.resize(buffer.len() * 2, 0);
    }

    let len = bytes_written as usize / size_of::<u32>();
    buffer.truncate(len);
    Ok(buffer)
}

/// Copies the process identifier for each process in the system to `buffer`
/// and returns the number copied process identifiers.
///
/// # Remarks
///
/// * If the returned `usize` equals the length of `buffer`,
///   it probably means the there are more process identifiers. See the implementation of [`get_pids`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocesses
///
pub fn get_pids_with_buffer(buffer: &mut [u32]) -> Result<usize> {
    let mut bytes_written = 0;
    call_BOOL! {
        EnumProcesses(
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            &mut bytes_written,
        ) return Error;
    };
    Ok(bytes_written as usize / size_of::<u32>())
}
