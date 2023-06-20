use crate::core::Result;
use crate::{call_BOOL, default_sized};
use windows_sys::Win32::System::Power::{
    GetCurrentPowerPolicies, IsAdminOverrideActive, ReadGlobalPwrPolicy, WriteGlobalPwrPolicy,
};

pub use windows_sys::Win32::System::Power::{
    ADMINISTRATOR_POWER_POLICY, GLOBAL_POWER_POLICY, POWER_POLICY,
};

/// Gets the current global power policy settings and the power policy settings that
/// are unique to the active power scheme.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-getcurrentpowerpolicies
///
pub fn get_current_policies() -> Result<(GLOBAL_POWER_POLICY, POWER_POLICY)> {
    call_BOOL! {
         GetCurrentPowerPolicies(
            &mut global,
            &mut active,
        ) -> mut (global, active) = default_sized!((GLOBAL_POWER_POLICY, POWER_POLICY))
    }
}

/// Writes global power policy settings.
///
/// This function replaces any existing power policy settings.
/// Each user has a separate global power scheme, which contains power policy
/// settings that apply to all power schemes for that user.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-writeglobalpwrpolicy
///
pub fn write_global(global_policy: &GLOBAL_POWER_POLICY) -> Result<()> {
    call_BOOL! {
        WriteGlobalPwrPolicy(global_policy)
    }
}

/// Writes global power policy settings.
///
/// This function replaces any existing power policy settings.
/// Each user has a separate global power scheme, which contains power policy
/// settings that apply to all power schemes for that user.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-readglobalpwrpolicy
///
pub fn read_global() -> Result<GLOBAL_POWER_POLICY> {
    call_BOOL! {
        ReadGlobalPwrPolicy(&mut policy)
            -> mut policy = default_sized!(GLOBAL_POWER_POLICY)
    }
}

/// Determines whether admin override is active for the administrator power policy specified by `admin_power_policy`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// TODO
///
pub fn is_admin_override_active(power_policy: &ADMINISTRATOR_POWER_POLICY) -> Result<bool> {
    call_BOOL! { IsAdminOverrideActive(power_policy) -> bool }
}
