use crate::call_BOOL;
use crate::common::To;
use crate::win32::core::Result;
use core::mem::transmute;
use core::ptr::addr_of;
use core::ptr::addr_of_mut;
use core::time::Duration;
use windows_sys::Win32::System::Time::FileTimeToSystemTime;

pub use windows_sys::Win32::Foundation::{FILETIME, SYSTEMTIME};

impl To<Duration> for FILETIME {
    fn to(&self) -> Duration {
        // Safety: `FILETIME` has the same size as `u64`
        let intervals = unsafe { transmute::<Self, u64>(*self) };
        Duration::from_millis(intervals / 10_000)
    }
}

/// Converts a [`FILETIME`] to [`SYSTEMTIME`] format. System time is based on Coordinated Universal Time (UTC).
///
/// # Errors
///
/// Returns a [`Win32Error`][crate::win32::core::Win32Error] if the function fails.
///
/// ## Possible errors
///
/// * `file_time` is greater than or equal to `0x8000000000000000`
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-filetimetosystemtime
///
pub fn file_time_to_system_time(file_time: FILETIME) -> Result<SYSTEMTIME> {
    call_BOOL! {
        FileTimeToSystemTime(
            addr_of!(file_time),
            addr_of_mut!(sys_time),
        ) -> mut sys_time: SYSTEMTIME
    }
}
