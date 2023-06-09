use crate::call_BOOL;
use crate::win32::core::Result;
use alloc::boxed::Box;
use core::mem::size_of;
use windows_sys::Win32::System::ProcessStatus::EnumProcesses;

/// The recommended value for `initial_buffer_size` in [`get_pids`].
pub const RECOMMENDED_INITIAL_PID_BUFFER_LENGTH: usize = 1024;
/// Maximum number of processes that [`EnumProcesses`] could enumerate.
const MAX_PID_BUFFER_LEN: usize = (u32::MAX >> 1) as usize + 1;

/// Gets the process identifier for each process in the system.
///
/// # Remarks
///
/// * If `initial_buffer_len` is too small, the buffer is resized by `buffer.len() * 2` until it is large enough to hold the process identifiers.
/// * The maximum number of process identifiers that could be returned is [`MAX_PID_BUFFER_LEN`].
/// * [`MAX_PID_BUFFER_LEN`] is used instead of `initial_buffer_len` to allocate the initial buffer if the specidied length is larger than [`MAX_PID_BUFFER_LEN`].
///
/// # Errors
///
/// Returns a [`Win32Error`][crate::win32::core::Win32Error] if the function fails.
///
/// # Examples
/// TODO
///
pub fn get_pids(initial_buffer_len: u32) -> Result<Box<[u32]>> {
    let mut buffer = vec![0_u32; initial_buffer_len as usize];
    let mut bytes_written = 0;
    while buffer.len() < MAX_PID_BUFFER_LEN {
        bytes_written = 0;
        call_BOOL! {
            EnumProcesses(
                buffer.as_mut_ptr(),
                (buffer.len() * size_of::<u32>()) as u32,
                &mut bytes_written)
            return Error;
        };
        if bytes_written as usize / size_of::<u32>() < buffer.len() {
            break;
        }

        buffer.resize(buffer.len() * 2, 0);
    }

    let len = bytes_written as usize / size_of::<u32>();
    buffer.truncate(len);
    Ok(buffer.into_boxed_slice())
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
/// Returns a [`Win32Error`][crate::win32::core::Win32Error] if the function fails.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocesses
///
pub fn get_pids_with_buffer(buffer: &mut [u32]) -> Result<usize> {
    let mut bytes_written = 0;
    call_BOOL! {
        EnumProcesses(
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            &mut bytes_written)
        return Error;
    };
    Ok(bytes_written as usize / size_of::<u32>())
}
