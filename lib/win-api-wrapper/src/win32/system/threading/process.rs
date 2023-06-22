use crate::core::{Result, To};
use crate::{call, call_BOOL, default_sized};
use crate::{from_BOOL, to_BOOL};
use core::ffi::c_void;
use core::mem::{size_of, transmute, zeroed};
use core::ptr::{self, addr_of, addr_of_mut};
use widestring::U16CString;
use windows_sys::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, FILETIME, HANDLE, STILL_ACTIVE};
use windows_sys::Win32::System::Threading::{
    ExitProcess, GetCurrentProcess, GetCurrentProcessId, GetExitCodeProcess, GetPriorityClass,
    GetProcessAffinityMask, GetProcessDefaultCpuSets, GetProcessGroupAffinity,
    GetProcessHandleCount, GetProcessId, GetProcessInformation, GetProcessIoCounters,
    GetProcessPriorityBoost, GetProcessTimes, GetProcessVersion, GetProcessWorkingSetSize,
    IsProcessCritical, OpenProcess, OpenProcessToken, ProcessAppMemoryInfo, ProcessLeapSecondInfo,
    ProcessMemoryPriority, ProcessPowerThrottling, ProcessProtectionLevelInfo,
    QueryFullProcessImageNameW, SetPriorityClass, SetProcessAffinityMask,
    SetProcessAffinityUpdateMode, SetProcessDefaultCpuSets, SetProcessInformation,
    SetProcessPriorityBoost, SetProcessWorkingSetSize, TerminateProcess, WaitForInputIdle,
    PROCESS_AFFINITY_DISABLE_AUTO_UPDATE, PROCESS_AFFINITY_ENABLE_AUTO_UPDATE,
};

pub use windows_sys::Win32::System::Threading::{
    ABOVE_NORMAL_PRIORITY_CLASS, APP_MEMORY_INFORMATION, BELOW_NORMAL_PRIORITY_CLASS,
    HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, IO_COUNTERS, MEMORY_PRIORITY,
    MEMORY_PRIORITY_BELOW_NORMAL, MEMORY_PRIORITY_INFORMATION, MEMORY_PRIORITY_LOW,
    MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_NORMAL, MEMORY_PRIORITY_VERY_LOW,
    NORMAL_PRIORITY_CLASS, PROCESS_ACCESS_RIGHTS as ProcessAccessRights, PROCESS_ALL_ACCESS,
    PROCESS_CREATE_PROCESS, PROCESS_CREATE_THREAD, PROCESS_CREATION_FLAGS, PROCESS_DELETE,
    PROCESS_DUP_HANDLE, PROCESS_INFORMATION_CLASS, PROCESS_LEAP_SECOND_INFO,
    PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND, PROCESS_LEAP_SECOND_INFO_VALID_FLAGS,
    PROCESS_MODE_BACKGROUND_BEGIN, PROCESS_MODE_BACKGROUND_END, PROCESS_NAME_FORMAT,
    PROCESS_NAME_NATIVE, PROCESS_NAME_WIN32, PROCESS_POWER_THROTTLING_CURRENT_VERSION,
    PROCESS_POWER_THROTTLING_EXECUTION_SPEED, PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION,
    PROCESS_POWER_THROTTLING_STATE, PROCESS_PROTECTION_LEVEL, PROCESS_PROTECTION_LEVEL_INFORMATION,
    PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_READ_CONTROL,
    PROCESS_SET_INFORMATION, PROCESS_SET_LIMITED_INFORMATION, PROCESS_SET_QUOTA,
    PROCESS_SET_SESSIONID, PROCESS_STANDARD_RIGHTS_REQUIRED, PROCESS_SUSPEND_RESUME,
    PROCESS_SYNCHRONIZE, PROCESS_TERMINATE, PROCESS_VM_OPERATION, PROCESS_VM_READ,
    PROCESS_VM_WRITE, PROCESS_WRITE_DAC, PROCESS_WRITE_OWNER, PROTECTION_LEVEL_ANTIMALWARE_LIGHT,
    PROTECTION_LEVEL_AUTHENTICODE, PROTECTION_LEVEL_CODEGEN_LIGHT, PROTECTION_LEVEL_LSA_LIGHT,
    PROTECTION_LEVEL_NONE, PROTECTION_LEVEL_PPL_APP, PROTECTION_LEVEL_WINDOWS,
    PROTECTION_LEVEL_WINDOWS_LIGHT, PROTECTION_LEVEL_WINTCB, PROTECTION_LEVEL_WINTCB_LIGHT,
    REALTIME_PRIORITY_CLASS, THREAD_POWER_THROTTLING_CURRENT_VERSION,
};

use crate::win32::security::{self, TOKEN_ELEVATION, TOKEN_QUERY};

#[doc(hidden)]
mod private {
    use super::Information;

    /// A marker trait for sealing traits.
    pub trait Sealed {}

    /// A marker trait for [process information][super::Information] types that can
    /// be set by [`set_information`][super::set_information].
    pub trait Settable: Information {}

    /// A marker trait for [process information][super::Information] types that can
    /// be retrieved by [`get_information`][super::get_information].
    pub trait Gettable: Information {}
}

/// Maximum number of characters allowed in a long path.
const MAX_CHARS_IN_LONG_PATH: usize = u16::MAX as usize / size_of::<u16>();

/// A member of the [`PROCESS_INFORMATION_CLASS`] enumeration.
///
/// This trait is sealed, therefore it cannot be implemented for other types
/// outside this crate.
pub trait Information: private::Sealed {
    /// Gets the [`PROCESS_INFORMATION_CLASS`], that is associated with the type.
    fn information_class() -> PROCESS_INFORMATION_CLASS;
}

impl private::Sealed for APP_MEMORY_INFORMATION {}
impl private::Sealed for MEMORY_PRIORITY_INFORMATION {}
impl private::Sealed for PROCESS_POWER_THROTTLING_STATE {}
impl private::Sealed for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl private::Sealed for PROCESS_LEAP_SECOND_INFO {}

impl private::Gettable for APP_MEMORY_INFORMATION {}
impl private::Gettable for MEMORY_PRIORITY_INFORMATION {}
impl private::Gettable for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl private::Gettable for PROCESS_LEAP_SECOND_INFO {}

impl private::Settable for MEMORY_PRIORITY_INFORMATION {}
impl private::Settable for PROCESS_LEAP_SECOND_INFO {}
impl private::Settable for PROCESS_POWER_THROTTLING_STATE {}

impl Information for MEMORY_PRIORITY_INFORMATION {
    fn information_class() -> PROCESS_INFORMATION_CLASS {
        ProcessMemoryPriority
    }
}

impl Information for APP_MEMORY_INFORMATION {
    fn information_class() -> PROCESS_INFORMATION_CLASS {
        ProcessAppMemoryInfo
    }
}

impl Information for PROCESS_POWER_THROTTLING_STATE {
    fn information_class() -> PROCESS_INFORMATION_CLASS {
        ProcessPowerThrottling
    }
}

impl Information for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn information_class() -> PROCESS_INFORMATION_CLASS {
        ProcessProtectionLevelInfo
    }
}

impl Information for PROCESS_LEAP_SECOND_INFO {
    fn information_class() -> PROCESS_INFORMATION_CLASS {
        ProcessLeapSecondInfo
    }
}

impl To<(u16, u16)> for u32 {
    #[inline]
    fn to(&self) -> (u16, u16) {
        // Safety: `u32` and `(u16, u16)` has the same size
        unsafe { transmute(*self) }
    }
}

/// Ends the calling process and all its threads.
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess
///
pub fn exit_current(exit_code: u32) {
    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        ExitProcess(exit_code)
    }
}

/// Gets a pseudo handle for the current process.
///
/// # Remarks
///
/// * The returned value is a special constant, currently `-1`, that is interpreted as the current process handle.
/// * The pseudo handle need not to be closed when it is no longer needed.
///
/// # Examples
///
/// Getting the priority class of the calling process:
/// ```
/// let current_process = get_current_handle();
/// match get_priority_class(current_process) {
///     Ok(priority_class) => println!("The current process priority class is {}", priority_class),
///     Err(error) => println!("Failed to get the current process priority class: {}", error),
/// }
/// ```
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess
///
pub fn get_current_handle() -> isize {
    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        GetCurrentProcess()
    }
}

/// Gets the process identifier of the calling process.
///
/// # Examples
///
/// Simple usage:
/// ```
/// let current_id = get_current_id();
/// println!("The current process ID is {}.", current_id);
/// ```
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid
///
pub fn get_current_id() -> u32 {
    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        GetCurrentProcessId()
    }
}

/// Opens an existing local process object.
///
/// # Arguments
///
/// * `pid`: The process identifier
/// * `access`: One or more [access rights][`ProcessAccessRights`] to the process object
/// * `inherit_handle`: Specifies whether processes created by this process should inherit it's handle
///
/// # Result
///
/// An open handle to the specified process.
///
/// # Remarks
///
/// * If the handle is not needed anymore close it using [`close_handle`](crate::win32::foundation::close_handle).
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
///
/// * The process doesn't exist.
/// * [`ERROR_INVALID_PARAMETER`][windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER]
///     * The specified process is the `System Idle Process` (`0`).
/// * [`ERROR_ACCESS_DENIED`][windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED]
///     * The specified process is the `System` process or one of the `Client Server Run-Time Subsystem` (`CSRSS`) processes.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess
///
pub fn open(pid: u32, access: ProcessAccessRights, inherit_handle: bool) -> Result<isize> {
    call! { OpenProcess(access, to_BOOL!(inherit_handle), pid) != 0 }
}

/// Determines whether the specified process is elevated.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
pub fn is_elevated(handle: isize) -> bool {
    let mut token_handle = HANDLE::default();
    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        if from_BOOL!(!OpenProcessToken(handle, TOKEN_QUERY, &mut token_handle)) {
            return false;
        }
    }

    security::get_token_information::<TOKEN_ELEVATION>(token_handle)
        .map_or(false, |info| info.TokenIsElevated > 0)
}

/// Gets the full name of the executable image for the specified process.
/// The `use_win32_path_format` parameter specifies whether to use Win32 or native system path format.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew
///
pub fn get_full_image_name(handle: isize, use_win32_path_format: bool) -> Result<U16CString> {
    let mut buffer_size = MAX_CHARS_IN_LONG_PATH as u32;
    let mut buffer = [0; MAX_CHARS_IN_LONG_PATH];
    call_BOOL! {
        QueryFullProcessImageNameW(
            handle,
            if use_win32_path_format { PROCESS_NAME_WIN32 } else { PROCESS_NAME_NATIVE },
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) return Error
    };

    // Safety: `buffer` is valid `u16` array with a length of `MAX_CHARS_IN_LONG_PATH`.
    Ok(unsafe { U16CString::from_ptr_str(buffer.as_ptr()) })
}

/// Gets the full name of the executable image for the specified process.
/// The `use_win32_path_format` parameter specifies whether to use Win32 or native system path format.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
/// * [`ERROR_INSUFFICIENT_BUFFER`][windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER]
///     * `buffer` is too small.
///
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew
///
pub fn get_full_image_name_with_buffer(
    handle: isize,
    buffer: &mut [u16],
    use_win32_path_format: bool,
) -> Result<usize> {
    let mut buffer_size = buffer.len() as u32;
    call_BOOL! {
        QueryFullProcessImageNameW(
            handle,
            if use_win32_path_format { PROCESS_NAME_WIN32 } else { PROCESS_NAME_NATIVE },
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) return Error
    };
    Ok(buffer_size as usize)
}

/// Gets the process affinity mask for the specified process and the system affinity mask for the system as `(usize, usize)`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask
///
pub fn get_affinity_mask(handle: isize) -> Result<(usize, usize)> {
    call_BOOL! {
        GetProcessAffinityMask(
            handle,
            &mut process_mask,
            &mut system_mask,
        ) -> mut (process_mask, system_mask): (usize, usize)
    }
}

/// Gets the list of CPU Sets in the process default set that was set by [`set_default_cpu_sets`].
/// If the process has no default CPU Sets the Result is [`None`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets
///
pub fn get_default_cpu_sets(handle: isize) -> Result<Option<Vec<u32>>> {
    let mut count = 0;
    call_BOOL! { GetProcessDefaultCpuSets(handle, ptr::null_mut(), count, &mut count) return Error };
    if count == 0 {
        return Ok(None);
    }

    let mut buffer = vec![0_u32; count as usize];
    call_BOOL! {
        GetProcessDefaultCpuSets(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            &mut count,
        ) return Error
    };
    Ok(Some(buffer))
}

/// Determines whether the specified process is running.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess
///
pub fn is_running(handle: isize) -> Result<bool> {
    let mut exit_code = 0;
    call_BOOL!(GetExitCodeProcess(handle, &mut exit_code) return Error);
    Ok(exit_code == STILL_ACTIVE as u32)
}

/// Gets the exit code of the specified process if it has exitied. If the process is still running the Result is [`None`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess
///
pub fn get_exit_code(handle: isize) -> Result<Option<u32>> {
    let mut exit_code = 0;
    call_BOOL! { GetExitCodeProcess(handle, &mut exit_code) return Error };
    if exit_code != STILL_ACTIVE as u32 {
        Ok(Some(exit_code))
    } else {
        Ok(None)
    }
}

/// Gets the processor group affinity of the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
/// * 'buffer' is too small.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processtopologyapi/nf-processtopologyapi-getprocessgroupaffinity
///
pub fn get_group_affinity(handle: isize) -> Result<Vec<u16>> {
    let mut count = 0;
    call_BOOL! {
        GetProcessGroupAffinity(
            handle,
            &mut count,
            ptr::null_mut(),
        ) ->
            if ERROR_INSUFFICIENT_BUFFER return;
            else return [].into();
    };
    call_BOOL! { GetProcessGroupAffinity(
        handle,
        &mut count,
        buffer.as_mut_ptr()) -> mut buffer = vec![0; count as usize]
    }
}

/// Returns the number of processor group numbers put into `buffer` array, that are part of the proccessor group affinity of the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
/// * [`ERROR_INSUFFICIENT_BUFFER`][windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER]
///     * `buffer` is too small.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processtopologyapi/nf-processtopologyapi-getprocessgroupaffinity
///
pub fn get_group_affinity_with_buffer(handle: isize, buffer: &mut [u16]) -> Result<u16> {
    call_BOOL! {
        GetProcessGroupAffinity(
            handle,
            &mut count,
            buffer.as_mut_ptr(),
        ) -> mut count = buffer.len() as u16
    }
}

/// Gets the number of open handles that belong to the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesshandlecount
///
pub fn get_handle_count(handle: isize) -> Result<u32> {
    call_BOOL! { GetProcessHandleCount(handle, &mut count) -> mut count: u32 }
}

/// Determines whether the specified process is considered critical.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocesscritical
///
pub fn is_critical(handle: isize) -> Result<bool> {
    call_BOOL! { IsProcessCritical(handle, &mut is_critical) -> mut is_critical }
}

/// Gets the process identifier of the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid
///
pub fn get_id(handle: isize) -> Result<u32> {
    call! { GetProcessId(handle) != 0 }
}

/// Gets the information of all I/O operations performed by the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessiocounters
///
pub fn get_io_counters(handle: isize) -> Result<IO_COUNTERS> {
    call_BOOL! { GetProcessIoCounters(handle, &mut counters) -> mut counters: IO_COUNTERS }
}

/// Returns whether the specified proces has priority boost enabled.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesspriorityboost
///
pub fn has_priority_boost(handle: isize) -> Result<bool> {
    call_BOOL! { GetProcessPriorityBoost(handle, &mut disabled) -> mut !disabled }
}

/// Gets the priority class for the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass
///
pub fn get_priority_class(handle: isize) -> Result<PROCESS_CREATION_FLAGS> {
    call! { GetPriorityClass(handle) != 0 => PROCESS_CREATION_FLAGS }
}

/// Gets the timing information for the specified process.
///
/// # Result
///
/// The [`FILETIME`] array contains the following `4` timing values in order:
///
/// * The creation time of the process
/// * The exit time of the process
/// * The amount of time that the process has executed in kernel mode
/// * The amount of time that the process has executed in user mode
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes
///
pub fn get_times(handle: isize) -> Result<[FILETIME; 4]> {
    call_BOOL! {
        GetProcessTimes(
            handle,
            &mut times[0],
            &mut times[1],
            &mut times[2],
            &mut times[3],
        ) -> mut times =
                // Safety: `FILETIME` is not a reference nor a pointer.
                [unsafe { zeroed::<FILETIME>() }; 4]
    }
}

/// Gets the major and minor version numbers of the system on which the specified process expects to run as a tuple.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `pid` is invalid.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessversion
///
pub fn get_version(pid: u32) -> Result<(u16, u16)> {
    call! { GetProcessVersion(pid) != 0 => To }
}

/// Gets the minimum and maximum working set sizes of the specified process in a tuple.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_QUERY_INFORMATION`] or [`PROCESS_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getprocessworkingsetsize
///
pub fn get_working_set_size(handle: isize) -> Result<(usize, usize)> {
    call_BOOL! {
        GetProcessWorkingSetSize(
            handle,
            &mut min_size,
            &mut max_size,
        ) -> mut (min_size, max_size): (usize, usize)
    }
}

/// Terminates the specified process and all of its threads with the given exit code.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_TERMINATE`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess
///
pub fn terminate(handle: isize, exit_code: u32) -> Result<()> {
    call_BOOL! { TerminateProcess(handle, exit_code) }
}

/// Gets [information][get_information#retrievable-process-information] about
/// the specified process.
///
/// # Retrievable process information
///
/// Check the sections below to see what kind of information can be
/// retreived by calling this function.
///
/// ## Application memory usage
///
/// If the specified information type is [`APP_MEMORY_INFORMATION`],
/// the the function returns the application's memory usage at time
/// of calling.
///
/// ### Fields of [`APP_MEMORY_INFORMATION`]
///
/// * `AvailableCommit`: [`u64`] - Total commit available to the app.
/// * `PrivateCommitUsage`: [`u64`] - The app's usage of private commit.
/// * `PeakPrivateCommitUsage`: [`u64`] - The app's peak usage of private commit.
/// * `TotalCommitUsage`: [`u64`] - The app's total usage of private plus shared commit.
///
/// ## Memory priority
///
/// If the specified information type is [`MEMORY_PRIORITY_INFORMATION`],
/// the function returns the memory priority of the process identified
/// by `handle`
///
/// ### Fields of [`MEMORY_PRIORITY_INFORMATION`]
///
/// `MemoryPriority`: [`u32`] - The memory priority for the process.
///     * See [Priority values#get_information] for possible
///       values.
///
/// ### Priority values
///
/// | Value | Description |
/// |-------|-------------|
/// | [`MEMORY_PRIORITY_VERY_LOW`] | Very low memory priority. |
/// | [`MEMORY_PRIORITY_LOW`] | Low memory priority. |
/// | [`MEMORY_PRIORITY_MEDIUM`] | Medium memory priority. |
/// | [`MEMORY_PRIORITY_BELOW_NORMAL`] | Below normal memory priority. |
/// | [`MEMORY_PRIORITY_NORMAL`] | Normal memory priority. This is the default priority for all threads and processes on the system. |
///
/// ## Protection level
///
/// If the specified information type is [`PROCESS_PROTECTION_LEVEL_INFORMATION`],
/// the function returns protection level information about the process.
///
/// ### Fields of [`PROCESS_PROTECTION_LEVEL_INFORMATION`]
///
/// * `ProtectionLevel`: [`u32`] - Specifies whether Protected Process Light (PPL) is enabled.
///     * See [get_information#protection-levels] for more information
///       about the possible values.
///
/// ### Protection levels
///
/// | Value | Meaning |
/// |-------|---------|
/// | [`PROTECTION_LEVEL_PPL_APP`] | The process is a third party app that is using process protection. |
/// | [`PROTECTION_LEVEL_NONE`] | The process is not protected. |
///
/// ## Leap seconds
///
/// If the specified information type is [`PROCESS_LEAP_SECOND_INFO`]
/// the function returns how the system handles positive leap seconds.
///
/// ## Fields of [`PROCESS_LEAP_SECOND_INFO`]
///
/// * `Flags`: [`u32`] - Currently, the only valid flag is [`PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND`].
///     * This parameter currently indicates whether the sixty seconds is enabled.
/// * `Reserved`: [`u32`] - Reserved for future use.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation
///
pub fn get_information<T>(handle: isize) -> Result<T>
where
    T: Information + private::Gettable,
{
    call_BOOL! {
        GetProcessInformation(
            handle,
            T::information_class(),
            addr_of_mut!(information).cast::<c_void>(),
            size_of::<T>() as u32,
        ) -> mut information = default_sized!(T)
    }
}

/// Sets information for the specified process.
///
/// # Modifiable process information
///
/// Check the sections below to see what kind of information can be
/// set for a process by calling this function.
///
/// ## Memory priority
///
/// If the specified information type is [`MEMORY_PRIORITY_INFORMATION`],
/// the function sets the memory priority for the process indentified by
/// `handle`. See [`get_information`]'s [Memory priority#get_information]
/// section for more information about the information type.
///   
/// ## Power throttling state
///
/// If the specified information type is [`PROCESS_POWER_THROTTLING_STATE`],
/// the function  enables throttling policies on a process, which can be used
/// to balance out performance and power efficiency in cases where optimal
/// performance is not required.
///
/// ### Fields of [`PROCESS_POWER_THROTTLING_STATE`]
///
/// * `Version`: [`u64`] - The version of the structure.
///     * Possible versions:
///         * [`PROCESS_POWER_THROTTLING_CURRENT_VERSION`]: The current version.
/// * `ControlMask`: Enables the caller to take control of the power throttling
///                  mechanism.
///     * This parameter specifies what to control.
///     * See [Masks][set_information#policy-masks] for possible values.
/// * `StateMask`: Manages the power throttling mechanism on/off state.
///     * See [Masks][set_information#policy-masks] for possible values.
///     * If `ControlMask` is not `0` and this parameter is
///         * 0, the function disables the policy specified by `ControlMask`
///           for the process.
///         * the same as `ControlMask`, the function enables the policy
///           specified by `ControlMask` for the process.
///     
/// ### Policy masks
///
/// | Mask | Meaning |
/// |------|---------|
/// | [`PROCESS_POWER_THROTTLING_EXECUTION_SPEED`] | Manages the execution speed of the process. |
/// | [`PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION`] | Whether to ignore resolution requests made by the process or honor. |
///
/// ### Default system managed behavior
///
/// If both mask fields are `0`, the function resets the deault system managed behaviour.
///
/// ### Controling Timer Resolution
///
/// If `ControlMask` is [`PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION`] and `StateMask` is
/// `0`, the function opts into enabling [`PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION`]
/// for the process, which means that any current timer resolution requests made by the process
/// will be ignored. Timers belonging to the process are no longer guaranteed to expire with
/// higher timer resolution, which can improve power efficiency.
///
/// If both masks are [`PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION`], opts out of using
/// [`PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION`] for the process, which means that
/// the system remembers and honors any previous timer resolution request by the process.
///
/// By default in Windows 11 if a window owning process becomes fully occluded, minimized,
/// or otherwise non-visible to the end user, and non-audible, Windows may automatically
/// ignore the timer resolution request and thus does not guarantee a higher resolution
/// than the default system resolution.
///
/// ### Controlling [Quality of Service] (QoS)
///
/// If `ControlMask` is [`PROCESS_POWER_THROTTLING_EXECUTION_SPEED`] and `StateMask` is
/// `0`, the function opts into enabling [`PROCESS_POWER_THROTTLING_EXECUTION_SPEED`]
/// for the process, which means that the process will be classified as `EcoQoS`. The
/// system will try to increase power efficiency through strategies such as reducing
/// CPU frequency or using more power efficient cores. EcoQoS should be used when the
/// work is not contributing to the foreground user experience, which provides longer
/// battery life, and reduced heat and fan noise. EcoQoS should not be used for
/// performance critical or foreground user experiences. Prior to Windows 11,
/// the `EcoQoS` level did not exist and the process was labeled as `LowQoS`.
///
/// If both masks are [`PROCESS_POWER_THROTTLING_EXECUTION_SPEED`], opts out of using
/// [`PROCESS_POWER_THROTTLING_EXECUTION_SPEED`] for the process, which means that
/// the system will use its own heuristics to automatically infer a `QoS` level.
/// For more information, see [Quality of Service].
///
/// ### [Efficiency mode]
///
/// Windows 11 introduced the [Efficiency mode] status label, that is shown enabled
/// in the Task Manager for processes that fulfill the following efficiency requirements:
/// * The process opted in for enabling [`PROCESS_POWER_THROTTLING_EXECUTION_SPEED`], therefore
///   its `QoS` level is `Eco`.
/// * The priority of the process is [`IDLE_PRIORITY_CLASS`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation
/// [Quality of Service]: https://learn.microsoft.com/en-us/windows/win32/procthread/quality-of-service
/// [`Efficiency mode`]: https://devblogs.microsoft.com/performance-diagnostics/reduce-process-interference-with-task-manager-efficiency-mode/
///
pub fn set_information<T>(handle: isize, information: T) -> Result<()>
where
    T: Information + private::Settable,
{
    call_BOOL! {
        SetProcessInformation(
            handle,
            T::information_class(),
            addr_of!(information).cast::<c_void>(),
            size_of::<T>() as u32,
        )
    }
}

/// Sets `class` as the priority class of the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass
///
pub fn set_priority_class(handle: isize, class: PROCESS_CREATION_FLAGS) -> Result<()> {
    call_BOOL! { SetPriorityClass(handle, class) }
}

/// Enables the affinity update mode of the current process if `enable_auto_update` is `true`. If `enable_auto_update` is `false`, then disables it.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessaffinityupdatemode
///
pub fn set_current_affinity_update_mode(enable_auto_update: bool) -> Result<()> {
    let flags = if enable_auto_update {
        PROCESS_AFFINITY_ENABLE_AUTO_UPDATE
    } else {
        PROCESS_AFFINITY_DISABLE_AUTO_UPDATE
    };
    #[allow(clippy::undocumented_unsafe_blocks)]
    let current_handle = unsafe { GetCurrentProcess() };
    call_BOOL! { SetProcessAffinityUpdateMode(current_handle, flags) }
}

/// Enables or disables the ability of the system to temporarily boost the priority of the threads of
/// the specified process based on the value of `enable`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocesspriorityboost
///
pub fn set_priority_boost(handle: isize, enable: bool) -> Result<()> {
    call_BOOL! { SetProcessPriorityBoost(handle, to_BOOL!(!enable)) }
}

/// Sets `mask` as the processor affinity mask for the threads of the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask
///
pub fn set_affinity_mask(handle: isize, mask: usize) -> Result<()> {
    call_BOOL! { SetProcessAffinityMask(handle, mask) }
}

/// Sets `min_size` as the minimum and `max_size` as the maximum working set size for the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_QUOTA`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-setprocessworkingsetsize
///
pub fn set_working_set_size(handle: isize, min_size: usize, max_size: usize) -> Result<()> {
    call_BOOL! { SetProcessWorkingSetSize(handle, min_size, max_size) }
}

/// Removes as many pages as possible from the working set of the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_QUOTA`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-setprocessworkingsetsize
///
pub fn shrink_working_set(handle: isize) -> Result<()> {
    call_BOOL! {
        SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX)
    }
}

/// Sets `cpu_sets` as the default CPU Set for threads in the specified specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_LIMITED_INFORMATION`] access right.
///
/// ## Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets
///
pub fn set_default_cpu_sets(handle: isize, cpu_sets: &[u32]) -> Result<()> {
    call_BOOL! { SetProcessDefaultCpuSets(handle, cpu_sets.as_ptr(), cpu_sets.len() as u32) }
}

/// Clears the default CPU Sets for the specified process.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`PROCESS_SET_LIMITED_INFORMATION`] access right.
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets
///
pub fn clear_default_cpu_sets(handle: isize) -> Result<()> {
    call_BOOL! { SetProcessDefaultCpuSets(handle, ptr::null(), 0) }
}

/// The current thread waits until the specified process has finished processing its initial input
/// and is waiting for user input with no input pending, or until `timeout_ms` interval has elapsed.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * [`WAIT_TIMEOUT`][windows_sys::Win32::Foundation::WAIT_TIMEOUT]
///     * The wait was terminated, because the time-out interval elapsed.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitforinputidle
///
pub fn wait_for_input_idle(handle: isize, timeout_ms: u32) -> Result<()> {
    call! { WaitForInputIdle(handle, timeout_ms) == 0 }
}
