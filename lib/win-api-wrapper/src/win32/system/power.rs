use super::threading::REASON_CONTEXT;
use crate::core::error::{Code, Error};
use crate::core::{Result, To};
use crate::win32::foundation::get_last_error;
use crate::{call, call_BOOL, default_sized, to_BOOL};
use windows_sys::Win32::Foundation::ERROR_SUCCESS;
pub use windows_sys::Win32::System::Power::{
    EffectivePowerModeBalanced, EffectivePowerModeBatterySaver, EffectivePowerModeBetterBattery,
    EffectivePowerModeGameMode, EffectivePowerModeHighPerformance,
    EffectivePowerModeMaxPerformance, EffectivePowerModeMixedReality, PowerRequestAwayModeRequired,
    PowerRequestDisplayRequired, PowerRequestExecutionRequired, PowerRequestSystemRequired,
    ACCESS_ACTIVE_SCHEME, ACCESS_AC_POWER_SETTING_INDEX, ACCESS_CREATE_SCHEME,
    ACCESS_DC_POWER_SETTING_INDEX, ACCESS_SCHEME, ADMINISTRATOR_POWER_POLICY,
    DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS, EFFECTIVE_POWER_MODE_V1, EFFECTIVE_POWER_MODE_V2,
    ES_AWAYMODE_REQUIRED, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, ES_USER_PRESENT,
    POWER_INFORMATION_LEVEL, POWER_PLATFORM_ROLE_V1, POWER_PLATFORM_ROLE_V2,
    POWER_PLATFORM_ROLE_VERSION, PROCESSOR_POWER_INFORMATION, SYSTEM_BATTERY_STATE,
    SYSTEM_POWER_CAPABILITIES, SYSTEM_POWER_INFORMATION, SYSTEM_POWER_POLICY, SYSTEM_POWER_STATUS,
    THERMAL_EVENT,
};
use windows_sys::Win32::System::Power::{
    GetSystemPowerStatus, IsSystemResumeAutomatic, PowerClearRequest, PowerCreateRequest,
    PowerDeterminePlatformRoleEx, PowerSetRequest, SetSuspendState, SetThreadExecutionState,
};

pub mod device;
pub mod info;
pub mod policy;
pub mod scheme;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
/// Indicates the OEM's preferred power management profile. These values are read from the
/// `Preferred_PM_Profile` field of the `Fixed ACPI Description Table` (`FADT`).
///
/// To determine the system platform role, use [`get_platform_role`].
///
pub enum PlatformRole {
    #[default]
    /// The OEM did not specify a specific role.
    Unspecified,
    /// The OEM specified a desktop role.
    Desktop,
    /// The OEM specified a mobile role (for example, a laptop).
    Mobile,
    /// The OEM specified a workstation role.
    Workstation,
    /// The OEM specified an enterprise server role.
    EnterpriseServer,
    /// The OEM specified a single office/home office (SOHO) server role.
    SOHOServer,
    /// The OEM specified an appliance PC role.
    AppliancePC,
    /// The OEM specified a performance server role.
    PerformanceServer,
    /// The OEM specified a tablet form factor role.
    Slate,
}

impl To<Option<PlatformRole>> for i32 {
    #[inline]
    fn to(&self) -> Option<PlatformRole> {
        match *self {
            0 => Some(PlatformRole::Unspecified),
            1 => Some(PlatformRole::Desktop),
            2 => Some(PlatformRole::Mobile),
            3 => Some(PlatformRole::Workstation),
            4 => Some(PlatformRole::EnterpriseServer),
            5 => Some(PlatformRole::SOHOServer),
            6 => Some(PlatformRole::AppliancePC),
            7 => Some(PlatformRole::PerformanceServer),
            8 => Some(PlatformRole::Slate),
            _ => None,
        }
    }
}

impl core::fmt::Display for PlatformRole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "Unspecified"),
            Self::Desktop => write!(f, "Desktop"),
            Self::Mobile => write!(f, "Mobile"),
            Self::Workstation => write!(f, "Workstation"),
            Self::EnterpriseServer => write!(f, "Enterprise server"),
            Self::SOHOServer => write!(f, "SOHO server"),
            Self::AppliancePC => write!(f, "Appliance PC"),
            Self::PerformanceServer => write!(f, "Performance server"),
            Self::Slate => write!(f, "Slate"),
        }
    }
}

/// Gets the power status of the system. The status indicates whether the system is
/// running on AC or DC power, whether the battery is currently discharging, how much
/// battery life remains, and if the battery saver is on or off.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getsystempowerstatus
///
pub fn get_system_power_status() -> Result<SYSTEM_POWER_STATUS> {
    call_BOOL! {
        GetSystemPowerStatus(
            &mut power_status
        ) -> mut power_status = default_sized!(SYSTEM_POWER_STATUS)
    }
}

/// Determines the current state of the computer.
///
/// If the system was restored to the working state automatically and the
/// user is not active, the returned value is `true`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-issystemresumeautomatic
///
pub fn is_system_resume_automatic() -> Result<bool> {
    call_BOOL! { IsSystemResumeAutomatic() -> bool }
}

/// Decrements the count of power requests of the specified type for a power
/// request object.
///
/// # Arguments
///
/// * `request_handle`: A handle to a power request object.
/// * `request_type`: The power request type to be decremented.
///     * See [Power request types][set_request#power-request-types]
///       for possbile values.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-powerclearrequest
///
pub fn clear_request(request_handle: isize, request_type: i32) -> Result<()> {
    call_BOOL! { PowerClearRequest(request_handle, request_type) }
}

/// Creates a new power request object from the specified information
/// about the power request.
///
/// When the power request object is no longer needed, use [`close_handle`][crate::win32::foundation::close_handle]
/// to free the handle clean up the object.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-powercreaterequest
///
pub fn create_request(context: REASON_CONTEXT) -> Result<isize> {
    call! { PowerCreateRequest(&context) != 0 }
}

/// Increments the count of power requests of the specified type for a power request object.
///
/// On Modern Standby systems on DC power, system and execution required power requests are
/// terminated `5` minutes after the system sleep timeout has expired.
///
/// Except for [`PowerRequestAwayModeRequired`] on Traditional Sleep (S3) systems, power requests
/// are terminated upon user-initiated system sleep entry (power button, lid close or
/// selecting `Sleep` from the `Start` menu).
///
/// To conserve power and provide the best user experience, applications that use power
/// requests should follow these best practices:
/// * When creating a power request, provide a localized text string that describes
///   the reason for the request in the [`REASON_CONTEXT`] instance.
/// * Call [`set_request`] immediately before the scenario that requires the request.
/// * Call [`clear_request`] to decrement the reference count for the request as
///   soon as the scenario is finished.
/// * Clean up all request objects and associated handles before the process exits
///   or the service stops.
///
/// # Arguments
///
/// * `request_handle`: A handle to a power request object.
/// * `request_type`: The power request type to be decremented.
///     * See [Power request types][set_request#power-request-types]
///       for possbile values.
///
/// ## Power request types
///
/// | Type | Description |
/// |------|-------------|
/// | [`PowerRequestDisplayRequired`] | The display remains on even if there is no user input for an extended period of time. |
/// | [`PowerRequestSystemRequired`] | The system continues to run instead of entering sleep after a period of user inactivity. |
/// | [`PowerRequestAwayModeRequired`] | The system enters away mode instead of sleep. In away mode, the system continues to run but turns off audio and video to give the appearance of sleep. [`PowerRequestAwayModeRequired`] is only applicable on Traditional Sleep (S3) systems. |
/// | [`PowerRequestExecutionRequired`] | The calling process continues to run instead of being suspended or terminated by process lifetime management mechanisms. When and how long the process is allowed to run depends on the operating system and power policy settings. On Traditional Sleep (S3) systems, an active [`PowerRequestExecutionRequired`] request implies [`PowerRequestSystemRequired`]. The [`PowerRequestExecutionRequired`] request type can be used only by applications. Services cannot use this request type. This request type is supported starting with Windows 8 and Windows Server 2012. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-powerclearrequest
///
pub fn set_request(power_request_handle: isize, request_type: i32) -> Result<()> {
    call_BOOL! { PowerSetRequest(power_request_handle, request_type) }
}

/// Gets the [`platform role`][PlatformRole] of the current system and
/// returns `None` if the retrieved platform role is unsupported on the
/// current version of Windows that the system is running on.
///
/// If the specified returned platform role is not supported on the
/// current version of Windows that the system is using, the return
/// value is `None`. For example, on Windows 7, Windows Server 2008 R2,
/// Windows Vista or Windows Server 2008 [`PlatformRole::Slate`] is unsupported
/// and therefore `None` is returned.
///
/// This function reads the `ACPI Fixed ACPI Description Table` (`FADT`)
/// to determine the OEM preferred computer role. If that information is
/// not available, the function looks for a battery. If at least one battery
/// is available, the function returns [`PlatformRole::Mobile`]. If no batteries
/// are available, the function returns [`PlatformRole::Desktop`].
///
/// If the OEM preferred computer role is not supported on the platform specified
/// by the caller, the function returns the closest supported value.
///
/// # Arguments
///
/// * `version`: The version of the platform role enumeration to use.
///     * See [Power platform role versions].
///
/// ## Power platform role versions.
///
/// | Role | Description |
/// |------|-------------|
/// | [`POWER_PLATFORM_ROLE_V1`] | The version for Windows 7, Windows Server 2008 R2, Windows Vista or Windows Server 2008. |
/// | [`POWER_PLATFORM_ROLE_V2`] | The version for Windows 8, Windows Server 2012 and newer versions of Windows. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-powerdetermineplatformroleex
///
pub fn get_platform_role(version: u32) -> Result<Option<PlatformRole>> {
    let result: Option<PlatformRole> = unsafe { PowerDeterminePlatformRoleEx(version) }.to();
    let last_error_code = get_last_error();
    if last_error_code == ERROR_SUCCESS {
        Ok(result)
    } else {
        Err(Error::from_code(Code::from_win32(last_error_code)))
    }
}

/// Suspends the system by shutting power down. Depending on the Hibernate parameter,
/// the system either enters a suspend (sleep) state or hibernation (S4).
///
/// # Arguments
///
/// * `hibernate`: Specifies whether to hibernate the system.
/// * `disable_wake_events`: Specifies wheter to disables all wake events or keep them enabled.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-setsuspendstate
///
pub fn set_suspend_state(hibernate: bool, disable_wake_events: bool) -> Result<()> {
    call_BOOL! {
        SetSuspendState(
            to_BOOL!(hibernate),
            to_BOOL!(false), // has no effect
            to_BOOL!(disable_wake_events)
        )
    }
}

/// Enables an application to inform the system that it is in use, thereby
/// preventing the system from entering sleep or turning off the display
/// while the application is running.
///
/// The system automatically detects activities such as local keyboard or
/// mouse input, server activity, and changing window focus. Activities
/// that are not automatically detected include disk or CPU activity and
/// video display.
///
/// Calling [`set_thread_execution_state`] without the `requirements` parameter with [`ES_CONTINUOUS`]
/// simply resets the idle timer; to keep the display or system in the working state,
/// the thread must call this function periodically.
///
/// # Arguments
///
/// * `requirements`: The thread's execution requirements.
///     * See [Execution requirements][set_thread_execution_state#execution-requirements]
///       for possible values.
///
/// ## Execution requirements
///
/// | Execution requirement flag | Meaning |
/// |------|---------|
/// | [`ES_AWAYMODE_REQUIRED`] | Enables away mode. This value must be specified with [`ES_CONTINUOUS`]. Away mode should be used only by media-recording and media-distribution applications that must perform critical background processing on desktop computers while the computer appears to be sleeping. |
/// | [`ES_CONTINUOUS`] | Informs the system that the state being set should remain in effect until the next call that uses [`ES_CONTINUOUS`] and one of the other state flags is cleared. |
/// | [`ES_DISPLAY_REQUIRED`] | Forces the display to be on by resetting the display idle timer. |
/// | [`ES_SYSTEM_REQUIRED`] | Forces the system to be in the working state by resetting the system idle timer. |
/// | [`ES_USER_PRESENT`] | This value is not supported. If [`ES_USER_PRESENT`] is combined with other execution requirement flags values, the call will fail and none of the specified states will be set. |
///
/// # Remarks
///
/// See the [Remarks] section in the official documentation.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate
/// [Remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate#remarks
///
pub fn set_thread_execution_state(requirements: u32) -> Result<u32> {
    call! { SetThreadExecutionState(requirements) != 0 }
}
