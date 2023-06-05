use crate::call_BOOL;
use crate::win32::core::Result;
use alloc::boxed::Box;
use core::mem::{size_of, size_of_val};
use windows_sys::Win32::System::ProcessStatus::EnumProcesses;

/// Initial buffer size for [`get_pids`].
const DEFAULT_PID_BUFFER_SIZE: usize = 1024;
/// Maximum number of processes that [`EnumProcesses`] could return.
const MAX_PID_BUFFER_SIZE: usize = (u32::MAX >> 1) as usize + 1;

/// Gets the process identifier for each process in the system.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
/// # Examples
/// TODO
///
pub fn get_pids() -> Result<Box<[u32]>> {
    let mut buffer = vec![0_u32; DEFAULT_PID_BUFFER_SIZE];
    let mut bytes_written = 0;
    while buffer.len() < MAX_PID_BUFFER_SIZE {
        bytes_written = 0;
        call_BOOL!(EnumProcesses(
                buffer.as_mut_ptr(),
                (buffer.len() * size_of::<u32>()) as u32,
                &mut bytes_written) return Error);
        if bytes_written as usize / size_of::<u32>() < buffer.len() {
            break;
        }

        buffer.resize(buffer.capacity() * 2, 0);
    }

    let len = bytes_written as usize / size_of::<u32>();
    buffer.truncate(len);
    Ok(buffer.into_boxed_slice())
}

/// Copies the process identifier for each process in the system to `buffer`
/// and returns the number copied process ids.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocesses
pub fn get_pids_with_buffer(buffer: &mut [u32]) -> Result<usize> {
    let mut bytes_written = 0;
    call_BOOL!(EnumProcesses(
            buffer.as_mut_ptr(),
            size_of_val(buffer) as u32,
            &mut bytes_written) return Error);
    Ok(bytes_written as usize / size_of::<u32>())
}
