use crate::core::Result;
use crate::default_sized;
use crate::{call, call_BOOL};
use windows_sys::Win32::Foundation::ERROR_NO_MORE_FILES;
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, Thread32First, Thread32Next,
};

pub use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CREATE_TOOLHELP_SNAPSHOT_FLAGS, PROCESSENTRY32W, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD,
    THREADENTRY32,
};

/// Takes a snapshot of the specified processes, as well as the heaps, modules, and threads used by these processes.
///
/// # Arguments
///
/// * `flags`: The portions of the system to be included in the snapshot.
/// * `pid`: The process identifier of the process to be included in the snapshot.
///
/// # Remarks
///
/// * `pid` parameter can be zero to indicate the current process.
/// * `pid` parameter is used when the
///   [`TH32CS_SNAPHEAPLIST`][windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPHEAPLIST],
///   [`TH32CS_SNAPMODULE`][windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPMODULE],
///   [`TH32CS_SNAPMODULE32`][windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPMODULE32],
///   or [`TH32CS_SNAPALL`][windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPALL] value is specified.
///   Otherwise, it is ignored and all processes are included in the snapshot.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_ACCESS_DENIED`][`windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED`]
///     * `pid` is the idle process or one of the CSRSS processes.
/// * [`ERROR_PARTIAL_COPY`][`windows_sys::Win32::Foundation::ERROR_PARTIAL_COPY`]
///     * `pid` is a 64-bit process and the caller is a 32-bit process.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot
///
pub fn create_snapshot(flags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, pid: u32) -> Result<isize> {
    call! { CreateToolhelp32Snapshot(flags, pid) != INVALID_HANDLE_VALUE }
}

/// Retrieves information about the first process recorded in the specified system snapshot.
/// If the snapshot doesn't contain any processes the return value is [`None`].
///
/// # Remarks
///
/// * [`first_process`] must be called once before calling [`next_process`] to successfully iterate over the processes.
///   Otherwise [`next_process`] will only yield `None`.
/// * Calling [`first_process`] resets the enumeration.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw
///
pub fn first_process(handle: isize) -> Result<Option<PROCESSENTRY32W>> {
    call_BOOL! {
        Process32FirstW(handle, &mut entry) -> Result<Option> {
            mut entry = default_sized!(Dw -> mut entry: PROCESSENTRY32W);
            ERROR_NO_MORE_FILES => None;
        }
    }
}

/// Retrieves information about the next process recorded in the specified system snapshot.
/// If the the snapshot has no processes or the last process was enumerated the return value is [`None`].
///
/// # Remarks
///
/// * [`first_process`] must be called once before calling [`next_process`] to successfully iterate over the processes.
///   Otherwise [`next_process`] will only yield [`None`].
/// * Calling [`first_process`] resets the enumeration.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw
///
pub fn next_process(handle: isize) -> Result<Option<PROCESSENTRY32W>> {
    call_BOOL! {
        Process32NextW(handle, &mut entry) -> Result<Option> {
            mut entry = default_sized!(Dw -> mut entry: PROCESSENTRY32W);
            ERROR_NO_MORE_FILES => None;
        }
    }
}

/// Retrieves information about the first thread of any process encountered in the specified system snapshot.
/// If the snapshot doesn't contain any threads the return value is [`None`].
///  
/// # Remarks
///
/// * [`first_thread`] must be called once before calling [`next_thread`] to successfully iterate over the processes.
///   Otherwise [`next_thread`] will only yield [`None`].
/// * Calling [`first_thread`] resets the enumeration.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32first
///
pub fn first_thread(handle: isize) -> Result<Option<THREADENTRY32>> {
    call_BOOL! {
        Thread32First(handle, &mut entry) -> Result<Option> {
            mut entry = default_sized!(Dw -> mut entry: THREADENTRY32);
            ERROR_NO_MORE_FILES => None;
        }
    }
}

/// Retrieves information about the next thread of any process encountered in the specified system snapshot.
/// If the the snapshot has no threads or the last thread was enumerated the return value is [`None`].
///
/// # Remarks
///
/// * [`first_thread`] must be called once before calling [`next_thread`] to successfully iterate over the processes.
///   Otherwise [`next_thread`] will only yield [`None`].
/// * Calling [`first_thread`] resets the enumeration.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32next
///
pub fn next_thread(handle: isize) -> Result<Option<THREADENTRY32>> {
    call_BOOL! {
        Thread32Next(handle, &mut entry) -> Result<Option> {
            mut entry = default_sized!(Dw -> mut entry: THREADENTRY32);
            ERROR_NO_MORE_FILES => None;
        }
    }
}
