use crate::core::Result;
use crate::{call_NTSTATUS, default_sized, to_BOOL};
use core::mem::size_of;
use core::ptr::{self, addr_of, addr_of_mut};
use windows_sys::Win32::System::Power::{
    CallNtPowerInformation, LastSleepTime, LastWakeTime, ProcessorInformation, SystemBatteryState,
    SystemExecutionState, SystemPowerCapabilities, SystemPowerInformation, SystemPowerPolicyAc,
    SystemPowerPolicyCurrent, SystemPowerPolicyDc, SystemReserveHiberFile,
};

pub use windows_sys::Win32::System::Power::{
    POWER_INFORMATION_LEVEL, PROCESSOR_POWER_INFORMATION, SYSTEM_BATTERY_STATE,
    SYSTEM_POWER_CAPABILITIES, SYSTEM_POWER_INFORMATION, SYSTEM_POWER_POLICY,
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
/// Represents the system power policies.
pub enum PowerPolicy {
    #[default]
    /// The current system power policy.
    Current,
    /// The power policy used while the system is running on AC power.
    AC,
    /// The power policy used while the system is running on battery power.
    DC,
}

impl PowerPolicy {
    /// Gets the associated [`POWER_INFORMATION_LEVEL`] for the power policy.
    pub(crate) const fn information_level(self) -> POWER_INFORMATION_LEVEL {
        match self {
            Self::Current => SystemPowerPolicyCurrent,
            Self::AC => SystemPowerPolicyAc,
            Self::DC => SystemPowerPolicyDc,
        }
    }
}

impl core::fmt::Display for PowerPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Current => write!(f, "Current"),
            Self::AC => write!(f, "AC"),
            Self::DC => write!(f, "DC"),
        }
    }
}

/// Gets the interrupt-time count, in 100-nanosecond units, at the last system sleep time.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_last_sleep_time() -> Result<u64> {
    call_NTSTATUS! {
        CallNtPowerInformation(
            LastSleepTime,
            ptr::null(),
            0,
            addr_of_mut!(last_sleep_time).cast(),
            size_of::<u64>() as u32,
        ) -> mut last_sleep_time: u64
    }
}

/// Gets the interrupt-time count, in 100-nanosecond units, at the last system wake time.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_last_wake_time() -> Result<u64> {
    call_NTSTATUS! {
        CallNtPowerInformation(
            LastWakeTime,
            ptr::null(),
            0,
            addr_of_mut!(last_wake_time).cast(),
            size_of::<u64>() as u32,
        ) -> mut last_wake_time: u64
    }
}

/// Gets information about the current system battery.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_battery_state() -> Result<SYSTEM_BATTERY_STATE> {
    call_NTSTATUS! {
        CallNtPowerInformation(
            SystemBatteryState,
            ptr::null(),
            0,
            addr_of_mut!(battery_state).cast(),
            size_of::<SYSTEM_BATTERY_STATE>() as u32,
        ) -> mut battery_state = default_sized!(SYSTEM_BATTERY_STATE)
    }
}

/// Returns an array of [`PROCESSOR_POWER_INFORMATION`] that contains power information for each processor.
/// The `cpu_count` paramater cannot be higher than the number of processors installed on the system.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_processor_info(cpu_count: u8) -> Result<Box<[PROCESSOR_POWER_INFORMATION]>> {
    let mut buffer = Vec::<PROCESSOR_POWER_INFORMATION>::with_capacity(cpu_count as usize);
    call_NTSTATUS! {
        CallNtPowerInformation(
            ProcessorInformation,
            ptr::null(),
            0,
            buffer.as_mut_ptr().cast(),
            (size_of::<PROCESSOR_POWER_INFORMATION>() * cpu_count as usize) as u32,
        ) return Error
    };
    // Safety: `cpu_count` is less than equal to the capacity of `buffer` and every element is initialized
    unsafe {
        buffer.set_len(cpu_count as usize);
    }
    Ok(buffer.into_boxed_slice())
}

/// Gets the system execution state buffer. This value may contain any combination of the following values:
/// [`ES_SYSTEM_REQUIRED`][super::ES_SYSTEM_REQUIRED], [`ES_DISPLAY_REQUIRED`][super::ES_DISPLAY_REQUIRED],
/// or [`ES_USER_PRESENT`][super::ES_USER_PRESENT].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_execution_state() -> Result<u32> {
    call_NTSTATUS! {
        CallNtPowerInformation(
            SystemExecutionState,
            ptr::null(),
            0,
            addr_of_mut!(execution_state).cast(),
            size_of::<u32>() as u32,
        ) -> mut execution_state: u32
    }
}

/// Gets power capabilities of the current system.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_power_capabilities() -> Result<SYSTEM_POWER_CAPABILITIES> {
    call_NTSTATUS! {
        CallNtPowerInformation(
            SystemPowerCapabilities,
            ptr::null(),
            0,
            addr_of_mut!(power_capabilites).cast(),
            size_of::<SYSTEM_POWER_CAPABILITIES>() as u32,
        ) -> mut power_capabilites = default_sized!(SYSTEM_POWER_CAPABILITIES)
    }
}

/// Gets power capabilities of the current system.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_power_info() -> Result<SYSTEM_POWER_INFORMATION> {
    call_NTSTATUS! {
        CallNtPowerInformation(
            SystemPowerInformation,
            ptr::null(),
            0,
            addr_of_mut!(power_info).cast(),
            size_of::<SYSTEM_POWER_INFORMATION>() as u32,
        ) -> mut power_info = default_sized!(SYSTEM_POWER_INFORMATION)
    }
}

/// Gets the power policy specified by `policy_type`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn get_power_policy(policy_type: PowerPolicy) -> Result<SYSTEM_POWER_POLICY> {
    call_NTSTATUS! {
        CallNtPowerInformation(
            policy_type.information_level(),
            ptr::null(),
            0,
            addr_of_mut!(power_policy).cast(),
            size_of::<SYSTEM_POWER_POLICY>() as u32,
        ) -> mut power_policy = default_sized!(SYSTEM_POWER_POLICY)
    }
}

/// Reserves or removes the system hibernation file depending on the value of `reserve`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn reserve_hibernation_file(reserve: bool) -> Result<()> {
    let input: u8 = to_BOOL!(reserve);
    call_NTSTATUS! {
        CallNtPowerInformation(
            SystemReserveHiberFile,
            addr_of!(input).cast(),
            size_of::<u8>() as u32,
            ptr::null_mut(),
            0,
        )
    }
}

/// Enables and disables the hibernate feature on the system.
///
///
/// If the `enable` parameter is
/// * `true`, the system reserves space for a hibernation file and enables hibernation.
/// * `false`, the system removes the hibernation file and disables hibernation.
///
/// # Remarks
///
/// `Fast Startup` can only be used if hibernation is enabled.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn set_hibernation(enable: bool) -> Result<()> {
    let input: u8 = to_BOOL!(enable);
    call_NTSTATUS! {
        CallNtPowerInformation(
            SystemReserveHiberFile,
            addr_of!(input).cast(),
            size_of::<u8>() as u32,
            ptr::null_mut(),
            0,
        )
    }
}
