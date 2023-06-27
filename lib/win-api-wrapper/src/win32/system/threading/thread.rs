use super::super::kernel::PROCESSOR_NUMBER;
use super::MEMORY_PRIORITY_INFORMATION;
use crate::core::Result;
use crate::win32::foundation::STILL_ACTIVE;
use crate::{call, call_BOOL, default_sized, free, to_BOOL};
use core::ffi::c_void;
use core::mem::{size_of, zeroed};
use core::ptr;
use core::ptr::{addr_of, addr_of_mut};
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::System::Threading::{
    ExitThread, GetCurrentThread, GetCurrentThreadId, GetExitCodeThread, GetProcessIdOfThread,
    GetThreadDescription, GetThreadIOPendingFlag, GetThreadIdealProcessorEx, GetThreadInformation,
    GetThreadPriority, GetThreadPriorityBoost, GetThreadSelectedCpuSets, OpenThread, ResumeThread,
    SetThreadAffinityMask, SetThreadDescription, SetThreadIdealProcessorEx, SetThreadInformation,
    SetThreadPriority, SetThreadPriorityBoost, SetThreadSelectedCpuSets, SuspendThread,
    SwitchToThread, TerminateThread, ThreadAbsoluteCpuPriority, ThreadDynamicCodePolicy,
    ThreadMemoryPriority, ThreadPowerThrottling, THREAD_INFORMATION_CLASS,
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

#[doc(hidden)]
mod private {
    use super::Information;

    /// A marker trait for sealing traits.
    pub trait Sealed {}

    /// A marker trait for [thread information][super::Information] types that can
    /// be set by [`set_information`][super::set_information].
    pub trait Settable: Information {}

    /// A marker trait for [thread information][super::Information] types that can
    /// be retrieved by [`get_information`][super::get_information].
    pub trait Gettable: Information {}
}

/// Specific information that the system contains about a thread.
///
/// This trait is sealed, therefore it cannot be implemented for other types
/// outside this crate.
pub trait Information: Sized {
    /// Returns the associated [`THREAD_INFORMATION_CLASS`] of the type.
    fn information_class() -> THREAD_INFORMATION_CLASS;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
/// Represents the absolute cpu priority of a thread.
///
/// To get or set the cpu priority of a thread use [`get_information`]
/// or [`set_information`].
pub struct CpuPriority(i32);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
/// The dynamic code policy of the process. When turned on, the process
/// cannot generate dynamic code or modify existing executable code.
///
/// Note that this information is supported in Windows Server 2016 and newer, Windows 10 LTSB 2016
/// and newer, and Windows 10 version 1607 and newer.
///
/// To get or set The dynamic code policy of a thread use [`get_information`]
/// or [`set_information`].
pub struct DynamicCodePolicy(u32);

impl private::Sealed for CpuPriority {}
impl private::Sealed for DynamicCodePolicy {}
impl private::Sealed for THREAD_POWER_THROTTLING_STATE {}
impl private::Sealed for MEMORY_PRIORITY_INFORMATION {}

impl private::Gettable for CpuPriority {}
impl private::Gettable for DynamicCodePolicy {}
impl private::Gettable for MEMORY_PRIORITY_INFORMATION {}

impl private::Settable for THREAD_POWER_THROTTLING_STATE {}
impl private::Settable for MEMORY_PRIORITY_INFORMATION {}

impl Information for CpuPriority {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadAbsoluteCpuPriority
    }
}

impl Information for DynamicCodePolicy {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadDynamicCodePolicy
    }
}

impl Information for THREAD_POWER_THROTTLING_STATE {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadPowerThrottling
    }
}

impl Information for MEMORY_PRIORITY_INFORMATION {
    fn information_class() -> THREAD_INFORMATION_CLASS {
        ThreadMemoryPriority
    }
}

/// Opens an existing thread object.
///
/// # Arguments
///
/// * `id`: The thread identifier
/// * `access`: One or more [access rights][`ThreadAccessRights`] to the thread object
/// * `inherit_handle`: Specifies whether processes created by this thread should inherit its handle
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
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread
///
pub fn open(id: u32, access_rights: ThreadAccessRights, inherit_handle: bool) -> Result<isize> {
    call! { OpenThread(access_rights, to_BOOL!(inherit_handle), id) != 0 }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Ends the calling thread.
///
/// For more information, see the official [documentation].
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
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread
///
pub fn get_current_handle() -> isize {
    unsafe { GetCurrentThread() }
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets the thread identifier of the calling thread.
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessidofthread
///
pub fn get_process_id(handle: isize) -> Result<u32> {
    call! { GetProcessIdOfThread(handle) != 0 }
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
/// * `handle` does not have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreaddescription
///
pub fn get_description(handle: isize) -> Result<U16CString> {
    let mut description_ptr = ptr::null_mut::<u16>();
    call! {
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
/// * `handle` does not have [`THREAD_QUERY_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadiopendingflag
///
pub fn is_io_pending(handle: isize) -> Result<bool> {
    call_BOOL! {
        GetThreadIOPendingFlag(handle, &mut is_io_pending) -> mut is_io_pending
    }
}

/// Gets [information][get_information#retrievable-thread-information]
/// about the thread specified by `handle`.
///
/// # Retrievable thread information
///
/// Check the sections below to see what kind of information can be
/// retreived by calling this function.
///
/// ## Cpu priority
///
/// If the specified information type is [`CpuPriority`],
/// the the function returns the absolute priority of the
/// cpu that the thread is running on.
///
/// ## Dynamic code policy
///
/// If the specified information type is [`CpuPriority`],
/// the the function returns a value indicating whether
/// the specified thread is allowed to generate dynamic
/// code or modify executable code.
///
/// This type of information is supported in Windows Server 2016
/// and newer, Windows 10 LTSB 2016 and newer, and Windows 10
/// version 1607 and newer.
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
///     * See [Priorities][get_information#priorities] for possible
///       values.
///
/// ### Priorities
///
/// | Value | Description |
/// |-------|-------------|
/// | [`MEMORY_PRIORITY_VERY_LOW`][super::MEMORY_PRIORITY_VERY_LOW] | Very low memory priority. |
/// | [`MEMORY_PRIORITY_LOW`][super::MEMORY_PRIORITY_LOW] | Low memory priority. |
/// | [`MEMORY_PRIORITY_MEDIUM`][super::MEMORY_PRIORITY_MEDIUM] | Medium memory priority. |
/// | [`MEMORY_PRIORITY_BELOW_NORMAL`][super::MEMORY_PRIORITY_BELOW_NORMAL] | Below normal memory priority. |
/// | [`MEMORY_PRIORITY_NORMAL`][super::MEMORY_PRIORITY_NORMAL] | Normal memory priority. This is the default priority for all threads and processes on the system. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have [`THREAD_QUERY_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadinformation
///
pub fn get_information<T>(handle: isize) -> Result<T>
where
    T: Information + private::Gettable,
{
    call_BOOL! {
        GetThreadInformation(
            handle,
            T::information_class(),
            addr_of_mut!(information).cast::<c_void>(),
            size_of::<T>() as u32,
        ) -> mut information = default_sized!(T)
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
/// * `handle` does not have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority
///
pub fn get_priority(handle: isize) -> Result<i32> {
    call! { GetThreadPriority(handle) != 0 }
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
/// * `handle` does not have [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_SUSPEND_RESUME`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread
///
pub fn resume(handle: isize) -> Result<u32> {
    call! { ResumeThread(handle) != u32::MAX }
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
/// * `handle` does not have [`THREAD_SUSPEND_RESUME`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread
///
pub fn suspend(handle: isize) -> Result<u32> {
    call! { SuspendThread(handle) != u32::MAX }
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
/// * `handle` does not have [`THREAD_SET_INFORMATION`] or [`THREAD_SET_LIMITED_INFORMATION`] and [`THREAD_QUERY_INFORMATION`] or [`THREAD_QUERY_LIMITED_INFORMATION`] access rights.
/// * [`ERROR_INVALID_PARAMETER`][windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER]
///     * `affinity_mask` requests a processor that is not selected for the process affinity mask.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadaffinitymask
///
pub fn set_affinity_mask(handle: isize, affinity_mask: usize) -> Result<usize> {
    call! { SetThreadAffinityMask(handle, affinity_mask) != 0 }
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
/// * `handle` does not have [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_SET_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_QUERY_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_SET_INFORMATION`] or [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_SET_INFORMATION`] or [`THREAD_SET_LIMITED_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriorityboost
///
pub fn set_priority_boost(handle: isize, enable: bool) -> Result<()> {
    call_BOOL! { SetThreadPriorityBoost(handle, to_BOOL!(!enable)) }
}

/// Sets [information][set_information#modifiable-process-information]
/// for the thread specified by `handle`.
///
/// # Modifiable thread information
///
/// Check the sections below to see what kind of information can be
/// set for a thread by calling this function.
///
/// ## Memory priority
///
/// If the specified information type is [`MEMORY_PRIORITY_INFORMATION`],
/// the function sets the memory priority for the thread indentified by
/// `handle`. See the [Memory priority#get_information] section of
/// [`get_information`] for more information about this information type.
///
/// To help improve system performance, applications should use this function
/// with this information to lower the memory priority of threads that perform
/// background operations or access files and data that are not expected to be
/// accessed again soon. For example, an anti-malware application might lower
/// the priority of threads involved in scanning files.
///
/// Memory priority helps to determine how long pages remain in the working
/// set of a process before they are trimmed. A thread's memory priority
/// determines the minimum priority of the physical pages that are added
/// to the process working set by that thread. When the memory manager
/// trims the working set, it trims lower priority pages before higher
/// priority pages. This improves overall system performance because
/// higher priority pages are less likely to be trimmed from the working
/// set and then trigger a page fault when they are accessed again.
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
///         * [`THREAD_POWER_THROTTLING_CURRENT_VERSION`]: The current version.
/// * `ControlMask`: Enables the caller to take control of the power throttling
///                  mechanism.
///     * This parameter specifies what to control.
///     * See [Policy masks][set_information#policy-masks] for possible values.
/// * `StateMask`: Manages the power throttling mechanism on/off state.
///     * See [Policy masks][set_information#policy-masks] for possible values.
///     * If `ControlMask` is not `0` and this parameter is
///         * 0, the function disables the policy specified by `ControlMask`
///           for the tjread.
///         * the same as `ControlMask`, the function enables the policy
///           specified by `ControlMask` for the tjread.
///
/// ### Policy masks
///
/// | Mask | Meaning |
/// |------|---------|
/// | [`THREAD_POWER_THROTTLING_EXECUTION_SPEED`] | Manages the execution speed of the thread. |
///
/// ### Default system managed behavior
///
/// If both mask fields are `0`, the function resets the deault system managed behaviour.
///
/// ### Controlling [Quality of Service] (QoS)
///
/// If both masks are [`THREAD_POWER_THROTTLING_EXECUTION_SPEED`], the function turns
/// on throttling policies for the thread, which means that the thread will be classified
/// as `EcoQoS`. The system will try to increase power efficiency through strategies
/// such as reducing CPU frequency or using more power efficient cores. `EcoQoS` should
/// be used when the work is not contributing to the foreground user experience, which
/// provides longer battery life, and reduced heat and fan noise. `EcoQoS` should not
/// be used for performance critical or foreground user experiences. Prior to Windows 11,
/// the `EcoQoS` level did not exist and the thread was labeled as `LowQoS`.
///
/// If `ControlMask` is [`THREAD_POWER_THROTTLING_EXECUTION_SPEED`] and `StateMask` is
/// `0`, the function turns off throttling policies for the thread, which means that
/// the system will use its own heuristics to automatically infer a `QoS` level.
/// For more information, see [Quality of Service].
///
/// ### [Efficiency mode]
///
/// In Windows 11 a thread can be considered efficient if it fulfills the following
/// efficiency requirements (similarly to a process):
/// * The thread opted in for enabling [`THREAD_POWER_THROTTLING_EXECUTION_SPEED`],
///   therefore its `QoS` level is `Eco`.
/// * The priority of the thread is [`THREAD_PRIORITY_IDLE`].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have [`THREAD_SET_INFORMATION`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadinformation
/// [Quality of Service]: https://learn.microsoft.com/en-us/windows/win32/procthread/quality-of-service
///
pub fn set_information<T>(handle: isize, information: T) -> Result<()>
where
    T: Information + private::Settable,
{
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
/// For more information, see the official [documentation].
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
/// * `handle` does not have [`THREAD_TERMINATE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminatethread
///
pub fn terminate(handle: isize, exit_code: u32) -> Result<()> {
    call_BOOL! { TerminateThread(handle, exit_code) }
}
