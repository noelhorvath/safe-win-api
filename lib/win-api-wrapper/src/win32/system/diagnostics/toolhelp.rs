use crate::default_sized;
use crate::win32::core::Result;
use crate::{call_BOOL, call_num};
use core::mem::size_of;
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
/// * `pid` parameter is used when the [TH32CS_SNAPHEAPLIST], [TH32CS_SNAPMODULE], [TH32CS_SNAPMODULE32], or [TH32CS_SNAPALL] value is specified.
///   Otherwise, it is ignored and all processes are included in the snapshot.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
/// ## Possible errors
///
/// * `pid` is the idle process or one of the CSRSS processes. ([ERROR_ACCESS_DENIED])
/// * `pid` is a 64-bit process and the caller is a 32-bit process. ([ERROR_PARTIAL_COPY])
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot
/// [ERROR_ACCESS_DENIED]: windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED
/// [ERROR_PARTIAL_COPY]: windows_sys::Win32::Foundation::ERROR_PARTIAL_COPY
/// [TH32CS_SNAPHEAPLIST]: windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPHEAPLIST
/// [TH32CS_SNAPMODULE]: windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPMODULE
/// [TH32CS_SNAPMODULE32]: windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPMODULE32
/// [TH32CS_SNAPALL]: windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPALL
///
pub fn create_snapshot(flags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, pid: u32) -> Result<isize> {
    call_num! { CreateToolhelp32Snapshot(flags, pid) != INVALID_HANDLE_VALUE }
}

/// Retrieves information about the first process recorded in the specified system snapshot.
/// If the snapshot doesn't contain any process the return value is [`None`].
///
/// # Remarks
///
/// * [`first_process`] must be called once before calling [`next_process`] to successfully iterate over the processes.
///   Otherwise [`next_process`] will only yield `None`.
/// * Each invokation of [`first_process`] resets the iteration to the first entry if there's any.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
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
            mut entry = default_sized!(mut PROCESSENTRY32W: SnapshotEntry);
            ERROR_NO_MORE_FILES => None;
        }
    }
}

/// Retrieves information about the next process recorded in the specified system snapshot.
/// If the snapshot doesn't contain any more processes the return value is [`None`].
///
/// # Remarks
///
/// * [`first_process`] must be called once before calling [`next_process`] to successfully iterate over the processes.
///   Otherwise [`next_process`] will only yield [`None`].
/// * Each invokation of [`first_process`] resets the iteration to the first entry if there's any.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
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
            mut entry = default_sized!(mut PROCESSENTRY32W: SnapshotEntry);
            ERROR_NO_MORE_FILES => None;
        }
    }
}

/// Retrieves information about the first thread of any process encountered in the specified system snapshot.
/// If the snapshot doesn't contain any thread the return value is [`None`].
///  
/// # Remarks
///
/// * [`first_thread`] must be called once before calling [`next_thread`] to successfully iterate over the processes.
///   Otherwise [`next_thread`] will only yield [`None`].
/// * Each invokation of [`first_thread`] resets the iteration to the first entry if there's any.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
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
            mut entry = default_sized!(mut THREADENTRY32: SnapshotEntry);
            ERROR_NO_MORE_FILES => None;
        }
    }
}

/// Retrieves information about the next thread of any process encountered in the specified system snapshot.
/// If the snapshot doesn't contain any more threads the return value is [`None`].
///
/// # Remarks
///
/// * [`first_thread`] must be called once before calling [`next_thread`] to successfully iterate over the processes.
///   Otherwise [`next_thread`] will only yield [`None`].
/// * Each invokation of [`first_thread`] resets the iteration to the first entry if there's any.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
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
            mut entry = default_sized!(mut THREADENTRY32: SnapshotEntry);
            ERROR_NO_MORE_FILES => None;
        }
    }
}
