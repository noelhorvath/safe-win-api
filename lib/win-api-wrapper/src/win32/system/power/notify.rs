use windows_sys::Win32::System::Power::{
    PowerRegisterForEffectivePowerModeNotifications, PowerRegisterSuspendResumeNotification,
    PowerReportThermalEvent, PowerSettingRegisterNotification, PowerSettingUnregisterNotification,
    PowerUnregisterFromEffectivePowerModeNotifications, PowerUnregisterSuspendResumeNotification,
    RegisterPowerSettingNotification, RegisterSuspendResumeNotification,
    UnregisterPowerSettingNotification, UnregisterSuspendResumeNotification,
};

/// Sepcifies how the
enum DeviceNotify {
    Window,
    Service,
    /// Specifies that a parameter is a pointer to a callback function to call
    /// when the power setting changes, or a pointer to a [`DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`] structure.
    Callback,
}
/// The flag that specifies that a parameter is a window handle when registering
/// for notifications.
const DEVICE_NOTIFY_WINDOW_HANDLE: u32 = 0;

/// The flag that specifies that a parameter is a pointer to a callback function to call
/// when the power setting changes, or a pointer to a [`DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`] structure.
const DEVICE_NOTIFY_SERVICE_HANDLE: u32 = 1;

/// The flag that specifies that a parameter is a pointer to a callback function to call
/// when the power setting changes, or a pointer to a [`DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`] structure.
const DEVICE_NOTIFY_CALLBACK: u32 = 2;

/// A callback for handling effective power mode changes.
/// For more information about the possible effective power mode values
/// see the [Effectie power modes][register_for_effective_power_mode_notifications#effective-power-modes]
/// section.
///
/// It can be used with [`register_for_effective_power_mode_notifications`]
/// to supply a callback.
pub type EffectivePowerModeCallback = unsafe extern "system" fn(
    power_mode: i32, // EFFECTIVE_POWER_MODE
    context: *const ::core::ffi::c_void,
) -> ();

/// A callback function that will be called when the application receives the notification.
///
/// This callback is used as the `Callback` field of [``] [power_register_suspend_resume_notification].
pub type DeviceNotifyCallback =
    unsafe extern "system" fn(context: *const c_void, r#type: u32, setting: *const c_void) -> u32;

/// Registers a callback to receive effective power mode change notifications and
/// returns a handle to the registration.
///
/// Immediately after registration, the callback will be invoked with the current
/// value of the power setting. If the registration occurs while the power mode is
/// changing, you may receive multiple callbacks; the last callback is the most
/// recent update.
///
/// Use [`unregister_from_effective_power_mode_notifications`] with the registration
/// handle to unregister for notifications.
///
/// # Arguments
///
/// * `version`: Supplies the maximum effective power mode version the caller
///              understands.
///     * See [Versions][register_for_effective_power_mode_notifications#versions] for possible
///       values.
///     * If the effective power mode comes form a later version, it is reduced to a compatible
///       version that is then passed to the callback.
/// * `callback`: The callback to call when the effective power mode changes.
///     * This will also called one upon registration to supply the current mode.
///     * If multiple callbacks are registered using this API, those callbacks can be
///       called concurrently.
/// * `context`: Caller-specified opaque context.
///
/// ## Versions
///
/// | Version | Description |
/// |---------|-------------|
/// | [`EFFECTIVE_POWER_MODE_V1`] | This version is available starting with Windows 10, version 1809 and tracks the performance power slider and battery saver states. |
/// | [`EFFECTIVE_POWER_MODE_V2`] | This version is available starting with Windows 10, version 1809 and tracks the performance power slider and battery saver states. |
///
/// # Effective Power modes
///
/// The callback can receive on of the values listed in the table below that
/// indicates the effective power mode the system is running.
///
/// | Effective Power mode | Description |
/// |----------------------|-------------|
/// | [`EffectivePowerModeBatterySaver`] | The system is in battery saver mode. |
/// | [`EffectivePowerModeBetterBattery`] | The system is in the better battery effective power mode. |
/// | [`EffectivePowerModeBalanced`] | The system is in the balanced effective power mode. |
/// | [`EffectivePowerModeHighPerformance`] | The system is in the high performance effective power mode. This effective power mode is only used for systems using the legacy high performance overlay. |
/// | [`EffectivePowerModeMaxPerformance`] | The system is in the maximum performance effective power mode. |
/// | [`EffectivePowerModeGameMode`] | The system is in game mode power mode. This mode is only available with the [`EFFECTIVE_POWER_MODE_V2`] version of the API |
/// | [`EffectivePowerModeMixedReality`] | The system is in the windows mixed reality power mode. This mode is only available with the [`EFFECTIVE_POWER_MODE_V2`] version of the API. |
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerregisterforeffectivepowermodenotifications
///
pub fn register_for_effective_power_mode_notifications(
    version: u32,
    callback: EffectivePowerModeCallback,
    context: *const c_void,
) -> Result<*const c_void> {
    call_HRESULT! {
        PowerRegisterForEffectivePowerModeNotifications(
            version,
            Some(callback),
            context,
            addr_of_mut!(handle_ptr), // TODO: find out handle type (isize?)
        ) -> mut handle_ptr = ptr::null_mut::<c_void>()
    }
}

/// Unregisters from effective power mode change notifications.
/// This function is intended to be called from cleanup code and will wait
/// for all callbacks to complete before unregistering.
///
/// Immediately after registration, the callback will be invoked with the current
/// value of the power setting. If the registration occurs while the power mode is
/// changing, you may receive multiple callbacks; the last callback is the most
/// recent update.
///
/// Use [`register_for_effective_power_mode_notifications`] to register for
/// effective power mode change notifications.
///
/// # Arguments
///
/// * `registration_handle`: The handle corresponding to a single power mode registration.
///     * This handle should have been saved by the caller after the call to
///       [`register_for_effective_power_mode_notifications`] and passed in here.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerunregisterfromeffectivepowermodenotifications
///
pub fn unregister_from_effective_power_mode_notifications(
    registration_handle: *const c_void,
) -> Result<()> {
    call_HRESULT! {
        PowerUnregisterFromEffectivePowerModeNotifications(registration_handle)
    }
}

/// Registers to receive notification when the system is suspended or resumed and
/// returns a handle to the registration.
///
/// Use [`power_unregister_suspend_resume_notification`] with the registration
/// handle to unregister for notifications.
///
/// # Arguments
///
/// * `parameters`: A value of type [`DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`] that
///                 contains the parameters used when registering for suspend
///                 and resume notifications.
///     * The `Callback` field of this parameter contains a callback of type [`DeviceNotifyCallback`]
///       that will be called when the application recieves the notification.
///     * The `Context` field of this parameter contains the application context
///       for the notification.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-powerregistersuspendresumenotification
///
pub fn power_register_suspend_resume_notification(
    parameters: &DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS,
) -> Result<*const c_void> {
    call_WIN32_ERROR! {
         PowerRegisterSuspendResumeNotification(
            DEVICE_NOTIFY_CALLBACK,
            parameters as *const _ as isize,
            addr_of_mut!(registration_handle)
         ) -> mut registration_handle = ptr::null_mut::<c_void>()
    }
}

/// Cancels a registration to receive notification when the system is suspended or resumed.
///
/// Use [`register_for_effective_power_mode_notifications`] to register for
/// notifications when the system is suspended or resumed.
///
/// # Arguments
///
/// * `registration`: A handle to a registration obtained by calling
///                   [`power_register_suspend_resume_notification`].
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-powerunregistersuspendresumenotification
///
pub fn power_unregister_suspend_resume_notification(
    registration_handle: *const c_void,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerUnregisterSuspendResumeNotification(registration_handle as isize)
    }
}

/// Notifies the operating system of thermal events.
///
/// Thermal managers call the [`report_thermal_event`] to notify the operating system
/// of a thermal event so that the event can be recorded in the system event
/// log. Before calling this function, the thermal manager sets the members of
/// the [`THERMAL_EVENT`] structure to describe the thermal event.
///
/// # Arguments
///
/// * `event`: A thermal event of type [`THERMAL_EVENT`].
///     * For more information see the official documentation of the [THERMAL_EVENT] structure.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-powerunregistersuspendresumenotification
/// [THERMAL_EVENT]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/ns-powrprof-thermal_event
///
pub fn report_thermal_event(event: &THERMAL_EVENT) -> Result<()> {
    call_WIN32_ERROR! { PowerReportThermalEvent(event) }
}

/// Registers to receive notification when the system is suspended or resumed.
/// Similar to [`power_register_suspend_resume_notification`], but operates in user
/// mode and can take a window handle.
///
/// Use [`unregister_suspend_resume_notification`] with the registration
/// handle to unregister for notifications.
///
/// # Arguments
///
/// * `parameters`: A value of type [`DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`] that
///                 contains the parameters used when registering for suspend
///                 and resume notifications.
///     * The `Callback` field of this parameter contains a callback of type [`DeviceNotifyCallback`]
///       that will be called when the application recieves the notification.
///     * The `Context` field of this parameter contains the application context
///       for the notification.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registersuspendresumenotification
///
pub fn register_suspend_resume_notification(
    parameters: &DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS,
) -> Result<isize> {
    call! {
        RegisterSuspendResumeNotification(
            parameters as *const _ as isize,
            DEVICE_NOTIFY_CALLBACK,
        ) != 0
    }
}

/// Registers to deliver events the window specified by `window_handle`when the system
/// is suspended or resumed. Similar to [`power_register_suspend_resume_notification`],
/// but operates in user mode and can take a window handle.
///
/// Use [`unregister_suspend_resume_notification`] with the registration
/// handle to unregister for notifications.
///
/// # Arguments
///
/// * `window_handle`: The handle to the window to deliver the events to.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registersuspendresumenotification
///
pub fn register_suspend_resume_notification_for_window(window_handle: isize) -> Result<isize> {
    call! {
        RegisterSuspendResumeNotification(
            window_handle,
            DEVICE_NOTIFY_WINDOW_HANDLE,
        ) != 0
    }
}

/// Cancels a registration to receive notification when the system is suspended or resumed.
/// Similar to [`power_unregister_suspend_resume_notification`] but operates in user mode.
///
/// Use [`register_suspend_resume_notification`] to register for
/// notifications when the system is suspended or resumed.
///
/// Use [`register_suspend_resume_notification_for_window`] to register for
/// notifications that are delivered to a specific window when the system
/// is suspended or resumed.
///
/// # Arguments
///
/// * `registration`: A handle to a registration obtained by calling [`power_register_suspend_resume_notification`].
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregistersuspendresumenotification
///
pub fn unregister_suspend_resume_notification(registration_handle: isize) -> Result<()> {
    call_BOOL! { UnregisterSuspendResumeNotification(registration_handle) }
}

/// Registers to receive notification when a power setting changes.
///
/// # Arguments
///
/// * `setting`: The identifier of the power setting.
/// * `callback`: The callback of type [`DeviceNotifyCallback`] to call when the power setting changes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_INVALID_PARAMETER`][windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER]
///     * `scheme` does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powersetactivescheme
///
pub fn register_setting_notification(
    setting: &GUID,
    recipient_handle: DeviceNotifyCallback,
) -> Result<*mut c_void> {
    call_WIN32_ERROR! {
        PowerSettingRegisterNotification(
            setting,
            DEVICE_NOTIFY_CALLBACK,
            recipient_handle as isize,
            addr_of_mut!(registration_handle),
        ) -> mut registration_handle = ptr::null_mut()
    }
}

pub fn register_setting_notification_for_service(
    setting: &GUID,
    service_handle: isize,
) -> Result<*mut c_void> {
    call_WIN32_ERROR! {
        PowerSettingRegisterNotification(
            setting,
            DEVICE_NOTIFY_SERVICE_HANDLE,
            recipient_handle,
            addr_of_mut!(registration_handle), // windows_sys::Win32::System::Power::HPOWERNOTIFY ???
        ) -> mut registration_handle = ptr::null_mut()
    }
}

/// registration_handle: [`HPOWERNOTIFY`][windows_sys::Win32::System::Power::HPOWERNOTIFY]
pub fn unregister_setting_notification(registration_handle: isize) -> Result<()> {
    call_WIN32_ERROR! { PowerSettingUnregisterNotification(registration_handle) }
}

/// For interactive applications, the `flags` parameter should be `0`, and the `window_handle` parameter should be a window handle.
///
/// For services, the `flags` parameter should be `1`, and the `window_handle` parameter should be a SERVICE_STATUS_HANDLE
pub fn register_setting_notification_for_window(
    window_handle: isize,
    setting: &GUID,
    flags: u32,
) -> Result<isize> {
    call! {
        RegisterPowerSettingNotification(
            window_handle,
            setting,
            flags,
        ) != 0
    }
}

pub fn unregister_setting_notification_for_window(registration_handle: isize) -> Result<()> {
    call_BOOL! { UnregisterPowerSettingNotification(registration_handle) }
}
