/// `Win32::System::Threading::Process`
pub mod process;
/// `Win32::System::Threading::Thread`
pub mod thread;

pub use windows_sys::Win32::System::Threading::{
    MEMORY_PRIORITY_BELOW_NORMAL, MEMORY_PRIORITY_INFORMATION, MEMORY_PRIORITY_LOW,
    MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_NORMAL, MEMORY_PRIORITY_VERY_LOW,
    PROCESS_POWER_THROTTLING_STATE, REASON_CONTEXT,
};
