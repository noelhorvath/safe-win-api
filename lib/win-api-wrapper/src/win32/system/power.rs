use super::registry::RegistryValue;
use super::threading::REASON_CONTEXT;
use crate::core::{get_string_len_from_bytes, guid_from_array, Result};
use crate::win32::foundation::MAX_PATH;
use crate::{
    call, call_BOOL, call_HRESULT, call_NTSTATUS, call_WIN32_ERROR, default_sized, free, to_BOOL,
};
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::{self, addr_of, addr_of_mut};
use widestring::{U16CStr, U16CString};
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::ERROR_NO_MORE_ITEMS;
use windows_sys::Win32::System::Power::{
    CallNtPowerInformation, DevicePowerClose, DevicePowerEnumDevices, DevicePowerOpen,
    DevicePowerSetDeviceState, GetCurrentPowerPolicies, GetDevicePowerState, GetSystemPowerStatus,
    IsAdminOverrideActive, IsSystemResumeAutomatic, PowerCanRestoreIndividualDefaultPowerScheme,
    PowerClearRequest, PowerCreatePossibleSetting, PowerCreateRequest, PowerCreateSetting,
    PowerDeleteScheme, PowerDeterminePlatformRoleEx, PowerDuplicateScheme, PowerEnumerate,
    PowerGetActiveScheme, PowerImportPowerScheme, PowerIsSettingRangeDefined,
    PowerOpenSystemPowerKey, PowerOpenUserPowerKey, PowerReadACDefaultIndex, PowerReadACValue,
    PowerReadACValueIndex, PowerReadDCDefaultIndex, PowerReadDCValue, PowerReadDCValueIndex,
    PowerReadDescription, PowerReadFriendlyName, PowerReadIconResourceSpecifier,
    PowerReadPossibleDescription, PowerReadPossibleFriendlyName, PowerReadPossibleValue,
    PowerReadSettingAttributes, PowerReadValueIncrement, PowerReadValueMax, PowerReadValueMin,
    PowerReadValueUnitsSpecifier, PowerRegisterForEffectivePowerModeNotifications,
    PowerRegisterSuspendResumeNotification, PowerRemovePowerSetting,
    PowerReplaceDefaultPowerSchemes, PowerReportThermalEvent, PowerRestoreDefaultPowerSchemes,
    PowerSetActiveScheme, PowerSetRequest, PowerSettingAccessCheckEx,
    PowerSettingRegisterNotification, PowerSettingUnregisterNotification,
    PowerUnregisterFromEffectivePowerModeNotifications, PowerUnregisterSuspendResumeNotification,
    PowerWriteACDefaultIndex, PowerWriteACValueIndex, PowerWriteDCDefaultIndex,
    PowerWriteDCValueIndex, PowerWriteDescription, PowerWriteFriendlyName,
    PowerWriteIconResourceSpecifier, PowerWritePossibleDescription, PowerWritePossibleFriendlyName,
    PowerWritePossibleValue, PowerWriteValueIncrement, PowerWriteValueMax, PowerWriteValueMin,
    PowerWriteValueUnitsSpecifier, RegisterPowerSettingNotification,
    RegisterSuspendResumeNotification, RequestWakeupLatency, SetSuspendState, SetSystemPowerState,
    SetThreadExecutionState, UnregisterPowerSettingNotification,
    UnregisterSuspendResumeNotification, ACCESS_INDIVIDUAL_SETTING, ACCESS_SUBGROUP, THERMAL_EVENT,
};
pub use windows_sys::Win32::System::Power::{
    LastSleepTime, LastWakeTime, PowerRequestAwayModeRequired, PowerRequestDisplayRequired,
    PowerRequestExecutionRequired, PowerRequestSystemRequired, ProcessorInformation,
    SystemBatteryState, SystemExecutionState, SystemPowerCapabilities, SystemPowerInformation,
    SystemPowerPolicyAc, SystemPowerPolicyCurrent, SystemPowerPolicyDc, SystemReserveHiberFile,
    ACCESS_ACTIVE_SCHEME, ACCESS_AC_POWER_SETTING_INDEX, ACCESS_CREATE_SCHEME,
    ACCESS_DC_POWER_SETTING_INDEX, ACCESS_SCHEME, ADMINISTRATOR_POWER_POLICY,
    DEVICEPOWER_AND_OPERATION, DEVICEPOWER_CLEAR_WAKEENABLED, DEVICEPOWER_FILTER_DEVICES_PRESENT,
    DEVICEPOWER_FILTER_ON_NAME, DEVICEPOWER_FILTER_WAKEENABLED,
    DEVICEPOWER_FILTER_WAKEPROGRAMMABLE, DEVICEPOWER_HARDWAREID, DEVICEPOWER_SET_WAKEENABLED,
    EFFECTIVE_POWER_MODE_V1, EFFECTIVE_POWER_MODE_V2, ES_AWAYMODE_REQUIRED, ES_CONTINUOUS,
    ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, ES_USER_PRESENT, GLOBAL_POWER_POLICY,
    PDCAP_S0_SUPPORTED, PDCAP_S1_SUPPORTED, PDCAP_S2_SUPPORTED, PDCAP_S3_SUPPORTED,
    PDCAP_S4_SUPPORTED, PDCAP_S5_SUPPORTED, PDCAP_WAKE_FROM_S0_SUPPORTED,
    PDCAP_WAKE_FROM_S1_SUPPORTED, PDCAP_WAKE_FROM_S2_SUPPORTED, PDCAP_WAKE_FROM_S3_SUPPORTED,
    POWER_INFORMATION_LEVEL, POWER_POLICY, PROCESSOR_POWER_INFORMATION, SYSTEM_BATTERY_STATE,
    SYSTEM_POWER_CAPABILITIES, SYSTEM_POWER_INFORMATION, SYSTEM_POWER_POLICY, SYSTEM_POWER_STATUS,
};

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

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum PowerPolicy {
    #[default]
    Current,
    AC,
    DC,
}

impl PowerPolicy {
    pub(crate) fn information_level(self) -> POWER_INFORMATION_LEVEL {
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

/// Used with [`enum_devices`].
/// Check https://learn.microsoft.com/en-us/windows/win32/power/using-the-device-power-api
pub fn close_device_list() -> Result<()> {
    call_BOOL! { DevicePowerClose() }
}

/// Check https://learn.microsoft.com/en-us/windows/win32/power/using-the-device-power-api
pub fn open_device_list() -> Result<()> {
    call_BOOL! { DevicePowerOpen(0) }
}

/// # Remarks
///
/// See the official [example][https://learn.microsoft.com/en-us/windows/win32/power/using-the-device-power-api].
///
/// # Examples
///
/// List all devices:
///
/// ```
/// power::open_device_list().unwrap();
/// let mut i = 0;
/// loop {
///     match power::enum_devices(i, 0, u32::MAX) {
///         Ok(device_name) => {
///             i += 1;
///             println!("{}. device: {}", i, device_name.to_string_lossy());
///         }
///         Err(error) => {
///             println!("{:?}", error);
///             break;
///         }
///     }
/// }
/// power::close_device_list().unwrap();
/// ```
///
/// Based on the official [example code][https://learn.microsoft.com/en-us/windows/win32/power/using-the-device-power-api].
///
pub fn enum_devices(
    index: u32,
    query_interpretation_flags: u32,
    query_flags: u32,
    //filter_device_name: Option<&U16CStr>,
) -> Result<U16CString> {
    let mut buffer = [0; MAX_PATH as usize * size_of::<u16>()]; // buffer is PCWSTR in example
    let mut buffer_size = MAX_PATH * size_of::<u16>() as u32;
    call_BOOL! {
        DevicePowerEnumDevices(
            index,
            query_interpretation_flags,
            query_flags,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) return Error
    };
    // Safety: `buffer` is valid for `get_string_len_from_bytes` elements.
    //          Result of `get_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

pub fn set_device_state(id: &U16CStr, flags: u32) -> Result<u32> {
    call! {
        DevicePowerSetDeviceState(id.as_ptr(), flags, ptr::null()) != 0
    }
}

pub fn get_current_power_policies() -> Result<(GLOBAL_POWER_POLICY, POWER_POLICY)> {
    call_BOOL! {
         GetCurrentPowerPolicies(
            &mut global,
            &mut active,
        ) -> mut (global, active) = default_sized!((GLOBAL_POWER_POLICY, POWER_POLICY))
    }
}

/// return value is `BOOL`
/// if `false` the device is in low-power state
pub fn get_device_power_state(device_handle: isize) -> Result<i32> {
    call_BOOL! {
        GetDevicePowerState(device_handle, &mut power_state) -> mut power_state: i32
    }
}

pub fn get_system_power_status() -> Result<SYSTEM_POWER_STATUS> {
    call_BOOL! {
        GetSystemPowerStatus(
            &mut power_status
        ) -> mut power_status = default_sized!(SYSTEM_POWER_STATUS)
    }
}

/// docs ???
pub fn is_admin_override_active(admin_power_policy: ADMINISTRATOR_POWER_POLICY) -> Result<bool> {
    call_BOOL! { IsAdminOverrideActive(&admin_power_policy) -> bool }
}

pub fn is_system_resume_automatic() -> Result<bool> {
    call_BOOL! { IsSystemResumeAutomatic() -> bool }
}

pub fn can_restore_default_power_scheme(scheme: GUID) -> Result<()> {
    call_WIN32_ERROR! { PowerCanRestoreIndividualDefaultPowerScheme(&scheme) }
}

/// check docs for request types
///
/// https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-powerclearrequest
///
pub fn clear_power_request(request_handle: isize, request_type: i32) -> Result<()> {
    call_BOOL! { PowerClearRequest(request_handle, request_type) }
}

pub fn create_possible_power_setting(
    sub_group: GUID,
    power_setting: GUID,
    possible_index: u32,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerCreatePossibleSetting(
            0,
            &sub_group,
            &power_setting,
            possible_index,
        )
    }
}

/// Power request object must be freed with [`close_handle`][crate::win32::foundation::close_handle].
pub fn create_power_request(context: REASON_CONTEXT) -> Result<isize> {
    call! { PowerCreateRequest(&context) != 0 }
}

pub fn create_power_setting(sub_group: GUID, power_setting: GUID) -> Result<()> {
    call_WIN32_ERROR! {
        PowerCreateSetting(0, &sub_group, &power_setting)
    }
}

pub fn delete_scheme(scheme: GUID) -> Result<()> {
    call_WIN32_ERROR! { PowerDeleteScheme(0, &scheme) }
}

/// in `POWER_PLATFORM_ROLE_VERSION` out `POWER_PLATFORM_ROLE`
/// win10+
pub fn get_platform_role(version: u32) -> i32 {
    unsafe { PowerDeterminePlatformRoleEx(version) }
}

pub fn duplicate_scheme(scheme: GUID) -> Result<GUID> {
    let mut dup_scheme_ptr = ptr::null_mut::<GUID>();
    call_WIN32_ERROR! {
        PowerDuplicateScheme(
            0,
            &scheme,
            addr_of_mut!(dup_scheme_ptr),
        ) return Error
    };
    // Safety: `dup_scheme_ptr` is valid and initialized with a `GUID` by `PowerDuplicateScheme`.
    let dup_scheme_guid = unsafe { dup_scheme_ptr.read() };
    free!(Local: dup_scheme_ptr);
    Ok(dup_scheme_guid)
}

/// POWER_DATA_ACCESSOR
pub fn enum_schemes(index: u32) -> Result<Option<GUID>> {
    let mut buffer = [0_u8; 16];
    call_WIN32_ERROR! {
        PowerEnumerate(
            0,
            ptr::null(),
            ptr::null(),
            ACCESS_SCHEME,
            index,
            buffer.as_mut_ptr(),
            &mut 16,
        ) != ERROR_NO_MORE_ITEMS => None
            else return Error;
    };
    Ok(Some(guid_from_array(buffer)))
}

pub fn enum_sub_groups(index: u32, scheme: GUID) -> Result<Option<GUID>> {
    let mut buffer = [0_u8; 16];
    call_WIN32_ERROR! {
        PowerEnumerate(
            0,
            &scheme,
            ptr::null(),
            ACCESS_SUBGROUP,
            index,
            buffer.as_mut_ptr(),
            &mut 16,
        ) != ERROR_NO_MORE_ITEMS => None
            else return Error;
    };
    Ok(Some(guid_from_array(buffer)))
}

// pub enum PowerSchemeSubGroup {}

pub fn enum_settings(index: u32, scheme: GUID, sub_group: GUID) -> Result<Option<GUID>> {
    let mut buffer = [0_u8; 16];
    call_WIN32_ERROR! {
        PowerEnumerate(
            0,
            &scheme,
            &sub_group,
            ACCESS_INDIVIDUAL_SETTING,
            index,
            buffer.as_mut_ptr(),
            &mut 16,
        ) != ERROR_NO_MORE_ITEMS => None
            else return Error;
    };
    Ok(Some(guid_from_array(buffer)))
}

pub fn get_active_scheme() -> Result<GUID> {
    let mut scheme_ptr = ptr::null_mut::<GUID>();
    call_WIN32_ERROR! {
        PowerGetActiveScheme(0, addr_of_mut!(scheme_ptr)) return Error
    };
    // Safety: `dup_scheme_ptr` is valid and initialized with a `GUID` by `PowerDuplicateScheme`.
    let scheme_guid = unsafe { scheme_ptr.read() };
    free!(Local: scheme_ptr);
    Ok(scheme_guid)
}

pub fn import_scheme(file_path: &U16CStr) -> Result<GUID> {
    let mut new_scheme_guid_ptr = ptr::null_mut::<GUID>();
    call_WIN32_ERROR! {
        PowerImportPowerScheme(
            0,
            file_path.as_ptr(),
            addr_of_mut!(new_scheme_guid_ptr),
        ) return Error
    }
    // Safety: `dup_scheme_ptr` is valid and initialized with a `GUID` by `PowerDuplicateScheme`.
    let new_scheme_guid = unsafe { new_scheme_guid_ptr.read() };
    free!(Local: new_scheme_guid_ptr);
    Ok(new_scheme_guid)
}

pub fn is_setting_range_defined(sub_group: GUID, setting: GUID) -> Result<bool> {
    call_BOOL! {
        PowerIsSettingRangeDefined(
            &sub_group,
            &setting,
        ) -> bool
    }
}

/// undocumented
pub fn open_system_power_key(access: u32, open_existing: bool) -> Result<isize> {
    call! {
        PowerOpenSystemPowerKey(
            &mut power_key_handle,
            access,
            to_BOOL!(open_existing),
        ) != 0 /* or == 0 ??? */ => mut power_key_handle: isize
    }
}

/// undocumented
pub fn open_user_power_key(access: u32, open_existing: bool) -> Result<isize> {
    call! {
        PowerOpenUserPowerKey(
            &mut power_key_handle,
            access,
            to_BOOL!(open_existing),
        ) != 0 /* or == 0 ??? */ => mut power_key_handle: isize
    }
}

pub fn read_ac_default_index(
    scheme_personality: GUID,
    sub_group: GUID,
    setting: GUID,
) -> Result<u32> {
    call! {
        PowerReadACDefaultIndex(
            0,
            &scheme_personality,
            &sub_group,
            &setting,
            &mut default_index,
        ) == 0 => mut default_index: u32
    }
}

pub fn read_ac_value_size(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadACValue(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut 0,
            ptr::null_mut(),
            &mut size,
        ) == 0 => mut size: u32
    }
}

pub fn read_ac_value(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<RegistryValue> {
    let mut buffer_size = read_ac_value_size(scheme, sub_group, setting)?;
    let buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadACValue(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut reg_value.value_type,
            reg_value.data.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut reg_value = RegistryValue { data: buffer.into_boxed_slice(), value_type: 0 }
    }
}

pub fn read_ac_value_with_buffer(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    buffer: &mut [u8],
) -> Result<(u32, u32)> {
    call! {
        PowerReadACValue(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut reg_value_type,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut (reg_value_type, buffer_size) = (0, buffer.len() as u32)
    }
}

pub fn read_ac_value_index(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadACValueIndex(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut ac_value_index,
        ) == 0 => mut ac_value_index: u32
    }
}

pub fn read_dc_default_index(
    scheme_personality: GUID,
    sub_group: GUID,
    setting: GUID,
) -> Result<u32> {
    call! {
        PowerReadDCDefaultIndex(
            0,
            &scheme_personality,
            &sub_group,
            &setting,
            &mut default_index,
        ) == 0 => mut default_index: u32
    }
}

pub fn read_dc_value_size(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadDCValue(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut 0,
            ptr::null_mut(),
            &mut size,
        ) == 0 => mut size: u32
    }
}

pub fn read_dc_value(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<RegistryValue> {
    let mut buffer_size = read_dc_value_size(scheme, sub_group, setting)?;
    let buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadDCValue(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut reg_value.value_type,
            reg_value.data.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut reg_value = RegistryValue {
            data: buffer.into_boxed_slice(),
            value_type: 0
        }
    }
}

pub fn read_dc_value_with_buffer(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    buffer: &mut [u8],
) -> Result<(u32, u32)> {
    call! {
        PowerReadDCValue(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut reg_value_type,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut (reg_value_type, buffer_size) = (0, buffer.len() as u32)
    }
}

pub fn read_dc_value_index(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadDCValueIndex(
            0,
            &scheme,
            &sub_group,
            &setting,
            &mut ac_value_index,
        ) == 0 => mut ac_value_index: u32
    }
}

pub fn read_description_size(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadDescription(
            0,
            &scheme,
            &sub_group,
            &setting,
            ptr::null_mut(),
            &mut needed_size,
        ) == 0 => mut needed_size: u32
    }
}

pub fn read_description(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<U16CString> {
    let mut buffer_size = read_description_size(scheme, sub_group, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadDescription(
            0,
            &scheme,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => return Error
    }
    // Safety: `buffer` is valid for `get_string_len_from_bytes` elements.
    //          Result of `get_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

pub fn read_description_with_buffer(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    buffer: &mut [u8],
) -> Result<u32> {
    call! {
        PowerReadDescription(
            0,
            &scheme,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut buffer_size = buffer.len() as u32
    }
}

pub fn read_friendly_name_size(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadFriendlyName(
            0,
            &scheme,
            &sub_group,
            &setting,
            ptr::null_mut(),
            &mut needed_size,
        ) == 0 => mut needed_size: u32
    }
}

pub fn read_friendly_name(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<U16CString> {
    let mut buffer_size = read_friendly_name_size(scheme, sub_group, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadFriendlyName(
            0,
            &scheme,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => return Error
    }
    // Safety: `buffer` is valid for `get_string_len_from_bytes` elements.
    //          Result of `get_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

pub fn read_friendly_name_with_buffer(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    buffer: &mut [u8],
) -> Result<u32> {
    call! {
        PowerReadFriendlyName(
            0,
            &scheme,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut buffer_size = buffer.len() as u32
    }
}

pub fn read_icon_resource_name_size(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadIconResourceSpecifier(
            0,
            &scheme,
            &sub_group,
            &setting,
            ptr::null_mut(),
            &mut needed_size,
        ) == 0 => mut needed_size: u32
    }
}

pub fn read_icon_resource_name(scheme: GUID, sub_group: GUID, setting: GUID) -> Result<U16CString> {
    let mut buffer_size = read_icon_resource_name_size(scheme, sub_group, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadIconResourceSpecifier(
            0,
            &scheme,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => return Error
    }
    // Safety: `buffer` is valid for `get_string_len_from_bytes` elements.
    //          Result of `get_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

pub fn read_icon_resource_with_buffer(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    buffer: &mut [u8],
) -> Result<u32> {
    call! {
        PowerReadIconResourceSpecifier(
            0,
            &scheme,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut buffer_size = buffer.len() as u32
    }
}

pub fn read_possible_description_size(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
) -> Result<u32> {
    call! {
        PowerReadPossibleDescription(
            0,
            &scheme,
            &sub_group,
            setting_index,
            ptr::null_mut(),
            &mut needed_size,
        ) == 0 => mut needed_size: u32
    }
}

pub fn read_possible_description(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
) -> Result<U16CString> {
    let mut buffer_size = read_possible_description_size(scheme, sub_group, setting_index)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadPossibleDescription(
            0,
            &scheme,
            &sub_group,
            setting_index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => return Error
    }
    // Safety: `buffer` is valid for `get_string_len_from_bytes` elements.
    //          Result of `get_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

pub fn read_possible_description_with_buffer(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
    buffer: &mut [u8],
) -> Result<u32> {
    call! {
        PowerReadPossibleDescription(
            0,
            &scheme,
            &sub_group,
            setting_index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut buffer_size = buffer.len() as u32
    }
}

pub fn read_possible_friendly_name_size(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
) -> Result<u32> {
    call! {
        PowerReadPossibleFriendlyName(
            0,
            &scheme,
            &sub_group,
            setting_index,
            ptr::null_mut(),
            &mut needed_size,
        ) == 0 => mut needed_size: u32
    }
}

pub fn read_possible_friendly_name(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
) -> Result<U16CString> {
    let mut buffer_size = read_possible_friendly_name_size(scheme, sub_group, setting_index)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadPossibleFriendlyName(
            0,
            &scheme,
            &sub_group,
            setting_index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => return Error
    }
    // Safety: `buffer` is valid for `get_string_len_from_bytes` elements.
    //          Result of `get_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

pub fn read_possible_friendly_name_with_buffer(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
    buffer: &mut [u8],
) -> Result<u32> {
    call! {
        PowerReadPossibleFriendlyName(
            0,
            &scheme,
            &sub_group,
            setting_index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut buffer_size = buffer.len() as u32
    }
}

pub fn read_possible_value_size(sub_group: GUID, setting: GUID, setting_index: u32) -> Result<u32> {
    call! {
        PowerReadPossibleValue(
            0,
            &sub_group,
            &setting,
            &mut 0,
            setting_index,
            ptr::null_mut(),
            &mut size,
        ) == 0 => mut size: u32
    }
}

pub fn read_possible_value(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    setting_index: u32,
) -> Result<RegistryValue> {
    let mut buffer_size = read_possible_value_size(scheme, sub_group, setting_index)?;
    let buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadPossibleValue(
            0,
            &sub_group,
            &setting,
            &mut reg_value.value_type,
            setting_index,
            reg_value.data.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut reg_value = RegistryValue {
            data: buffer.into_boxed_slice(),
            value_type: 0
        }
    }
}

pub fn read_possible_value_with_buffer(
    sub_group: GUID,
    setting: GUID,
    setting_index: u32,
    buffer: &mut [u8],
) -> Result<(u32, u32)> {
    call! {
        PowerReadPossibleValue(
            0,
            &sub_group,
            &setting,
            &mut reg_value_type,
            setting_index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => mut (reg_value_type, buffer_size) = (0, buffer.len() as u32)
    }
}

// error ???
pub fn read_setting_attributes(sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadSettingAttributes(
            &sub_group,
            &setting,
        ) != 0
    }
}

pub fn read_value_increment(sub_group: GUID, setting: GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadValueIncrement(
            0,
            &sub_group,
            &setting,
            &mut value_increment,
        ) -> mut value_increment: u32
    }
}

pub fn read_max_value(sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadValueMax(
            0,
            &sub_group,
            &setting,
            &mut max_value,
        ) != 0 => mut max_value: u32
    }
}

pub fn read_min_value(sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadValueMin(
            0,
            &sub_group,
            &setting,
            &mut min_value,
        ) != 0 => mut min_value: u32
    }
}

pub fn read_value_unit_size(sub_group: GUID, setting: GUID) -> Result<u32> {
    call! {
        PowerReadValueUnitsSpecifier(
            0,
            &sub_group,
            &setting,
            ptr::null_mut(),
            &mut buffer_size,
        ) != 0 => mut buffer_size: u32
    }
}

pub fn read_value_unit(sub_group: GUID, setting: GUID) -> Result<U16CString> {
    let mut buffer_size = read_value_unit_size(sub_group, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadValueUnitsSpecifier(
            0,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0 => return Error
    };
    // Safety: `buffer` is valid for `get_string_len_from_bytes` elements.
    //          Result of `get_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

pub fn read_value_unit_with_buffer(
    sub_group: GUID,
    setting: GUID,
    buffer: &mut [u8],
) -> Result<u32> {
    call! {
        PowerReadValueUnitsSpecifier(
            0,
            &sub_group,
            &setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) != 0 => mut buffer_size = buffer.len() as u32
    }
}

pub type EffectivePowerModeCallback = unsafe extern "system" fn(
    mode: i32, // EFFECTIVE_POWER_MODE
    context: *const ::core::ffi::c_void,
) -> ();

/// versions: EFFECTIVE_POWER_MODE_V1, EFFECTIVE_POWER_MODE_V2
/// min build: Win10 1809
/// TODO: how to use it?
pub fn register_for_effective_power_mode_notifications(
    version: u32,
    callback: EffectivePowerModeCallback,
    context: *const c_void, // TODO: safer type instead of ptr (&cvoid instead? or as Option? can be null ptr?)
) -> Result<*mut c_void> {
    call_HRESULT! {
        PowerRegisterForEffectivePowerModeNotifications(
            version,
            Some(callback),
            context,
            addr_of_mut!(handle), // TODO: find out handle type (isize?)
        ) -> mut handle = ptr::null_mut()
    }
}

/// registration_flags must be [`DEVICE_NOTIFY_CALLBACK`][windows_sys::Win32::UI::WindowsAndMessaging::DEVICE_NOTIFY_CALLBACK]
pub fn power_register_suspend_resume_notification(recipient_handle: isize) -> Result<*mut c_void> {
    call_WIN32_ERROR! {
         PowerRegisterSuspendResumeNotification(
            2,
            recipient_handle,
            addr_of_mut!(registration_handle),
         ) -> mut registration_handle = ptr::null_mut()
    }
}

pub fn remove_setting(sub_group: GUID, setting: GUID) -> Result<()> {
    call_WIN32_ERROR! {
        PowerRemovePowerSetting(
            &sub_group,
            &setting
        )
    }
}

/// The caller must be a member of the local Administrators group.
pub fn replace_default_schemes() -> Result<()> {
    call! { PowerReplaceDefaultPowerSchemes() == 0 }
}

pub fn report_thermal_event(thermal_event: THERMAL_EVENT) -> Result<()> {
    call_WIN32_ERROR! { PowerReportThermalEvent(&thermal_event) }
}

pub fn restore_default_power_schemes() -> Result<()> {
    call_WIN32_ERROR! { PowerRestoreDefaultPowerSchemes() }
}

pub fn set_active_scheme(scheme: GUID) -> Result<()> {
    call_WIN32_ERROR! { PowerSetActiveScheme(0, &scheme) }
}

pub fn set_request(power_request_handle: isize, request_type: i32) -> Result<()> {
    call_BOOL! { PowerSetRequest(power_request_handle, request_type) }
}

pub fn setting_access_check(access: i32, setting: GUID, access_type: u32) -> Result<()> {
    call_WIN32_ERROR! { PowerSettingAccessCheckEx(access, &setting, access_type) }
}

pub fn setting_register_notification(
    setting: GUID,
    register_notifcation_flags: u32,
    recipient_handle: isize,
) -> Result<*mut c_void> {
    call_WIN32_ERROR! {
        PowerSettingRegisterNotification(
            &setting,
            register_notifcation_flags,
            recipient_handle,
            addr_of_mut!(registration_handle), // windows_sys::Win32::System::Power::HPOWERNOTIFY ???
        ) -> mut registration_handle = ptr::null_mut()
    }
}

/// registration_handle: [`HPOWERNOTIFY`][windows_sys::Win32::System::Power::HPOWERNOTIFY]
pub fn setting_unregister_notification(registration_handle: isize) -> Result<()> {
    call_WIN32_ERROR! { PowerSettingUnregisterNotification(registration_handle) }
}

pub fn unregister_from_effective_power_mode_notifications(
    registration_handle: *const c_void,
) -> Result<()> {
    call_HRESULT! {
        PowerUnregisterFromEffectivePowerModeNotifications(registration_handle)
    }
}

pub fn power_unregister_suspend_resume_notification(registration_handle: isize) -> Result<()> {
    call_WIN32_ERROR! {
        PowerUnregisterSuspendResumeNotification(registration_handle)
    }
}

pub fn write_ac_default_index(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    default_index: u32,
) -> Result<()> {
    call! {
        PowerWriteACDefaultIndex(
            0,
            &scheme,
            &sub_group,
            &setting,
            default_index,
        ) == 0
    }
}

pub fn write_ac_value_index(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    value_index: u32,
) -> Result<()> {
    call! {
        PowerWriteACValueIndex(
            0,
            &scheme,
            &sub_group,
            &setting,
            value_index,
        ) == 0
    }
}

pub fn write_dc_default_index(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    default_index: u32,
) -> Result<()> {
    call! {
        PowerWriteDCDefaultIndex(
            0,
            &scheme,
            &sub_group,
            &setting,
            default_index,
        ) == 0
    }
}

pub fn write_dc_value_index(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    value_index: u32,
) -> Result<()> {
    call! {
        PowerWriteDCValueIndex(
            0,
            &scheme,
            &sub_group,
            &setting,
            value_index,
        ) == 0
    }
}

pub fn write_description(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    description: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteDescription(
            0,
            &scheme,
            &sub_group,
            &setting,
            description.as_ptr().cast(),
            description.len() as u32,
        )
    }
}

pub fn write_friendly_name(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    friendly_name: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteFriendlyName(
            0,
            &scheme,
            &sub_group,
            &setting,
            friendly_name.as_ptr().cast(),
            friendly_name.len() as u32,
        )
    }
}

pub fn write_icon_resource(
    scheme: GUID,
    sub_group: GUID,
    setting: GUID,
    icon_resource: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteIconResourceSpecifier(
            0,
            &scheme,
            &sub_group,
            &setting,
            icon_resource.as_ptr().cast(),
            icon_resource.len() as u32,
        )
    }
}

pub fn write_possible_description(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
    description: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWritePossibleDescription(
            0,
            &scheme,
            &sub_group,
            setting_index,
            description.as_ptr().cast(),
            description.len() as u32,
        )
    }
}

pub fn write_possible_friendly_name(
    scheme: GUID,
    sub_group: GUID,
    setting_index: u32,
    friendly_name: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWritePossibleFriendlyName(
            0,
            &scheme,
            &sub_group,
            setting_index,
            friendly_name.as_ptr().cast(),
            friendly_name.len() as u32,
        )
    }
}

pub fn write_possible_value(
    sub_group: GUID,
    setting: GUID,
    value_type: u32,
    setting_index: u32,
    value: &[u8],
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWritePossibleValue(
            0,
            &sub_group,
            &setting,
            value_type,
            setting_index,
            value.as_ptr(),
            value.len() as u32,
        )
    }
}

pub fn write_value_increment(sub_group: GUID, setting: GUID, value_increment: u32) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueIncrement(
            0,
            &sub_group,
            &setting,
            value_increment,
        )
    }
}

pub fn write_value_max(sub_group: GUID, setting: GUID, max_value: u32) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueMax(
            0,
            &sub_group,
            &setting,
            max_value,
        )
    }
}

pub fn write_value_min(sub_group: GUID, setting: GUID, min_value: u32) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueMin(
            0,
            &sub_group,
            &setting,
            min_value,
        )
    }
}

pub fn write_value_unit(sub_group: GUID, setting: GUID, unit: &U16CStr) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueUnitsSpecifier(
            0,
            &sub_group,
            &setting,
            unit.as_ptr().cast(),
            unit.len() as u32,
        )
    }
}

/// For interactive applications, the `flags` parameter should be `0`, and the `window_handle` parameter should be a window handle.
///
/// For services, the `flags` parameter should be `1`, and the `window_handle` parameter should be a SERVICE_STATUS_HANDLE
pub fn register_setting_notification(
    window_handle: isize,
    setting: GUID,
    flags: u32,
) -> Result<isize> {
    call! {
        RegisterPowerSettingNotification(
            window_handle,
            &setting,
            flags,
        ) != 0
    }
}

/// Similar to [`power_register_suspend_resume_notification`], but operates in user mode and can take a window handle.
pub fn register_suspend_resume_notification(window_handle: isize, flags: u32) -> Result<isize> {
    call! {
        RegisterSuspendResumeNotification(
            window_handle,
            flags,
        ) != 0
    }
}

pub fn request_wake_up_latency(latency: i32) -> Result<()> {
    call_BOOL! { RequestWakeupLatency(latency) }
}

pub fn set_susoend_state(hibernate: bool, disable_wake_events: bool) -> Result<()> {
    call_BOOL! {
        SetSuspendState(
            to_BOOL!(hibernate),
            to_BOOL!(false), // has no effect
            to_BOOL!(disable_wake_events)
        )
    }
}

pub fn set_system_power_state(suspend: bool) -> Result<()> {
    // 2. param has no effect
    call_BOOL! { SetSystemPowerState(to_BOOL!(suspend), to_BOOL!(false)) }
}

pub fn set_thread_execution_state(flags: u32) -> Result<u32> {
    call! { SetThreadExecutionState(flags) != 0 }
}

pub fn unregister_setting_notification(registration_handle: isize) -> Result<()> {
    call_BOOL! { UnregisterPowerSettingNotification(registration_handle) }
}

/// same as [`power_unregister_suspend_resume_notification`], but operates in user mode
pub fn unregister_suspend_resume_notification(registration_handle: isize) -> Result<()> {
    call_BOOL! { UnregisterSuspendResumeNotification(registration_handle) }
}
