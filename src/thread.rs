use std::ffi::OsStr;

use crate::{
    common::{As, ConversionError, MemoryPriority, PowerThrottlingState},
    diagnostics::{DiagnosticsError, Snapshot},
    kernel::Processor,
    process::{self, Process, ProcessError},
};
pub use win_api_wrapper::win32::system::threading::thread::access::ThreadAccessRights;
use win_api_wrapper::{
    core::WinError,
    win32::system::threading::{
        process::{MEMORY_PRIORITY_INFORMATION, STILL_ACTIVE},
        thread::{
            self,
            access::{THREAD_QUERY_INFORMATION, THREAD_SUSPEND_RESUME, THREAD_TERMINATE},
            THREAD_MODE_BACKGROUND_BEGIN, THREAD_MODE_BACKGROUND_END,
            THREAD_POWER_THROTTLING_STATE, THREAD_PRIORITY, THREAD_PRIORITY_ABOVE_NORMAL,
            THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_HIGHEST, THREAD_PRIORITY_LOWEST,
            THREAD_PRIORITY_NORMAL, THREAD_PRIORITY_TIME_CRITICAL,
        },
    },
};
use win_api_wrapper::{
    win32::foundation,
    win32::system::threading::thread::access::{
        THREAD_QUERY_LIMITED_INFORMATION, THREAD_SET_INFORMATION, THREAD_SET_LIMITED_INFORMATION,
    },
};

type Result<T> = core::result::Result<T, ThreadError>;

#[derive(Debug)]
pub enum ThreadError {
    WinError { code: i32, message: String },
    ConversionError(ConversionError),
    DiagnosticsError(DiagnosticsError),
    ProcessError(ProcessError),
}

impl From<WinError> for ThreadError {
    fn from(value: WinError) -> Self {
        ThreadError::WinError {
            code: value.code().0,
            message: value.message().to_string_lossy(),
        }
    }
}

impl From<ConversionError> for ThreadError {
    fn from(value: ConversionError) -> Self {
        ThreadError::ConversionError(value)
    }
}

impl From<DiagnosticsError> for ThreadError {
    fn from(value: DiagnosticsError) -> Self {
        ThreadError::DiagnosticsError(value)
    }
}

impl From<ProcessError> for ThreadError {
    fn from(value: ProcessError) -> Self {
        ThreadError::ProcessError(value)
    }
}

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    Lowest = THREAD_PRIORITY_LOWEST.0,
    BelowNormal = THREAD_PRIORITY_BELOW_NORMAL.0,
    #[default]
    Normal = THREAD_PRIORITY_NORMAL.0,
    AboveNormal = THREAD_PRIORITY_ABOVE_NORMAL.0,
    Highest = THREAD_PRIORITY_HIGHEST.0,
    TimeCritical = THREAD_PRIORITY_TIME_CRITICAL.0,
    BackgroundModeBegin = THREAD_MODE_BACKGROUND_BEGIN.0,
    BackgroundModeEnd = THREAD_MODE_BACKGROUND_END.0,
}

impl As<i32> for ThreadPriority {
    #[inline]
    fn as_type(&self) -> i32 {
        *self as i32
    }
}

impl From<ThreadPriority> for THREAD_PRIORITY {
    fn from(value: ThreadPriority) -> THREAD_PRIORITY {
        THREAD_PRIORITY(value.as_type())
    }
}

impl TryFrom<THREAD_PRIORITY> for ThreadPriority {
    type Error = ThreadError;
    fn try_from(value: THREAD_PRIORITY) -> Result<Self> {
        match value {
            THREAD_PRIORITY_LOWEST => Ok(Self::Lowest),
            THREAD_PRIORITY_BELOW_NORMAL => Ok(Self::BelowNormal),
            THREAD_PRIORITY_NORMAL => Ok(Self::Normal),
            THREAD_PRIORITY_ABOVE_NORMAL => Ok(Self::AboveNormal),
            THREAD_PRIORITY_HIGHEST => Ok(Self::Highest),
            THREAD_PRIORITY_TIME_CRITICAL => Ok(Self::TimeCritical),
            THREAD_MODE_BACKGROUND_BEGIN => Ok(Self::BackgroundModeBegin),
            THREAD_MODE_BACKGROUND_END => Ok(Self::BackgroundModeEnd),
            _ => Err(ConversionError::InvalidValue.into()),
        }
    }
}

pub struct ThreadHandle {
    id: isize,
}

impl ThreadHandle {
    fn try_open(pid: u32, access_rights: ThreadAccessRights, inherit_handle: bool) -> Result<Self> {
        Ok(ThreadHandle {
            id: thread::open(pid, access_rights, inherit_handle)?,
        })
    }

    fn try_with_access(pid: u32, access_rights: ThreadAccessRights) -> Result<Self> {
        ThreadHandle::try_open(pid, access_rights, false)
    }
}

impl Drop for ThreadHandle {
    fn drop(&mut self) {
        if let Err(error) = foundation::close_handle(self.id) {
            println!(
                "Error occured while closing thread handle: {}!",
                error.message().to_string_lossy(),
            );
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Thread {
    id: u32,
    owner_pid: u32,
}

impl Thread {
    pub(crate) fn new(id: u32, owner_pid: u32) -> Self {
        Thread { id, owner_pid }
    }

    fn try_open_handle_with_default_access(&self) -> Result<ThreadHandle> {
        ThreadHandle::try_with_access(self.id, THREAD_QUERY_LIMITED_INFORMATION)
    }

    fn try_open_handle_with_query_info_access(&self) -> Result<ThreadHandle> {
        ThreadHandle::try_with_access(self.id, THREAD_QUERY_INFORMATION)
    }

    fn try_open_handle_with_set_limited_info_access(&self) -> Result<ThreadHandle> {
        ThreadHandle::try_with_access(self.id, THREAD_SET_LIMITED_INFORMATION)
    }

    fn try_open_handle_with_set_info_access(&self) -> Result<ThreadHandle> {
        ThreadHandle::try_with_access(self.id, THREAD_SET_INFORMATION)
    }

    fn try_open_handle_with_suspend_resume_access(&self) -> Result<ThreadHandle> {
        ThreadHandle::try_with_access(self.id, THREAD_SUSPEND_RESUME)
    }

    fn try_open_handle_with_limited(&self) -> Result<ThreadHandle> {
        ThreadHandle::try_with_access(
            self.id,
            THREAD_QUERY_LIMITED_INFORMATION | THREAD_SET_LIMITED_INFORMATION,
        )
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn owner_pid(&self) -> u32 {
        self.owner_pid
    }

    pub fn owner(&self) -> Result<Option<Process>> {
        process::find_by_id(self.owner_pid).map_err(ThreadError::from)
    }

    pub fn exit_code(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_default_access()?;
        thread::get_exit_code(handle.id).map_err(ThreadError::from)
    }

    pub fn is_alive(&self) -> Result<bool> {
        let handle = self.try_open_handle_with_default_access()?;
        Ok(thread::get_exit_code(handle.id)? == STILL_ACTIVE.0 as u32)
    }

    pub fn description(&self) -> Result<Box<OsStr>> {
        let handle = self.try_open_handle_with_default_access()?;
        thread::get_description(handle.id).map_err(ThreadError::from)
    }

    pub fn set_description(&self, description: Box<OsStr>) -> Result<()> {
        let handle = self.try_open_handle_with_set_limited_info_access()?;
        thread::set_description(handle.id, description).map_err(ThreadError::from)
    }

    pub fn is_io_pending(&self) -> Result<bool> {
        let handle = self.try_open_handle_with_query_info_access()?;
        thread::is_io_pending(handle.id).map_err(ThreadError::from)
    }

    pub fn memory_priority(&self) -> Result<MemoryPriority> {
        let handle = self.try_open_handle_with_query_info_access()?;
        thread::get_information::<MEMORY_PRIORITY_INFORMATION>(handle.id)?
            .try_into()
            .map_err(ThreadError::from)
    }

    pub fn absolute_cpu_priority(&self) -> Result<i32> {
        let handle = self.try_open_handle_with_query_info_access()?;
        thread::get_information::<i32>(handle.id).map_err(ThreadError::from)
    }

    pub fn dynamic_code_policy(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_query_info_access()?;
        thread::get_information::<u32>(handle.id).map_err(ThreadError::from)
    }

    pub fn priority(&self) -> Result<ThreadPriority> {
        let handle = self.try_open_handle_with_default_access()?;
        thread::get_priority(handle.id)?
            .try_into()
            .map_err(ThreadError::from)
    }

    pub fn set_priority(&self, priority: ThreadPriority) -> Result<()> {
        let handle = self.try_open_handle_with_set_limited_info_access()?;
        thread::set_priority(handle.id, priority.into()).map_err(ThreadError::from)
    }

    pub fn has_priority_boost(&self) -> Result<bool> {
        let handle = self.try_open_handle_with_default_access()?;
        thread::has_priority_boost(handle.id).map_err(ThreadError::from)
    }

    pub fn set_priority_boost(&self, enable: bool) -> Result<()> {
        let handle = self.try_open_handle_with_set_limited_info_access()?;
        thread::set_priority_boost(handle.id, enable).map_err(ThreadError::from)
    }

    pub fn selected_cpu_set_count(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_default_access()?;
        thread::get_selected_cpu_set_count(handle.id).map_err(ThreadError::from)
    }

    pub fn selected_cpu_sets(&self) -> Result<Box<[u32]>> {
        let handle = self.try_open_handle_with_default_access()?;
        thread::get_selected_cpu_sets(handle.id).map_err(ThreadError::from)
    }

    pub fn clear_selected_cpu_sets(&self) -> Result<()> {
        let handle = self.try_open_handle_with_set_limited_info_access()?;
        thread::clear_selected_cpu_sets(handle.id).map_err(ThreadError::from)
    }

    pub fn set_selected_cpu_sets(&self, cpu_sets: &[u32]) -> Result<()> {
        let handle = self.try_open_handle_with_set_limited_info_access()?;
        thread::set_selected_cpu_sets(handle.id, cpu_sets).map_err(ThreadError::from)
    }

    pub fn set_power_throttling_state(&self, state: PowerThrottlingState) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        thread::set_information::<THREAD_POWER_THROTTLING_STATE>(handle.id, state.try_into()?)
            .map_err(ThreadError::from)
    }

    pub fn set_memory_priority(&self, memory_priority: MemoryPriority) -> Result<()> {
        let handle = self.try_open_handle_with_set_info_access()?;
        thread::set_information::<MEMORY_PRIORITY_INFORMATION>(handle.id, memory_priority.into())
            .map_err(ThreadError::from)
    }

    pub fn set_affinity_mask(&self, affinity_mask: usize) -> Result<usize> {
        let handle = self.try_open_handle_with_limited()?;
        thread::set_affinity_mask(handle.id, affinity_mask).map_err(ThreadError::from)
    }

    pub fn get_ideal_processor(&self) -> Result<Processor> {
        let handle = self.try_open_handle_with_default_access()?;
        thread::get_ideal_processor(handle.id)?
            .try_into()
            .map_err(ThreadError::from)
    }

    pub fn set_ideal_processor(&self, ideal_processor: Processor) -> Result<Processor> {
        let handle = self.try_open_handle_with_set_info_access()?;
        thread::set_ideal_processor(handle.id, ideal_processor.into())?
            .try_into()
            .map_err(ThreadError::from)
    }

    pub fn resume(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_suspend_resume_access()?;
        Ok(thread::resume(handle.id))
    }

    pub fn suspend(&self) -> Result<u32> {
        let handle = self.try_open_handle_with_suspend_resume_access()?;
        Ok(thread::suspend(handle.id))
    }

    pub fn terminate(&self, exit_code: u32) -> Result<()> {
        let handle = ThreadHandle::try_with_access(self.id, THREAD_TERMINATE)?;
        Ok(thread::terminate(handle.id, exit_code)?)
    }
}

pub fn switch() -> Result<()> {
    thread::switch_to_another().map_err(ThreadError::from)
}

pub fn current_id() -> u32 {
    process::current_id()
}

pub fn exit_current(exit_code: u32) {
    thread::exit_current(exit_code)
}

pub fn find_by_id(id: u32) -> Result<Option<Thread>> {
    Snapshot::<Thread>::try_create()
        .map(|mut snap| snap.find(|thread_info| thread_info.id() == id))
        .map_err(ThreadError::from)
}

pub fn threads() -> Result<impl Iterator<Item = Thread>> {
    Snapshot::<Thread>::try_create().map_err(ThreadError::from)
}
