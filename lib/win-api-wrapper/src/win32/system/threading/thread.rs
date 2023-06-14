use super::super::kernel::PROCESSOR_NUMBER;
use crate::core::Result;
use crate::{call_BOOL, call_num, free, to_BOOL};
use core::ffi::c_void;
use core::mem::{size_of, zeroed};
use core::ptr;
use core::ptr::{addr_of, addr_of_mut};
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::STILL_ACTIVE;
use windows_sys::Win32::System::Threading::{
    ExitThread, GetCurrentThread, GetCurrentThreadId, GetExitCodeThread, GetProcessIdOfThread,
    GetThreadDescription, GetThreadIOPendingFlag, GetThreadIdealProcessorEx, GetThreadInformation,
    GetThreadPriority, GetThreadPriorityBoost, GetThreadSelectedCpuSets, OpenThread, ResumeThread,
    SetThreadAffinityMask, SetThreadDescription, SetThreadIdealProcessorEx, SetThreadInformation,
    SetThreadPriority, SetThreadPriorityBoost, SetThreadSelectedCpuSets, SuspendThread,
    SwitchToThread, TerminateThread, ThreadAbsoluteCpuPriority, ThreadDynamicCodePolicy,
    ThreadMemoryPriority, ThreadPowerThrottling, MEMORY_PRIORITY_INFORMATION,
    THREAD_INFORMATION_CLASS,
};

pub use windows_sys::Win32::System::Threading::{
    THREAD_ACCESS_RIGHTS as ThreadAccessRights, THREAD_ALL_ACCESS, THREAD_DELETE,
    THREAD_DIRECT_IMPERSONATION, THREAD_GET_CONTEXT, THREAD_IMPERSONATE,
    THREAD_MODE_BACKGROUND_BEGIN, THREAD_MODE_BACKGROUND_END,
    THREAD_POWER_THROTTLING_CURRENT_VERSION, THREAD_POWER_THROTTLING_EXECUTION_SPEED,
    THREAD_POWER_THROTTLING_STATE, THREAD_POWER_THROTTLING_VALID_FLAGS, THREAD_PRIORITY,
    THREAD_PRIORITY_ABOVE_NORMAL, THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_HIGHEST,
    THREAD_PRIORITY_IDLE, THREAD_PRIORITY_LOWEST, THREAD_PRIORITY_NORMAL,
    THREAD_PRIORITY_TIME_CRITICAL, THREAD_QUERY_INFORMATION, THREAD_QUERY_LIMITED_INFORMATION,
    THREAD_READ_CONTROL, THREAD_RESUME, THREAD_SET_CONTEXT, THREAD_SET_INFORMATION,
    THREAD_SET_LIMITED_INFORMATION, THREAD_SET_THREAD_TOKEN, THREAD_STANDARD_RIGHTS_REQUIRED,
    THREAD_SUSPEND_RESUME, THREAD_SYNCHRONIZE, THREAD_TERMINATE, THREAD_WRITE_DAC,
    THREAD_WRITE_OWNER,
};

/// Opens an existing thread object.
///
/// # Arguments
///
/// * `id`: The thread identifier
/// * `access`: One or more [access rights][`ThreadAccessRights`] to the thread object
/// * `inherit_handle`: Specifies whether processes created by this thread should inherit it's handle
///
/// # Result
///
/// An open handle to the specified thread.
///
/// # Remarks
///
/// * If the handle is not needed anymore close it using [`close_handle`](crate::win32::foundation::close_handle).
///  
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible error
///
/// * `id` is not a valid thread identifier.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread
///
pub fn open(id: u32, access_rights: ThreadAccessRights, inherit_handle: bool) -> Result<isize> {
    call_num! { OpenThread(access_rights, to_BOOL!(inherit_handle), id) != 0 }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Ends the calling thread.
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread    
///
pub fn exit_current(exit_code: u32) {
    unsafe { ExitThread(exit_code) }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets a pseudo handle for the calling thread.
///
/// # Remarks
///
/// * A pseudo handle is a special constant that is interpreted as the current thread handle.
/// * The calling thread can use this handle to specify itself whenever a thread handle is required.
/// * Pseudo handles are not inherited by child processes.
/// * This handle has the [`THREAD_ALL_ACCESS`] access right to the thread object.
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread
///
pub fn get_current_handle() -> isize {
    unsafe { GetCurrentThread() }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets the thread identifier of the calling thread.
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid
///
pub fn get_current_id() -> u32 {
    unsafe { GetCurrentThreadId() }
}

/// Determines whether the specified thread is running.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread
///
pub fn is_running(handle: isize) -> Result<bool> {
    let mut exit_code = 0;
    call_BOOL!(GetExitCodeThread(handle, &mut exit_code) return Error);
    Ok(exit_code == STILL_ACTIVE as u32)
}

/// Gets the exit code of the specified thread if it has exitied. If the thread is still running the Result is [`None`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread
///
pub fn get_exit_code(handle: isize) -> Result<Option<u32>> {
    let mut exit_code = 0;
    call_BOOL!(GetExitCodeThread(handle, &mut exit_code) return Error);
    if exit_code != STILL_ACTIVE as u32 {
        Ok(Some(exit_code))
    } else {
        Ok(None)
    }
}

/// Gets the process identifier of the process associated with the specified thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessidofthread
///
pub fn get_process_id(handle: isize) -> Result<u32> {
    call_num! { GetProcessIdOfThread(handle) != 0 }
}

/// Gets the description that was assigned to a thread by [`set_description`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreaddescription
///
pub fn get_description(handle: isize) -> Result<U16CString> {
    let mut description_ptr = ptr::null_mut::<u16>();
    call_num! {
        GetThreadDescription(
            handle,
            addr_of_mut!(description_ptr).cast()
        ) == 0 => return Error
    };
    // Safety: `description_ptr` is a valid nul-terminated string pointer.
    let description = unsafe { U16CString::from_ptr_str(description_ptr) };
    free!(Local: description_ptr);
    Ok(description)
}

/// Determines whether a specified thread has any I/O requests pending.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadiopendingflag
///
pub fn is_io_pending(handle: isize) -> Result<bool> {
    call_BOOL! {
        GetThreadIOPendingFlag(handle, &mut is_io_pending) -> mut is_io_pending
    }
}

/// Specific information that the system contains about a thread.
pub trait ThreadInformation: Sized {
    /// Returns the associated [`THREAD_INFORMATION_CLASS`] of the type.
    fn information_class() -> THREAD_INFORMATION_CLASS;

    fn default_information() -> Self {
        // Safety: type is not a reference or prointer.
        unsafe { zeroed() }
    }
}

impl ThreadInformation for i32 {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadAbsoluteCpuPriority
    }
}

impl ThreadInformation for u32 {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadDynamicCodePolicy
    }
}

impl ThreadInformation for THREAD_POWER_THROTTLING_STATE {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadPowerThrottling
    }
}

impl ThreadInformation for MEMORY_PRIORITY_INFORMATION {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadMemoryPriority
    }
}

/// Gets information about the specified thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadinformation
///
pub fn get_information<T: Copy + ThreadInformation>(handle: isize) -> Result<T> {
    call_BOOL! {
        GetThreadInformation(
            handle,
            T::information_class(),
            addr_of_mut!(information).cast::<c_void>(),
            size_of::<T>() as u32,
        ) -> mut information = T::default_information()
    }
}

/// Gets the priority value for the specified thread.
///
/// # Remarks
/// * This value, together with the priority class of the thread's process, determines the thread's base-priority level.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority
///
pub fn get_priority(handle: isize) -> Result<i32> {
    call_num! { GetThreadPriority(handle) != 0 }
}

/// Determines whether the specified thread has dynamic boosting enabled.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriorityboost
///
pub fn has_priority_boost(handle: isize) -> Result<bool> {
    call_BOOL! {
        GetThreadPriorityBoost(handle, &mut is_disabled) -> mut !is_disabled
    }
}

/// Gets the required capacity of to hold the entire list of thread selected CPU Sets.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadselectedcpusets
///
pub fn get_selected_cpu_set_count(handle: isize) -> Result<u32> {
    call_BOOL! { GetThreadSelectedCpuSets(handle, ptr::null_mut(), 0, &mut count) -> mut count: u32 }
}

/// Gets the explicit CPU Set assignment of the specified thread, if any assignment was set using [`set_selected_cpu_sets`].
///
/// # Remarks
/// * If no explicit assignment is set, the Result is any empty boxed array slice.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadselectedcpusets
///
pub fn get_selected_cpu_sets(handle: isize) -> Result<Vec<u32>> {
    let mut count = 0;
    call_BOOL! { GetThreadSelectedCpuSets(handle, ptr::null_mut(), count, &mut count) return Error};
    if count == 0 {
        return Ok(Vec::new());
    }

    call_BOOL! {
        GetThreadSelectedCpuSets(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            &mut count,
        ) -> mut buffer = vec![0; count as usize]
    }
}

/// Decrements a thread's suspend count. When the suspend count is decremented to `0`, the execution of the thread is resumed.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SUSPEND_RESUME`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
///
pub fn resume(handle: isize) -> Result<u32> {
    call_num! { ResumeThread(handle) != u32::MAX }
}

/// Suspends the specified thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SUSPEND_RESUME`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread
///
pub fn suspend(handle: isize) -> Result<u32> {
    call_num! { SuspendThread(handle) != u32::MAX }
}

/// Sets a processor affinity mask for the specified thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SET_INFORMATION`] or [`THREAD_SET_LIMITED_INFORMATION`] and [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access rights.
/// * [`ERROR_INVALID_PARAMETER`][windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER]
///     * `affinity_mask` requests a processor that is not selected for the process affinity mask.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadaffinitymask
///
pub fn set_affinity_mask(handle: isize, affinity_mask: usize) -> Result<usize> {
    call_num! { SetThreadAffinityMask(handle, affinity_mask) != 0 }
}

/// Sets description to a thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreaddescription
///
pub fn set_description(handle: isize, description: &U16CStr) -> Result<()> {
    call_BOOL! { SetThreadDescription(handle, description.as_ptr()) }
}

/// Gets the processor number of the ideal processor for the specified thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex
///
pub fn get_ideal_processor(handle: isize) -> Result<PROCESSOR_NUMBER> {
    call_BOOL! {
        GetThreadIdealProcessorEx(
            handle,
            &mut ideal_proc,
        ) -> mut ideal_proc =
            // Safety: `PROCESS_NUMBER` is not a reference nor a pointer.
            unsafe { zeroed::<PROCESSOR_NUMBER>() }
    }
}

/// Sets the ideal processor for the specified thread and returns the previous ideal processor.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SET_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex
///
pub fn set_ideal_processor(
    handle: isize,
    ideal_processor: PROCESSOR_NUMBER,
) -> Result<PROCESSOR_NUMBER> {
    call_BOOL! {
        SetThreadIdealProcessorEx(
            handle,
            &ideal_processor,
            &mut prev_ideal_proc,
        ) -> mut prev_ideal_proc =
            // Safety: `PROCESS_NUMBER` is not a reference nor a pointer.
            unsafe { zeroed::<PROCESSOR_NUMBER>() }
    }
}

/// Clears the explicit CPU Set assignment of the specified thread, if any assignment was set using [`set_selected_cpu_sets`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets
///
pub fn clear_selected_cpu_sets(handle: isize) -> Result<()> {
    call_BOOL! { SetThreadSelectedCpuSets(handle, ptr::null_mut(), 0) }
}

/// Sets the selected CPU Sets assignment for the specified thread. This assignment overrides the process default assignment, if one is set.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets
///
pub fn set_selected_cpu_sets(handle: isize, cpu_sets: &[u32]) -> Result<()> {
    call_BOOL! { SetThreadSelectedCpuSets(handle, cpu_sets.as_ptr(), cpu_sets.len() as u32) }
}

/// Sets the priority value for the specified thread. This value, together with the priority class of the thread's process,
/// determines the thread's base priority level.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SET_INFORMATION`] or [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority
///
pub fn set_priority(handle: isize, priority: i32) -> Result<()> {
    call_BOOL! { SetThreadPriority(handle, priority) }
}

/// Enables or disables the ability of the system to temporarily boost the priority of a thread based on the value of `enable`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SET_INFORMATION`] or [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriorityboost
///
pub fn set_priority_boost(handle: isize, enable: bool) -> Result<()> {
    call_BOOL! { SetThreadPriorityBoost(handle, to_BOOL!(!enable)) }
}

/// Sets information for the specified thread.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_SET_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadinformation
///
pub fn set_information<T: Copy + ThreadInformation>(handle: isize, information: T) -> Result<()> {
    call_BOOL! {
        SetThreadInformation(
            handle,
            T::information_class(),
            addr_of!(information).cast::<c_void>(),
            size_of::<T>() as u32,
        )
    }
}

/// Causes the calling thread to yield execution to another thread that is ready to run on the current processor.
/// The operating system selects the next thread to be executed.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * No other threads are ready to execute.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-switchtothread
///
pub fn switch_to_another() -> Result<()> {
    call_BOOL! { SwitchToThread() }
}

/// Terminates the specified thread with `exit_code`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` doesn't have [`THREAD_TERMINATE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminatethread
///
pub fn terminate(handle: isize, exit_code: u32) -> Result<()> {
    call_BOOL! { TerminateThread(handle, exit_code) }
}
