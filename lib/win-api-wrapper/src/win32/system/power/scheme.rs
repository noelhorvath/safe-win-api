use crate::core::{get_wide_string_len_from_bytes, guid_from_array, Result, GUID};
use crate::win32::system::registry::RegistryValue;
use crate::{call, call_WIN32_ERROR, free, from_BOOL};
use core::ptr::{self, addr_of_mut};
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_NO_MORE_ITEMS};
use windows_sys::Win32::System::Power::{
    PowerCanRestoreIndividualDefaultPowerScheme, PowerCreatePossibleSetting, PowerCreateSetting,
    PowerDeleteScheme, PowerDuplicateScheme, PowerEnumerate, PowerGetActiveScheme,
    PowerImportPowerScheme, PowerIsSettingRangeDefined, PowerReadACDefaultIndex, PowerReadACValue,
    PowerReadACValueIndex, PowerReadDCDefaultIndex, PowerReadDCValue, PowerReadDCValueIndex,
    PowerReadDescription, PowerReadFriendlyName, PowerReadIconResourceSpecifier,
    PowerReadPossibleDescription, PowerReadPossibleFriendlyName, PowerReadPossibleValue,
    PowerReadSettingAttributes, PowerReadValueIncrement, PowerReadValueMax, PowerReadValueMin,
    PowerReadValueUnitsSpecifier, PowerRemovePowerSetting, PowerReplaceDefaultPowerSchemes,
    PowerRestoreDefaultPowerSchemes, PowerSetActiveScheme, PowerSettingAccessCheckEx,
    PowerWriteACDefaultIndex, PowerWriteACValueIndex, PowerWriteDCDefaultIndex,
    PowerWriteDCValueIndex, PowerWriteDescription, PowerWriteFriendlyName,
    PowerWriteIconResourceSpecifier, PowerWritePossibleDescription, PowerWritePossibleFriendlyName,
    PowerWritePossibleValue, PowerWriteSettingAttributes, PowerWriteValueIncrement,
    PowerWriteValueMax, PowerWriteValueMin, PowerWriteValueUnitsSpecifier,
};

pub use windows_sys::Win32::System::Power::{
    ACCESS_ACTIVE_OVERLAY_SCHEME, ACCESS_ACTIVE_SCHEME, ACCESS_AC_POWER_SETTING_INDEX,
    ACCESS_AC_POWER_SETTING_MAX, ACCESS_AC_POWER_SETTING_MIN, ACCESS_ATTRIBUTES,
    ACCESS_CREATE_SCHEME, ACCESS_DC_POWER_SETTING_INDEX, ACCESS_DC_POWER_SETTING_MAX,
    ACCESS_DC_POWER_SETTING_MIN, ACCESS_DEFAULT_AC_POWER_SETTING, ACCESS_DEFAULT_DC_POWER_SETTING,
    ACCESS_DEFAULT_SECURITY_DESCRIPTOR, ACCESS_DESCRIPTION, ACCESS_FRIENDLY_NAME,
    ACCESS_ICON_RESOURCE, ACCESS_INDIVIDUAL_SETTING, ACCESS_OVERLAY_SCHEME,
    ACCESS_POSSIBLE_POWER_SETTING, ACCESS_POSSIBLE_POWER_SETTING_DESCRIPTION,
    ACCESS_POSSIBLE_POWER_SETTING_FRIENDLY_NAME, ACCESS_POSSIBLE_VALUE_INCREMENT,
    ACCESS_POSSIBLE_VALUE_MAX, ACCESS_POSSIBLE_VALUE_MIN, ACCESS_POSSIBLE_VALUE_UNITS,
    ACCESS_PROFILE, ACCESS_SCHEME, ACCESS_SUBGROUP, POWER_ATTRIBUTE_HIDE,
    POWER_ATTRIBUTE_SHOW_AOAC,
};

pub use windows_sys::Win32::System::SystemServices::{
    GUID_ADAPTIVE_POWER_BEHAVIOR_SUBGROUP, GUID_BATTERY_SUBGROUP, GUID_DISK_SUBGROUP,
    GUID_ENERGY_SAVER_SUBGROUP, GUID_GRAPHICS_SUBGROUP, GUID_IDLE_RESILIENCY_SUBGROUP,
    GUID_INTSTEER_SUBGROUP, GUID_PCIEXPRESS_SETTINGS_SUBGROUP, GUID_PROCESSOR_SETTINGS_SUBGROUP,
    GUID_SLEEP_SUBGROUP, GUID_SYSTEM_BUTTON_SUBGROUP, NO_SUBGROUP_GUID,
};

/// Determines if the current user has access to the data for the power scheme specified by `scheme`
/// so that it could be restored if necessary.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powercanrestoreindividualdefaultpowerscheme
///
pub fn can_restore_default_power_scheme(scheme: &GUID) -> Result<()> {
    call_WIN32_ERROR! { PowerCanRestoreIndividualDefaultPowerScheme(scheme) }
}

/// Creates a possible setting value for a specified power setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting being created.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powercreatepossiblesetting
///
pub fn create_possible_power_setting(subgroup: &GUID, setting: &GUID, index: u32) -> Result<()> {
    call_WIN32_ERROR! {
        PowerCreatePossibleSetting(
            0,
            subgroup,
            setting,
            index,
        )
    }
}

/// Creates a setting value for a specified power setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
/// * `setting`: The identifier of the power setting that is being used.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powercreatesetting
///
pub fn create_power_setting(subgroup: &GUID, setting: &GUID) -> Result<()> {
    call_WIN32_ERROR! {
        PowerCreateSetting(0, subgroup, setting)
    }
}

/// Deletes the specified power scheme from the database.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerdeletescheme
///
pub fn delete(scheme: &GUID) -> Result<()> {
    call_WIN32_ERROR! { PowerDeleteScheme(0, scheme) }
}

/// Duplicates an existing power scheme.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerduplicatescheme
///
pub fn duplicate(scheme: &GUID) -> Result<GUID> {
    let mut dup_scheme_ptr = ptr::null_mut::<GUID>();
    call_WIN32_ERROR! {
        PowerDuplicateScheme(
            0,
            scheme,
            addr_of_mut!(dup_scheme_ptr),
        ) return Error
    };
    // Safety: `dup_scheme_ptr` is valid and initialized with a `GUID` by `PowerDuplicateScheme`.
    let dup_scheme_guid = unsafe { dup_scheme_ptr.read() };
    free!(Local: dup_scheme_ptr);
    Ok(dup_scheme_guid)
}

/// Enumerates the power schemes in the system. This function is normally called in a
/// loop incrementing the `index` parameter to retrieve schemes until they have all been
/// enumerated (`None`).
///
/// If the function succeeds and there is no pwoer scheme found at the specified index,
/// the return value is `None`.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerenumerate
///
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
    Ok(Some(guid_from_array(&buffer)))
}

/// Enumerates the subgroups under the `scheme`. This function is normally called in a
/// loop incrementing the `index` parameter to retrieve subgroups until they have all been
/// enumerated (`None`).
///
/// If the function succeeds and there is no subgroup found at the specified index,
/// the return value is `None`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_INVALID_HANDLE`][windows_sys::Win32::Foundation::ERROR_INVALID_HANDLE]
///     * The specified power scheme does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerenumerate
///
pub fn enum_subgroups(index: u32, scheme: &GUID) -> Result<Option<GUID>> {
    let mut buffer = [0_u8; 16];
    call_WIN32_ERROR! {
        PowerEnumerate(
            0,
            scheme,
            ptr::null(),
            ACCESS_SUBGROUP,
            index,
            buffer.as_mut_ptr(),
            &mut 16,
        ) != ERROR_NO_MORE_ITEMS => None
            else return Error;
    };
    Ok(Some(guid_from_array(&buffer)))
}

/// Enumerates the settings under the specified scheme or subhroup. This function is normally called in a
/// loop incrementing the `index` parameter to retrieve settings until they have all been
/// enumerated (`None`).
///
/// If the function succeeds and there is no power setting found at the specified index,
/// the return value is `None`.
///
/// To enumerate power settings directly under `scheme`, use [`NO_SUBGROUP_GUID`] as
/// the `subgroup` parameter.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_INVALID_HANDLE`][windows_sys::Win32::Foundation::ERROR_INVALID_HANDLE]
///     * The specified power scheme or subgroup does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerenumerate
///
pub fn enum_settings(index: u32, scheme: &GUID, subgroup: &GUID) -> Result<Option<GUID>> {
    let mut buffer = [0_u8; 16];
    call_WIN32_ERROR! {
        PowerEnumerate(
            0,
            scheme,
            subgroup,
            ACCESS_INDIVIDUAL_SETTING,
            index,
            buffer.as_mut_ptr(),
            &mut 16,
        ) != ERROR_NO_MORE_ITEMS => None
            else return Error;
    };
    Ok(Some(guid_from_array(&buffer)))
}

/// Retrieves the active power scheme and returns a [`GUID`] that identifies the scheme.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powergetactivescheme
///
pub fn get_active() -> Result<GUID> {
    let mut scheme_ptr = ptr::null_mut::<GUID>();
    call_WIN32_ERROR! {
        PowerGetActiveScheme(0, addr_of_mut!(scheme_ptr)) return Error
    };
    // Safety: `dup_scheme_ptr` is valid and initialized with a `GUID` by `PowerDuplicateScheme`.
    let scheme_guid = unsafe { scheme_ptr.read() };
    free!(Local: scheme_ptr);
    Ok(scheme_guid)
}

/// Imports a power scheme from a file returns its [`GUID`].
///
/// If the specified file does not exist or represent a valid power scheme backup,
/// the function succeeds and returns the same [`GUID`] that was passed in.
///
/// # Arguments
///
/// * `file_path`: The path to a power scheme backup file created by `PowerCfg.Exe /Export`.
/// * `guid`: An optional [`GUID`] that is assigned to the imported power scheme.
///     * If this parameter is `None`, a new [`GUID`] is generated for the imported
///       power scheme.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerimportpowerscheme
///
pub fn import(file_path: &U16CStr, guid: Option<&GUID>) -> Result<GUID> {
    let mut scheme_guid_ptr = guid.map_or(ptr::null_mut(), |guid| guid as *const _ as *mut _);
    call_WIN32_ERROR! {
        PowerImportPowerScheme(
            0,
            file_path.as_ptr(),
            addr_of_mut!(scheme_guid_ptr),
        ) return Error
    };
    match guid {
        None => {
            // Safety: `dup_scheme_ptr` is valid and initialized with a `GUID` by `PowerDuplicateScheme`.
            let generated_scheme_guid = unsafe { scheme_guid_ptr.read() };
            free!(Local: scheme_guid_ptr);
            Ok(generated_scheme_guid)
        }
        Some(&guid) => Ok(guid),
    }
}

/// Determines whether the specified power setting represents a range of
/// possible values.
///
/// If the specified subgroup or power setting does not exist, the return
/// value is `false`.
///
/// # Arguments
///
/// * `subgroup`: The identifier of the subkey to search.
/// * `setting`: The identifier of the power setting to query.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerissettingrangedefined
///
pub fn is_range_defined(subgroup: &GUID, setting: &GUID) -> bool {
    from_BOOL!(unsafe { PowerIsSettingRangeDefined(subgroup, setting,) })
}

/*
pub fn open_system_power_key(access: u32, open_existing: bool) -> Result<isize> {
    call! {
        PowerOpenSystemPowerKey(
            &mut power_key_handle,
            access,
            to_BOOL!(open_existing),
        ) != 0 => mut power_key_handle: isize
    }
}

pub fn open_user_power_key(access: u32, open_existing: bool) -> Result<isize> {
    call! {
        PowerOpenUserPowerKey(
            &mut power_key_handle,
            access,
            to_BOOL!(open_existing),
        ) != 0 => mut power_key_handle: isize
    }
}
*/

/// Gets the default AC index of the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier for the scheme personality for this power setting.
///     * A power setting can have different default values depending on the power
///       scheme personality.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting for the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have a default AC index.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadacdefaultindex
///
pub fn read_ac_default_index(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadACDefaultIndex(
            0,
            scheme,
            subgroup,
            setting,
            &mut default_index,
        ) -> mut default_index: u32
    }
}

/// Gets the size of the AC power value in bytes for the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting`s value size for the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have an AC value.
///
/// # Examples
///
/// TODO
///
pub fn read_ac_value_size(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadACValue(
            0,
            scheme,
            subgroup,
            setting,
            &mut 0,
            ptr::null_mut(),
            &mut size,
        ) -> mut size: u32
    }
}

/// Gets the AC power value for the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting for the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have an AC value.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerreadacvalue
///
pub fn read_ac_value(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<RegistryValue> {
    let mut buffer_size = read_ac_value_size(scheme, subgroup, setting)?;
    let buffer = vec![0; buffer_size as usize];
    call_WIN32_ERROR! {
        PowerReadACValue(
            0,
            scheme,
            subgroup,
            setting,
            &mut reg_value.value_type,
            reg_value.data.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut reg_value = RegistryValue { data: buffer.into_boxed_slice(), value_type: 0 }
    }
}

/// Gets the AC power value for the specified power setting, copies its data into `buffer`
/// and returns the type and size of the value data.
///
/// If the the specified power setting does not have an AC value, the function
/// fails.
///
/// To get the size of the value's data, use [`read_ac_value_size`].
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting for the default power scheme.
/// * `setting`: The identifier for the single power setting.
/// * `buffer`: The buffer that receives the data value.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the requested data.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have an AC value.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerreadacvalue
///
pub fn read_ac_value_with_buffer(
    scheme: &GUID,
    subgroup: &GUID,
    setting: &GUID,
    buffer: &mut [u8],
) -> Result<(u32, u32)> {
    call_WIN32_ERROR! {
        PowerReadACValue(
            0,
            scheme,
            subgroup,
            setting,
            &mut reg_value_type,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut (reg_value_type, buffer_size) = (0, buffer.len() as u32)
    }
}

/// Gets the value AC index of the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have an AC value index.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadacvalueindex
///
pub fn read_ac_value_index(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadACValueIndex(
            0,
            scheme,
            subgroup,
            setting,
            &mut ac_value_index,
        ) -> mut ac_value_index: u32
    }
}

/// Gets the default DC index of the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting for the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have a default DC index.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreaddcdefaultindex
///
pub fn read_dc_default_index(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadDCDefaultIndex(
            0,
            scheme,
            subgroup,
            setting,
            &mut default_index,
        ) -> mut default_index: u32
    }
}

/// Gets the size of the DC power value in bytes for the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting's size for the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have a DC value.
///
/// # Examples
///
/// TODO
///
pub fn read_dc_value_size(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadDCValue(
            0,
            scheme,
            subgroup,
            setting,
            &mut 0,
            ptr::null_mut(),
            &mut size,
        ) -> mut size: u32
    }
}

/// Gets the DC power value for the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting for the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have a DC value.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerreaddcvalue
///
pub fn read_dc_value(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<RegistryValue> {
    let mut buffer_size = read_dc_value_size(scheme, subgroup, setting)?;
    let buffer = vec![0; buffer_size as usize];
    call_WIN32_ERROR! {
        PowerReadDCValue(
            0,
            scheme,
            subgroup,
            setting,
            &mut reg_value.value_type,
            reg_value.data.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut reg_value = RegistryValue {
            data: buffer.into_boxed_slice(),
            value_type: 0
        }
    }
}

/// Gets the DC power value for the specified power setting, copies its data into `buffer`
/// and returns the type and size of the value data.
///
/// To get the size of the value's data, use [`read_ac_value_size`].
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to retrieve the setting for the default power scheme.
/// * `setting`: The identifier for the single power setting.
/// * `buffer`: The buffer that receives the data value.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the requested data.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have a DC value.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerreaddcvalue
///
pub fn read_dc_value_with_buffer(
    scheme: &GUID,
    subgroup: &GUID,
    setting: &GUID,
    buffer: &mut [u8],
) -> Result<(u32, u32)> {
    call_WIN32_ERROR! {
        PowerReadDCValue(
            0,
            scheme,
            subgroup,
            setting,
            &mut reg_value_type,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut (reg_value_type, buffer_size) = (0, buffer.len() as u32)
    }
}

/// Gets the DC value index of the specified power setting.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power setting does not have a DC value index.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreaddcvalueindex
///
pub fn read_dc_value_index(scheme: &GUID, subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadDCValueIndex(
            0,
            scheme,
            subgroup,
            setting,
            &mut ac_value_index,
        ) -> mut ac_value_index: u32
    }
}

/// Gets the size of the description for the specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the size of the power scheme's
/// description in bytes will be returned.
/// If only `setting` is `None`, the size of the subgroup's description in bytes will be
/// returned.
/// If both `subgroup` and `setting` are identifiers, the size of the power setting's
/// description in bytes will be returned.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have a description.
///
/// # Examples
///
/// TODO
///
pub fn read_description_size(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadDescription(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            ptr::null_mut(),
            &mut needed_size,
        ) -> mut needed_size: u32
    }
}

/// Gets the the description for the specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the description of the power scheme
/// will be returned.
/// If only `setting` is `None`, the description of the subgroup will be returned.
/// If both `subgroup` and `setting` are identifiers, the description of the power
/// setting will be returned.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have a description.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreaddescription
///
pub fn read_description(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
) -> Result<U16CString> {
    let mut buffer_size = read_description_size(scheme, subgroup, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call_WIN32_ERROR! {
        PowerReadDescription(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) return Error
    }
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_wide_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

/// Gets the the description as bytes for the specified power setting, subgroup, or scheme,
/// copies it into `buffer` and returns the number of bytes copied.
///
/// Use [`read_description_size`], to get the size of the description in bytes for the
/// specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the size of the power scheme's
/// description in bytes will be returned and `buffer` will contain the description as bytes.
/// If only `setting` is `None`, the size of the subgroup's description in bytes will be
/// returned and `buffer` will contain the description as bytes.
/// If both `subgroup` and `setting` are identifiers, the size of the power setting's
/// description in bytes will be returned and `buffer` will contain the description as bytes.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
/// * `buffer`: The buffer that receives the description as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the description as bytes.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have a description.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreaddescription
///
pub fn read_description_with_buffer(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
    buffer: &mut [u8],
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadDescription(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut buffer_size = buffer.len() as u32
    }
}

/// Gets the size of the friendly name for the specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the size of the power scheme's
/// friendly name in bytes will be returned.
/// If only `setting` is `None`, the size of the subgroup's friendly name in bytes will be
/// returned.
/// If both `subgroup` and `setting` are identifiers, the size of the power setting's
/// friendly name in bytes will be returned.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have a friendly name.
///
/// # Examples
///
/// TODO
///
pub fn read_friendly_name_size(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadFriendlyName(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            ptr::null_mut(),
            &mut needed_size,
        ) -> mut needed_size: u32
    }
}

/// Gets the the friendly name for the specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the friendly name of the power scheme
/// will be returned.
/// If only `setting` is `None`, the friendly name of the subgroup will be returned.
/// If both `subgroup` and `setting` are identifiers, the friendly name of the power
/// setting will be returned.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have a friendly name.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadfriendlyname
///
pub fn read_friendly_name(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
) -> Result<U16CString> {
    let mut buffer_size = read_friendly_name_size(scheme, subgroup, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call_WIN32_ERROR! {
        PowerReadFriendlyName(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) return Error
    }
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_wide_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

/// Gets the the friendly name as bytes for the specified power setting, subgroup, or scheme,
/// copies it into `buffer` and returns the number of bytes copied.
///
/// Use [`read_friendly_name_size`], to get the size of the friendly name in bytes for the
/// specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the size of the power scheme's
/// friendly name in bytes will be returned and `buffer` will contain the friendly name
/// as bytes.
/// If only `setting` is `None`, the size of the subgroup's friendly name in bytes will be
/// returned and `buffer` will contain the friendly name as bytes.
/// If both `subgroup` and `setting` are identifiers, the size of the power setting's
/// friendly name in bytes will be returned and `buffer` will contain the friendly name
/// as bytes.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
/// * `buffer`: The buffer that receives the friendly name as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the friendly name as bytes.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have a friendly name.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadfriendlyname
///
pub fn read_friendly_name_with_buffer(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
    buffer: &mut [u8],
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadFriendlyName(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut buffer_size = buffer.len() as u32
    }
}

/// Gets the size of the icon resource for the specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the size of the power scheme's
/// icon resource in bytes will be returned.
/// If only `setting` is `None`, the size of the subgroup's icon resource in bytes
/// will be returned.
/// If both `subgroup` and `setting` are identifiers, the size of the power setting's
/// icon resource in bytes will be returned.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have an icon
///       resource.
///
/// # Examples
///
/// TODO
///
pub fn read_icon_resource_size(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadIconResourceSpecifier(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            ptr::null_mut(),
            &mut needed_size,
        ) -> mut needed_size: u32
    }
}

/// Gets the the icon resource for the specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the icon resource of the power scheme
/// will be returned.
/// If only `setting` is `None`, the icon resource of the subgroup will be returned.
/// If both `subgroup` and `setting` are identifiers, the icon resource of the power
/// setting will be returned.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have an icon
///       resource.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadiconresourcespecifier
///
pub fn read_icon_resource(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
) -> Result<U16CString> {
    let mut buffer_size = read_icon_resource_size(scheme, subgroup, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call_WIN32_ERROR! {
        PowerReadIconResourceSpecifier(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) return Error
    }
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_wide_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

/// Gets the the icon resource as bytes for the specified power setting, subgroup, or scheme,
/// copies it into `buffer` and returns the number of bytes copied.
///
/// Use [`read_friendly_name_size`], to get the size of the icon resource in bytes for the
/// specified power setting, subgroup, or scheme.
///
/// If both `subgroup` and `setting` are `None`, the size of the power scheme's
/// icon resource in bytes will be returned and `buffer` will contain the icon resource
/// as bytes.
/// If only `setting` is `None`, the size of the subgroup's icon resource in bytes will be
/// returned and `buffer` will contain the icon resource as bytes.
/// If both `subgroup` and `setting` are identifiers, the size of the power setting's
/// icon resource in bytes will be returned and `buffer` will contain the icon resource
/// as bytes.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
/// * `buffer`: The buffer that receives the icon resource as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the icon resource as bytes.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///     * The specified power scheme, subgroup or setting does not have an icon
///       resource.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadiconresourcespecifier
///
pub fn read_icon_resource_with_buffer(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
    buffer: &mut [u8],
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadIconResourceSpecifier(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut buffer_size = buffer.len() as u32
    }
}

/// Gets the size of the description in bytes for one of the possible choices
/// of a power setting value.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible description.
///
/// # Examples
///
/// TODO
///
pub fn read_possible_description_size(subgroup: &GUID, setting: &GUID, index: u32) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadPossibleDescription(
            0,
            subgroup,
            setting,
            index,
            ptr::null_mut(),
            &mut needed_size,
        ) -> mut needed_size: u32
    }
}

/// Gets the description for one of the possible choices of a power setting value.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible description.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadpossibledescription
///
pub fn read_possible_description(
    subgroup: &GUID,
    setting: &GUID,
    index: u32,
) -> Result<U16CString> {
    let mut buffer_size = read_possible_description_size(subgroup, setting, index)?;
    let mut buffer = vec![0; buffer_size as usize];
    call_WIN32_ERROR! {
        PowerReadPossibleDescription(
            0,
            subgroup,
            setting,
            index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) return Error
    }
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_wide_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

/// Gets the description as bytes for one of the possible choices of a power
/// setting value, copies it into `buffer` and returns the number of bytes copied.
///
/// Use [`read_possible_description_size`], to get size of the possible description in
/// bytes for one of the possible choices of a power setting value.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
/// * `buffer`: The buffer that receives the possible description as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the possible description of the power setting
///       value as bytes.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible description.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadpossibledescription
///
pub fn read_possible_description_with_buffer(
    subgroup: &GUID,
    setting: &GUID,
    index: u32,
    buffer: &mut [u8],
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadPossibleDescription(
            0,
            subgroup,
            setting,
            index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut buffer_size = buffer.len() as u32
    }
}

/// Gets the size of the friendly name in bytes for one of the possible choices of a
/// power setting value.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible friendly name.
///
/// # Examples
///
/// TODO
///
pub fn read_possible_friendly_name_size(
    subgroup: &GUID,
    setting: &GUID,
    index: u32,
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadPossibleFriendlyName(
            0,
            subgroup,
            setting,
            index,
            ptr::null_mut(),
            &mut needed_size,
        ) -> mut needed_size: u32
    }
}

/// Gets the friendly name for one of the possible choices of a power setting value.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible friendly name.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadpossiblefriendlyname
///
pub fn read_possible_friendly_name(
    subgroup: &GUID,
    setting: &GUID,
    index: u32,
) -> Result<U16CString> {
    let mut buffer_size = read_possible_friendly_name_size(subgroup, setting, index)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadPossibleFriendlyName(
            0,
            subgroup,
            setting,
            index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) != 0 => return Error
    }
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_wide_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

/// Gets the friendly name as bytes for one of the possible choices of a power setting value,
/// copies it into `buffer` and returns the number of bytes copied.
///
/// Use [`read_possible_friendly_name_size`], to get size of the possible friendly name in bytes for
/// one of the possible choices of a power setting value.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
/// * `buffer`: The buffer that receives the friendly name as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the possible friendly name of
///       the power setting value as bytes.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible friendly name.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadpossiblefriendlyname
///
pub fn read_possible_friendly_name_with_buffer(
    subgroup: &GUID,
    setting: &GUID,
    index: u32,
    buffer: &mut [u8],
) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadPossibleFriendlyName(
            0,
            subgroup,
            setting,
            index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut buffer_size = buffer.len() as u32
    }
}

/// Gets the value size for a possible value of a power setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible value.
///
/// # Examples
///
/// TODO
///
pub fn read_possible_value_size(subgroup: &GUID, setting: &GUID, index: u32) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadPossibleValue(
            0,
            subgroup,
            setting,
            &mut 0,
            index,
            ptr::null_mut(),
            &mut size,
        ) -> mut size: u32
    }
}

/// Gets the value for a possible value of a power setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible value.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadpossiblevalue
///
pub fn read_possible_value(subgroup: &GUID, setting: &GUID, index: u32) -> Result<RegistryValue> {
    let mut buffer_size = read_possible_value_size(subgroup, setting, index)?;
    let buffer = vec![0; buffer_size as usize];
    call_WIN32_ERROR! {
        PowerReadPossibleValue(
            0,
            subgroup,
            setting,
            &mut reg_value.value_type,
            index,
            reg_value.data.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut reg_value = RegistryValue {
            data: buffer.into_boxed_slice(),
            value_type: 0
        }
    }
}

/// Gets the value as bytes of possible value of a power setting, copies it into `buffer`
/// and returns the number of bytes copied.
///
/// Use [`read_possible_value_size`], to get the value size of a power setting's
/// possible value in bytes.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `index`: The zero-based index for the possible setting.
/// * `buffer`: The buffer that receives the possible value as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the value data of the power setting's
///       possible value as bytes.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not have a possible value.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadpossiblevalue
///
pub fn read_possible_value_with_buffer(
    subgroup: &GUID,
    setting: &GUID,
    index: u32,
    buffer: &mut [u8],
) -> Result<(u32, u32)> {
    call_WIN32_ERROR! {
        PowerReadPossibleValue(
            0,
            subgroup,
            setting,
            &mut reg_value_type,
            index,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) -> mut (reg_value_type, buffer_size) = (0, buffer.len() as u32)
    }
}

/// Gets the current attribute of the specified power setting, subgroup or the
/// the combination of attributes for the setting and subgroup. See the
/// [Attributes][read_attributes#attributes] section for more information about
/// the possible attribute values.
///
/// If only `subgroup` is `None` then the attribute for `setting` is returned.
/// If only `setting` is `None` then the attribute for `subgroup` is returned.
/// If both `subgroup` and `setting` contain a valid identifier then the return
/// value is the combination (bitwise OR) of the attributes of the subgroup and
/// the power setting.
///
/// ## Attributes
///
/// | Attribute | Meaning |
/// |-----------|-------------|
/// | [`POWER_ATTRIBUTE_HIDE`] | The specified subgroup is hidden or both the subgroup and its power setting are hidden. |
/// | [`POWER_ATTRIBUTE_SHOW_AOAC`] | The specified subgroup is not hidden or both the subgroup and its power setting are not hidden. |
/// | `0x00000011` (`3`) | If both `subgroup` and `setting` parameters contain valid identifiers, and either the subgroup or its power setting is hidden. |
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified subgroup or power setting does not have an attribute.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadsettingattributes
///
pub fn read_attributes(subgroup: &GUID, setting: Option<&GUID>) -> Result<u32> {
    call! {
        PowerReadSettingAttributes(
            subgroup,
            setting.map_or(ptr::null(), |guid| guid),
        ) == 0 => SetError ERROR_FILE_NOT_FOUND
    }
}

/// Gets the current attribute of the specified power setting. See the
/// [Attributes][write_attributes#attributes] section for more information about
/// the possible attribute values.
///
/// If `setting` is `None` then the attribute for `subgroup` is set.
/// Otherwise the attribute for `setting` is set.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `hide`: Specifies whether to hide the specified subgroup or power setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritesettingattributes
///
pub fn write_attributes(subgroup: &GUID, setting: Option<&GUID>, hide: bool) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteSettingAttributes(
            subgroup,
            setting.map_or(ptr::null(), |guid| guid),
            if hide { POWER_ATTRIBUTE_HIDE } else { POWER_ATTRIBUTE_SHOW_AOAC },
        )
    }
}

/// Gets the increment for valid values between the power settings minimum and maximum.
/// If the power setting is not defined with a range of possible values then this
/// function will return an error.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting is not defined with a range of possible values.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadvalueincrement
///
pub fn read_value_increment(subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call_WIN32_ERROR! {
        PowerReadValueIncrement(
            0,
            subgroup,
            setting,
            &mut increment,
        ) -> mut increment: u32
    }
}

/// Retrieves the maximum value for the specified power setting.
/// If the power setting is not defined with a range of possible values then this
/// function will return an error.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting is not defined with a range of possible values.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadvaluemax
///
pub fn read_max_value(subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call! {
        PowerReadValueMax(
            0,
            subgroup,
            setting,
            &mut max,
        ) != 0 => mut max: u32
    }
}

/// Retrieves the minimum value for the specified power setting.
/// If the power setting is not defined with a range of possible values then this
/// function will return an error.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting is not defined with a range of possible values.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadvaluemin
///
pub fn read_min_value(subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call! {
        PowerReadValueMin(
            0,
            subgroup,
            setting,
            &mut min,
        ) != 0 => mut min: u32
    }
}

/// Gets the size of the string in bytes that used to describe the units of a
/// power setting that supports a range of values.
/// For example `"minutes"` may be used to describe a timeout setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not a have a unit specifier.
///
/// # Examples
///
/// TODO
///
pub fn read_value_unit_size(subgroup: &GUID, setting: &GUID) -> Result<u32> {
    call! {
        PowerReadValueUnitsSpecifier(
            0,
            subgroup,
            setting,
            ptr::null_mut(),
            &mut buffer_size,
        ) != 0 => mut buffer_size: u32
    }
}

/// Gets the string  used to describe the units of a power setting that supports
/// a range of values.
/// For example `"minutes"` may be used to describe a timeout setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not a have a unit specifier.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadvalueunitsspecifier
///
pub fn read_value_unit(subgroup: &GUID, setting: &GUID) -> Result<U16CString> {
    let mut buffer_size = read_value_unit_size(subgroup, setting)?;
    let mut buffer = vec![0; buffer_size as usize];
    call! {
        PowerReadValueUnitsSpecifier(
            0,
            subgroup,
            setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) != 0 => return Error
    };
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_wide_string_len_from_bytes(buffer.as_slice()),
        )
    })
}

/// Gets the string used to describe the units of a power setting that supports
/// a range of values, copies it into `buffer` and returns the number of bytes
/// copied.
/// For example `"minutes"` may be used to describe a timeout setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting that is being used.
/// * `buffer`: The buffer that receives the value unit string as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to hold the value unit string of the
///        specified power setting.
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///     * The specified power setting does not a have a unit specifier.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadvalueunitsspecifier
///
pub fn read_value_unit_with_buffer(
    subgroup: &GUID,
    setting: &GUID,
    buffer: &mut [u8],
) -> Result<u32> {
    call! {
        PowerReadValueUnitsSpecifier(
            0,
            subgroup,
            setting,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) != 0 => mut buffer_size = buffer.len() as u32
    }
}

/// Deletes the specified power setting.
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting to be deleted.
/// * `buffer`: The buffer that receives the value unit string as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerremovepowersetting
///
pub fn remove_setting(subgroup: &GUID, setting: &GUID) -> Result<()> {
    call_WIN32_ERROR! {
        PowerRemovePowerSetting(
            subgroup,
            setting
        )
    }
}

/// Replaces the default power schemes with the current user's power schemes.
/// This allows an administrator to change the default power schemes for the
/// system. Replacing the default schemes enables users to use the Restore
/// Defaults option in the Control Panel Power Options application to restore
/// customized power scheme defaults instead of the original Windows power
/// scheme defaults.
///
/// The caller must be a member of the local Administrators group.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * The caller is not be a member of the local Administrators group.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreplacedefaultpowerschemes
///
pub fn replace_default_schemes() -> Result<()> {
    call! { PowerReplaceDefaultPowerSchemes() == 0 }
}

/// Replaces the power schemes for the system with default power schemes.
/// All current power schemes and settings are deleted and replaced with
/// the default system power schemes.
///
/// The caller must be a member of the local Administrators group.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * The caller is not be a member of the local Administrators group.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerrestoredefaultpowerschemes
///
pub fn restore_default_power_schemes() -> Result<()> {
    call_WIN32_ERROR! { PowerRestoreDefaultPowerSchemes() }
}

/// Sets the active power scheme for the current user specified by `scheme`.
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
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powersetactivescheme
///
pub fn set_active_scheme(scheme: &GUID) -> Result<()> {
    call_WIN32_ERROR! { PowerSetActiveScheme(0, scheme) }
}

/// Queries for a group policy override for specified power settings
/// and specifies the requested access for the setting.
///
/// # Arguments
///
/// * `setting`: The identifier of the power setting.
/// * `group_policy_override_access`: The type of access to check for group policy overrides.
///     * See [Group policy override accesses][check_access#group-policy-override-accesses]
///       for possible values.
/// * `setting_access`: The type of security access for the setting.
///     * See [Power setting access rights][check_access#power-setting-access-rights]
///       for possible values.
///
/// ## Group policy override accesses
///
/// | Access | Description |
/// |--------|-------------|
/// | [`ACCESS_AC_POWER_SETTING_INDEX`] | Check for overrides on AC power settings. |
/// | [`ACCESS_DC_POWER_SETTING_INDEX`] | Check for overrides on DC power settings. |
/// | [`ACCESS_SCHEME`] | Check for restrictions on specific power schemes. |
/// | [`ACCESS_ACTIVE_SCHEME`] | Check for restrictions on active power schemes. |
/// | [`ACCESS_CREATE_SCHEME`] | Check for restrictions on creating or restoring power schemes. |
///
/// ## Power setting access rights
///
/// | Access | Description |
/// |--------|-------------|
/// | [`KEY_READ`][super::super::registry::KEY_READ] | Combines the `STANDARD_RIGHTS_READ`, [`KEY_QUERY_VALUE`][super::super::registry::KEY_QUERY_VALUE], [`KEY_ENUMERATE_SUB_KEYS`][super::super::registry::KEY_ENUMERATE_SUB_KEYS], and [`KEY_NOTIFY`][super::super::registry::KEY_NOTIFY] values. |
/// | [`KEY_WRITE`][super::super::registry::KEY_WRITE] | Combines the `STANDARD_RIGHTS_WRITE`, [`KEY_SET_VALUE`][super::super::registry::KEY_SET_VALUE], and [`KEY_CREATE_SUB_KEY`][super::super::registry::KEY_CREATE_SUB_KEY] access rights. |
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
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powersetactivescheme
///
pub fn check_access(
    // TODO: improve documentation
    setting: Option<&GUID>,
    group_policy_override_access: i32,
    setting_access: u32,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerSettingAccessCheckEx(
            group_policy_override_access,
            setting.map_or(ptr::null(), |guid| guid),
            setting_access)
    }
}

/// Sets the default AC index of the specified power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier for the single power setting.
/// * `index`: The default AC index.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwriteacdefaultindex
///
pub fn write_ac_default_index(
    scheme: &GUID,
    subgroup: &GUID,
    setting: &GUID,
    default_index: u32,
) -> Result<()> {
    call! {
        PowerWriteACDefaultIndex(
            0,
            scheme,
            subgroup,
            setting,
            default_index,
        ) == 0
    }
}

/// Sets the AC value index of the specified power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier for the single power setting.
/// * `index`: The AC value index.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerwriteacvalueindex
///
pub fn write_ac_value_index(
    scheme: &GUID,
    subgroup: &GUID,
    setting: &GUID,
    value_index: u32,
) -> Result<()> {
    call! {
        PowerWriteACValueIndex(
            0,
            scheme,
            subgroup,
            setting,
            value_index,
        ) == 0
    }
}

/// Sets the default DC index of the specified power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier for the single power setting.
/// * `index`: The default DC index.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritedcdefaultindex
///
pub fn write_dc_default_index(
    scheme: &GUID,
    subgroup: &GUID,
    setting: &GUID,
    default_index: u32,
) -> Result<()> {
    call! {
        PowerWriteDCDefaultIndex(
            0,
            scheme,
            subgroup,
            setting,
            default_index,
        ) == 0
    }
}

/// Sets the DC value index of the specified power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier for the single power setting.
/// * `index`: The DC value index.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerwritedcvalueindex
///
pub fn write_dc_value_index(
    scheme: &GUID,
    subgroup: &GUID,
    setting: &GUID,
    value_index: u32,
) -> Result<()> {
    call! {
        PowerWriteDCValueIndex(
            0,
            scheme,
            subgroup,
            setting,
            value_index,
        ) == 0
    }
}

/// Sets the description for the specified power setting, subgroup, or scheme.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// If both `subgroup` and `setting` are `None`, the description of the power scheme
/// will be set.
/// If only `setting` is `None`, the description of the subgroup will be set.
/// If both `subgroup` and `setting` are identifiers, the description of the power
/// setting will be set.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier for the single power setting.
/// * `description`: The description.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritedescription
///
pub fn write_description(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
    description: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteDescription(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            description.as_ptr().cast(),
            description.len() as u32,
        )
    }
}

/// Sets the the friendly name for the specified power setting, subgroup, or scheme.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// If both `subgroup` and `setting` are `None`, the friendly name of the power scheme
/// will be set.
/// If only `setting` is `None`, the friendly name of the subgroup will be set.
/// If both `subgroup` and `setting` are identifiers, the friendly name of the power
/// setting will be set.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
/// * `friendly_name`: The friendly name.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritefriendlyname
///
pub fn write_friendly_name(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
    friendly_name: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteFriendlyName(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            friendly_name.as_ptr().cast(),
            friendly_name.len() as u32,
        )
    }
}

/// Sets the the friendly name for the specified power setting, subgroup, or scheme.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// If both `subgroup` and `setting` are `None`, the friendly name of the power scheme
/// will be set.
/// If only `setting` is `None`, the friendly name of the subgroup will be set.
/// If both `subgroup` and `setting` are identifiers, the friendly name of the power
/// setting will be set.
///
/// # Arguments
///
/// * `scheme`: The identifier of the power scheme.
/// * `subgroup`: An optional parameter that specifies the subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`:  An optional parameter that specifies the identifier for the single power setting.
/// * `icon_resource`: The icon resource.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified power scheme, subgroup or setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwriteiconresourcespecifier
///
pub fn write_icon_resource(
    scheme: &GUID,
    subgroup: Option<&GUID>,
    setting: Option<&GUID>,
    icon_resource: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteIconResourceSpecifier(
            0,
            scheme,
            subgroup.map_or(ptr::null(), |guid| guid),
            setting.map_or(ptr::null(), |guid| guid),
            icon_resource.as_ptr().cast(),
            icon_resource.len() as u32,
        )
    }
}

/// Sets the description for one of the possible choices of a power setting value.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting.
/// * `index`: The zero-based index for the possible setting.
/// * `description`: The description.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerreadpossibledescription
///
pub fn write_possible_description(
    scheme: &GUID,
    subgroup: &GUID,
    setting_index: u32,
    description: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWritePossibleDescription(
            0,
            scheme,
            subgroup,
            setting_index,
            description.as_ptr().cast(),
            description.len() as u32,
        )
    }
}

/// Sets the friendly name for the specified possible setting of a power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting.
/// * `index`: The zero-based index for the possible setting.
/// * `friendly_name`: The friendly name.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritepossiblefriendlyname
///
pub fn write_possible_friendly_name(
    scheme: &GUID,
    subgroup: &GUID,
    setting_index: u32,
    friendly_name: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWritePossibleFriendlyName(
            0,
            scheme,
            subgroup,
            setting_index,
            friendly_name.as_ptr().cast(),
            friendly_name.len() as u32,
        )
    }
}

/// Sets the value for a possible value of a power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting.
/// * `index`: The zero-based index for the possible setting.
/// * `value`: The value for the possible setting.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritepossiblevalue
///
pub fn write_possible_value(
    subgroup: &GUID,
    setting: &GUID,
    value_type: u32,
    setting_index: u32,
    value: &[u8],
) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWritePossibleValue(
            0,
            subgroup,
            setting,
            value_type,
            setting_index,
            value.as_ptr(),
            value.len() as u32,
        )
    }
}

/// Sets the increment for valid values between the power settings minimum and maximum.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting.
/// * `index`: The zero-based index for the possible setting.
/// * `increment`: The increment to be set.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritevalueincrement
///
pub fn write_value_increment(subgroup: &GUID, setting: &GUID, increment: u32) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueIncrement(
            0,
            subgroup,
            setting,
            increment,
        )
    }
}

/// Sets the maximum value for the specified power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting.
/// * `index`: The zero-based index for the possible setting.
/// * `max`: The maximum value to be set.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritevaluemax
///
pub fn write_value_max(subgroup: &GUID, setting: &GUID, max: u32) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueMax(
            0,
            subgroup,
            setting,
            max,
        )
    }
}

/// Sets the minimum value for the specified power setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting.
/// * `index`: The zero-based index for the possible setting.
/// * `min`: The minimum value to be set.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritevaluemin
///
pub fn write_value_min(subgroup: &GUID, setting: &GUID, min: u32) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueMin(
            0,
            subgroup,
            setting,
            min,
        )
    }
}

/// Writes the string used to describe the units of a power setting that supports a
/// range of values. For example `"minutes"` may be used to describe a timeout
/// setting.
///
/// Changes to the settings for the active power scheme do not take effect until
/// you call [`set_active_scheme`].
///
/// # Arguments
///
/// * `subgroup`: The subgroup of power settings.
///     * Use [`NO_SUBGROUP_GUID`] to refer to the default power scheme.
/// * `setting`: The identifier of the power setting.
/// * `index`: The zero-based index for the possible setting.
/// * `unit`: The units specifier.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_FILE_NOT_FOUND`]
///     * The specified subgroup or power setting does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powerwritevalueunitsspecifier
///
pub fn write_value_unit(subgroup: &GUID, setting: &GUID, unit: &U16CStr) -> Result<()> {
    call_WIN32_ERROR! {
        PowerWriteValueUnitsSpecifier(
            0,
            subgroup,
            setting,
            unit.as_ptr().cast(),
            unit.len() as u32,
        )
    }
}
