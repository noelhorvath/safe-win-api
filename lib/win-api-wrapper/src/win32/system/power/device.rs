use crate::core::error::Error;
use crate::core::{get_wide_string_len_from_bytes, multi_string_bytes_to_strings, Result};
use crate::win32::foundation::MAX_PATH;
use crate::{call, call_BOOL, from_BOOL, U16CStr, U16CString};
use bitflags::bitflags;
use core::mem::size_of;
use core::ptr;
use windows_sys::Win32::Foundation::ERROR_NO_MORE_ITEMS;
use windows_sys::Win32::System::Power::{
    DevicePowerClose, DevicePowerEnumDevices, DevicePowerOpen, DevicePowerSetDeviceState,
    GetDevicePowerState,
};

use windows_sys::Win32::System::Power::{
    DEVICEPOWER_AND_OPERATION, DEVICEPOWER_FILTER_DEVICES_PRESENT, DEVICEPOWER_FILTER_ON_NAME,
    DEVICEPOWER_FILTER_WAKEENABLED, DEVICEPOWER_FILTER_WAKEPROGRAMMABLE, DEVICEPOWER_HARDWAREID,
};

pub use windows_sys::Win32::System::Power::{
    DEVICEPOWER_CLEAR_WAKEENABLED, DEVICEPOWER_SET_WAKEENABLED, PDCAP_S0_SUPPORTED,
    PDCAP_S1_SUPPORTED, PDCAP_S2_SUPPORTED, PDCAP_S3_SUPPORTED, PDCAP_S4_SUPPORTED,
    PDCAP_S5_SUPPORTED, PDCAP_WAKE_FROM_S0_SUPPORTED, PDCAP_WAKE_FROM_S1_SUPPORTED,
    PDCAP_WAKE_FROM_S2_SUPPORTED, PDCAP_WAKE_FROM_S3_SUPPORTED,
};

pub use windows_sys::Win32::System::SystemServices::{
    PDCAP_D0_SUPPORTED, PDCAP_D1_SUPPORTED, PDCAP_D2_SUPPORTED, PDCAP_D3_SUPPORTED,
    PDCAP_WAKE_FROM_D0_SUPPORTED, PDCAP_WAKE_FROM_D1_SUPPORTED, PDCAP_WAKE_FROM_D2_SUPPORTED,
    PDCAP_WAKE_FROM_D3_SUPPORTED, PDCAP_WARM_EJECT_SUPPORTED,
};

/// The maximum size of the device name that [`enum_device_names`] could return
/// based on the [offical example].
///
/// [offical example]: https://learn.microsoft.com/en-us/windows/win32/power/using-the-device-power-api.
///
pub const MAX_DEVICE_NAME_SIZE: usize = MAX_PATH as usize * size_of::<u16>();

bitflags! {
    #[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
    /// The set of option flags that can be used with [`enum_device`] and [`has_device_with_capabilities`] to specify the search criteria.
    pub struct EnumerationOptions: u32 {
        /// No options
        const None = 0;
        /// Ignore devices not currently present in the system.
        const FilterPresent = DEVICEPOWER_FILTER_DEVICES_PRESENT;
        /// Check whether the device is currently enabled to wake the system from a sleep state.
        const FilterWakeEnabled = DEVICEPOWER_FILTER_WAKEENABLED;
        /// `Undocumented`
        const FilterWakeProgrammable = DEVICEPOWER_FILTER_WAKEPROGRAMMABLE;
        /// Perform an AND operation on capability flags.
        const AndOperationOnCapabilities = DEVICEPOWER_AND_OPERATION;
        /// All options enabled
        const All = Self::FilterPresent.bits()
            | Self::FilterWakeEnabled.bits()
            | Self::FilterWakeProgrammable.bits()
            | Self::AndOperationOnCapabilities.bits();
    }
}

/// Frees all nodes in the device list and destroys the device list.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// See the [first example][enum_device_names#examples] of [`enum_device_names`].
///
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepowerclose
///
pub fn close_device_list() -> Result<()> {
    call_BOOL! { DevicePowerClose() }
}

/// Initializes a device list by querying all the devices.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// See the [first example][enum_device_names#examples] of [`enum_device_names`].
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepoweropen
///
pub fn open_device_list() -> Result<()> {
    call_BOOL! { DevicePowerOpen(0) }
}

/// Enumerates devices on the system that meet the specified criteria and returns the name of the device
/// at the specified index. If the function succeeds and no device was found or there are no more devices,
/// the result is `None`.
///
/// Opening and closing the device list is a costly process in terms of CPU time.
/// Therefore it is recommended to open and close the device list explicitly when
/// subsequent function calls are using it (implicitly) like in the [first example][enum_device_names#examples].
///
/// The function detects whether the device list is open, and if not, opens a new one.
/// If the device list was opened by the [`enum_device_names`], it is closed before the function returns, but
/// the function will not close the device list if it has been explicitly opened by [`open_device_list`].
/// To close the explicitly opened device list call [`close_device_list`].
///
/// # Arguments
///
/// * `index`: The index of the requested device.
///     * For initial calls, this value should be zero.
/// * `options`: The criteria applied to the search results.
///     * See [`EnumerationOptions`].
/// * `capability_flags`: The flags that specify the query criteria.
///     * See the [`Query flags`][enum_device_names#capability-flags] section.
///
/// ## Capability flags
///
/// | Flag | Description |
/// |------|------|
/// | [`PDCAP_D0_SUPPORTED`] | The device supports system power state D0. |
/// | [`PDCAP_D1_SUPPORTED`] | The device supports system power state D1. |
/// | [`PDCAP_D2_SUPPORTED`] | The device supports system power state D2. |
/// | [`PDCAP_D3_SUPPORTED`] | The device supports system power state D3. |
/// | [`PDCAP_S0_SUPPORTED`] | The device supports system power state S0. |
/// | [`PDCAP_S1_SUPPORTED`] | The device supports system power state S1. |
/// | [`PDCAP_S2_SUPPORTED`] | The device supports system power state S2. |
/// | [`PDCAP_S3_SUPPORTED`] | The device supports system power state S3. |
/// | [`PDCAP_S4_SUPPORTED`] | The device supports system power state S4. |
/// | [`PDCAP_S5_SUPPORTED`] | The device supports system power state S5. |
/// | [`PDCAP_WAKE_FROM_D0_SUPPORTED`] | The device supports waking from system power state D0. |
/// | [`PDCAP_WAKE_FROM_D1_SUPPORTED`] | The device supports waking from system power state D1. |
/// | [`PDCAP_WAKE_FROM_D2_SUPPORTED`] | The device supports waking from system power state D2. |
/// | [`PDCAP_WAKE_FROM_D3_SUPPORTED`] | The device supports waking from system power state D3. |
/// | [`PDCAP_WAKE_FROM_S0_SUPPORTED`] | The device supports waking from system power state S0. |
/// | [`PDCAP_WAKE_FROM_S1_SUPPORTED`] | The device supports waking from system power state S1. |
/// | [`PDCAP_WAKE_FROM_S2_SUPPORTED`] | The device supports waking from system power state S2. |
/// | [`PDCAP_WAKE_FROM_S3_SUPPORTED`] | The device supports waking from system power state S3. |
/// | [`PDCAP_WARM_EJECT_SUPPORTED`] | The device supports warm eject. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// Enumerating all devices and querying their name:
///
/// ```
/// // Opening and closing the device list is a costly process in terms of CPU time.
/// // If the device list is used more than once it is efficient to open it explicitly
/// // and close it after it is no longer needed.
/// // Opening the device list explicitly.
/// if let Err(error) = open_device_list() {
///     println!("Failed to open the device list -> {}", error);
///     return 1;
/// } else {
///     let mut i = 0;
///     loop {
///         // `enum_device_names` detects the opened device list and will not initialize a new one every time
///         // it is called in this case.
///         match enum_device_names(i, EnumerationOptions::None, u32::MAX) {
///             Ok(Some(device_name)) => {
///                 println!("{}. device: {}", i, device_name.to_string_lossy());
///                 i += 1;
///             }
///             Ok(None) => {
///                 if i == 0 {
///                     println!("No device was found.");
///                 } else {
///                     println!("Enumeration finished.");
///                 }
///                 break;
///             }
///             Err(error) => {
///                 println!("Enumeration failed: {:?}", error);
///                 return 2;
///             }
///         }
///     }
///     // Freeing the device list as it is no longer needed.
///     if let Err(error) = close_device_list() {
///         println!("Failed to close the device list -> {}", error);
///     }
/// }
/// ```
///
/// Based on the official `C++` [example].
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepowerenumdevices
/// [example]: https://learn.microsoft.com/en-us/windows/win32/power/using-the-device-power-api
///
pub fn enum_device_names(
    index: u32,
    options: EnumerationOptions,
    capability_flags: u32,
) -> Result<Option<U16CString>> {
    let mut buffer = [0; MAX_DEVICE_NAME_SIZE];
    call_BOOL! {
        DevicePowerEnumDevices(
            index,
            options.0.0,
            capability_flags,
            buffer.as_mut_ptr(),
            &mut (buffer.len() as u32),
        ) -> if Error == ERROR_NO_MORE_ITEMS => None;
             else return
    };
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(Some(unsafe {
        U16CString::from_ptr_unchecked(
            buffer.as_ptr().cast(),
            get_wide_string_len_from_bytes(buffer.as_slice()),
        )
    }))
}

/// Enumerates devices on the system that meet the specified criteria, gets the name of the device
/// at the specified index and copies it into `buffer`. If the function succeeds the return value
/// indicates whether a device name was found at `index`.
///
/// Opening and closing the device list is a costly process in terms of CPU time.
/// Therefore it is recommended to open and close the device list explicitly when
/// subsequent function calls are using it (implicitly) like in the [first example][enum_device_names_with_buffer#examples].
///
/// The function detects whether the device list is open, and if not, opens a new one.
/// If the device list was opened by the [`enum_device_names_with_buffer`], it is closed before the function returns, but
/// the function will not close the device list if it has been explicitly opened by [`open_device_list`].
/// To close the explicitly opened device list call [`close_device_list`].
///
/// # Arguments
///
/// * `index`: The index of the requested device.
///     * For initial calls, this value should be zero.
/// * `options`: The criteria applied to the search results.
///     * See [`EnumerationOptions`].
/// * `capability_flags`: The flags that specify the query criteria.
///     * See the [`Capability flags`][enum_device_names#capability-flags] section.
/// * `buffer`: A buffer with enough size to contain the null-terminated name of the device
///             as bytes.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * [`ERROR_INSUFFICIENT_BUFFER`][windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER]
///     * `buffer` has insufficient size to contain the null-terminated name of the queried device.
///
/// # Examples
///
/// Enumerating all devices and querying their name with a buffer:
///
/// ```
/// // Opening and closing the device list is a costly process in terms of CPU time.
/// // If the device list is used more than once it is efficient to open it explicitly
/// // and close it after it is no longer needed.
/// // Opening the device list explicitly.
/// if let Err(error) = open_device_list() {
///     println!("Failed to open the device list -> {}", error);
/// } else {
///     let mut i = 0;
///     let mut buffer = [0; MAX_DEVICE_NAME_SIZE];
///     loop {
///         // `enum_device_names_with_buffer` detects the opened device list and will not initialize a new one every time
///         // it is called in this case.
///         match enum_device_names_with_buffer(i, EnumerationOptions::None, u32::MAX, buffer.as_mut_slice()) {
///             Ok(has_name) => {
///                 if has_name {
///                     println!("{}. raw device name received", i);
///                 } else {
///                     if i == 0 {
///                         println!("No device was found.");
///                     } else {
///                         println!("Enumeration finished.");
///                     }
///                     break;
///                 }
///                 i += 1;
///             }
///             Err(error) => {
///                 println!("Enumeration failed: {:?}", error);
///                 break;
///             }
///         }
///     }
///     // Freeing the device list as it is no longer needed.
///     if let Err(error) = close_device_list() {
///         println!("Failed to close the device list -> {}", error);
///     }
/// }
/// ```
///
/// Based on the official `C++` [example].
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepowerenumdevices
/// [example]: https://learn.microsoft.com/en-us/windows/win32/power/using-the-device-power-api
///
pub fn enum_device_names_with_buffer(
    index: u32,
    options: EnumerationOptions,
    capability_flags: u32,
    buffer: &mut [u8],
) -> Result<bool> {
    call_BOOL! {
        DevicePowerEnumDevices(
            index,
            options.0.0,
            capability_flags,
            buffer.as_mut_ptr(),
            &mut (buffer.len() as u32),
        ) -> if Error == ERROR_NO_MORE_ITEMS => false;
             else return
    };
    Ok(true)
}

/// Enumerates devices on the system that meet the specified criteria and returns all hardware identifiers
/// of the device at the specified index. If the function succeeds the return value indicates whether a
/// device name was found at `index`.
///
/// Opening and closing the device list is a costly process in terms of CPU time.
/// Therefore it is recommended to open and close the device list explicitly when
/// subsequent function calls are using it (implicitly) like in the [first example][enum_device_hardware_ids#examples].
///
/// The function detects whether the device list is open, and if not, opens a new one.
/// If the device list was opened by the [`enum_device_hardware_ids`], it is closed before the function returns, but
/// the function will not close the device list if it has been explicitly opened by [`open_device_list`].
/// To close the explicitly opened device list call [`close_device_list`].
///
/// # Arguments
///
/// * `index`: The index of the requested device.
///     * For initial calls, this value should be zero.
/// * `options`: The criteria applied to the search results.
///     * See [`EnumerationOptions`].
/// * `capability_flags`: The flags that specify the query criteria.
///     * See the [`Capability flags`][enum_device_names#capability-flags] section.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// Enumerating all devices and querying their hardware ids:
///
/// ```
/// // Opening and closing the device list is a costly process in terms of CPU time.
/// // If the device list is used more than once it is efficient to open it explicitly
/// // and close it after it is no longer needed.
/// // Opening the device list explicitly.
/// if let Err(error) = open_device_list() {
///     println!("Failed to open the device list -> {}", error);
/// } else {
///     let mut i = 0;
///     loop {
///         // `enum_device_hardware_ids` detects the opened device list and will not initialize a new one every time
///         // it is called in this case.
///         match power::device::enum_device_hardware_ids(i, EnumerationOptions::None, u32::MAX) {
///             Ok(Some(hardware_ids)) => {
///                 println!("{}. device hardware ids: {:?}", i, hardware_ids);
///                 i += 1;
///             }
///             Ok(None) => {
///                 if i == 0 {
///                     println!("No device was found.");
///                 } else {
///                     println!("Enumeration finished.");
///                 }
///                 break;
///             }
///             Err(error) => {
///                 println!("{:?}", error);
///                 break;
///             }
///         }
///     }
///     // Freeing the device list as it is no longer needed.
///     if let Err(error) = close_device_list() {
///         println!("Failed to close the device list -> {}", error);
///     }
/// }
/// ```
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepowerenumdevices
///
pub fn enum_device_hardware_ids(
    index: u32,
    options: EnumerationOptions,
    capability_flags: u32,
) -> Result<Option<Vec<U16CString>>> {
    let mut buffer = [0; 2048];
    let result = unsafe {
        DevicePowerEnumDevices(
            index,
            options.0 .0 | DEVICEPOWER_HARDWAREID,
            capability_flags,
            buffer.as_mut_ptr(),
            &mut (buffer.len() as u32),
        )
    };
    if from_BOOL!(result) {
        Ok(Some(multi_string_bytes_to_strings(buffer.as_slice())))
    } else {
        let last_error = Error::last_win32();
        if last_error.code.as_u32() == ERROR_NO_MORE_ITEMS {
            Ok(None)
        } else {
            Err(last_error)
        }
    }
}

/// Enumerates devices on the system that meet the specified criteria, gets the hardware identifiers of the
/// device at the specified index as a multi-string byte sequence and copies it into `buffer`. If the function
/// succeeds the return value indicates whether a device name was found at `index`.
///
/// Opening and closing the device list is a costly process in terms of CPU time.
/// Therefore it is recommended to open and close the device list explicitly when
/// subsequent function calls are using it (implicitly) like in the [first example][enum_device_hardware_ids_with_buffer#examples].
///
/// The function detects whether the device list is open, and if not, opens a new one.
/// If the device list was opened by the [`enum_device_hardware_ids_with_buffer`], it is closed before the function returns, but
/// the function will not close the device list if it has been explicitly opened by [`open_device_list`].
/// To close the explicitly opened device list call [`close_device_list`].
///
/// # Arguments
///
/// * `index`: The index of the requested device.
///     * For initial calls, this value should be zero.
/// * `options`: The criteria applied to the search results.
///     * See [`EnumerationOptions`].
/// * `capability_flags`: The flags that specify the query criteria.
///     * See the [`Capability flags`][enum_device_names#capability-flags] section.
/// * `buffer`: A buffer that is large enough to contain the multi-string byte sequence that
///             represents the hardware ids of the specified device.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * [`ERROR_INSUFFICIENT_BUFFER`][windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER]
///     * `buffer` has insufficient size to contain the multi-string byte sequence that represents
///       the hardware ids of the specified device.
///
/// # Examples
///
/// Enumerating all devices and querying their hardware ids with a buffer:
///
/// ```
/// // Opening and closing the device list is a costly process in terms of CPU time.
/// // If the device list is used more than once it is efficient to open it explicitly
/// // and close it after it is no longer needed.
/// // Opening the device list explicitly.
/// if let Err(error) = open_device_list() {
///     println!("Failed to open the device list -> {}", error);
/// } else {
///     let mut i = 0;
///     let mut buffer = [0; 2048];
///     loop {
///         // `enum_device_names_with_buffer` detects the opened device list and will not initialize a new one every time
///         // it is called in this case.
///         match enum_device_hardware_ids_with_buffer(i, EnumerationOptions::None, u32::MAX, buffer.as_mut_slice()) {
///             Ok(has_hardware_ids) => {
///                 if has_hardware_ids {
///                     println!("{}. device raw hardware ids received.", i);
///                 } else {
///                     if i == 0 {
///                         println!("No device was found.");
///                     } else {
///                         println!("Enumeration finished.");
///                     }
///                     break;
///                 }
///                 i += 1;
///             }
///             Err(error) => {
///                 println!("Enumeration failed: {:?}", error);
///                 break;
///             }
///         }
///     }
///     // Freeing the device list as it is no longer needed.
///     if let Err(error) = close_device_list() {
///         println!("Failed to close the device list -> {}", error);
///     }
/// }
/// ```
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepowerenumdevices
///
pub fn enum_device_hardware_ids_with_buffer(
    index: u32,
    options: EnumerationOptions,
    capability_flags: u32,
    buffer: &mut [u8],
) -> Result<bool> {
    call_BOOL! {
        DevicePowerEnumDevices(
            index,
            options.0.0 | DEVICEPOWER_HARDWAREID,
            capability_flags,
            buffer.as_mut_ptr(),
            &mut (buffer.len() as u32),
        ) -> if Error == ERROR_NO_MORE_ITEMS => false;
             else return
    };
    Ok(true)
}

/// Determines whether the device specified by `device_name` has one of the capabilities.
/// To match all the capabilites use the [`EnumerationOptions::AndOperationOnCapabilities`] flag.
///
/// Opening and closing the device list is a costly process in terms of CPU time.
/// Therefore it is recommended to open and close the device list explicitly when
/// subsequent function calls are using it (implicitly) like in the [first example][has_device#examples].
///
/// The function detects whether the device list is open, and if not, opens a new one.
/// If the device list was opened by the [`has_device`], it is closed before the function returns, but
/// the function will not close the device list if it has been explicitly opened by [`open_device_list`].
/// To close the explicitly opened device list call [`close_device_list`].
///
/// # Arguments
///
/// * `index`: The index of the requested device.
///     * For initial calls, this value should be zero.
/// * `options`: The criteria applied to the search results.
///     * See [`EnumerationOptions`].
/// * `capability_flags`: The query criteria.
///     * See the [`Capability flags`][enum_device_names#query-flags] section.
/// * `filter_device_name`: The query criteria.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
///
/// Checking if the fingerprint reader supports D0 and D1 power states:
///
/// ```
/// let device_name = u16cstr!("FPC Fingerprint Reader (Disum)");
///     // Let `has_device` open the device list for this one function call
///     match power::device::has_device(
///         device_name,
///         // With option we can check if the device
///         // supports both D0 and D1 power states.
///         // If we used `EnumerationOptions::None` the function
///         // would check if the device either has D0 or D1 support
///         EnumerationOptions::AndOperationOnCapabilities,
///         PDCAP_D1_SUPPORTED | PDCAP_D0_SUPPORTED,
///     ) {
///         Ok(has_capabilities) => {
///             println!(
///                 "The fingerprint device {} both
///                  D0 and D1 power states.",
///                 if has_capabilities {
///                     "supports"
///                 } else {
///                     "does not support"
///                 }
///             );
///         }
///         Err(error) => println!("{:?}", error),
///     }
/// ```
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepowerenumdevices
///
pub fn has_device(
    device_name: &U16CStr,
    options: EnumerationOptions,
    capability_flags: u32,
) -> Result<bool> {
    if device_name.len() == 0 {
        return Ok(false);
    }

    let buffer_size = (device_name.len() + 1) * size_of::<u16>(); // size has to include the null terminator as well
    let mut buffer = vec![0_u8; buffer_size];
    // Safety: `device_name` is valid string poninter for `name_size` length.
    let filter_bytes_slice =
        unsafe { core::slice::from_raw_parts(device_name.as_ptr() as *const u8, buffer_size) };
    buffer.copy_from_slice(filter_bytes_slice);
    call_BOOL! {
        DevicePowerEnumDevices(
            0,
            options.0.0 | DEVICEPOWER_FILTER_ON_NAME,
            capability_flags,
            buffer.as_mut_ptr(),
            &mut (buffer.len() as u32),
        ) -> if Error == ERROR_NO_MORE_ITEMS => false;
             else return
    };
    // Safety: `buffer` is valid for `get_wide_string_len_from_bytes` elements.
    //          Result of `get_wide_string_len_from_bytes` is always less than equal to `buffer_size` / `size_of::<u16>`
    Ok(buffer[0] != 0)
}

/// Retrieves the current power state of the specified object on the device such
/// as a file or socket, or the device itself.
/// This function cannot be used to query the power state of a display device.
///
/// If the device is in lower-power state the returned value is `0`. Otherwise the
/// device is in working state.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `device_handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getdevicepowerstate
///
pub fn get_device_power_state(handle: isize) -> Result<i32> {
    call_BOOL! {
        GetDevicePowerState(handle, &mut power_state) -> mut power_state: i32
    }
}

/// Modifies the specified data on the specified device.
///
/// # Arguments
///
/// * `id`: The name or hardware identifier string of the device to be modified.
/// * `flags`: The properties of the device that are to be modified.
///     * See the [`Set Flags`][set_device_state#set-flags] section
///
/// ## Set flags
///
/// | Flag | Description |
/// |------|------|
/// | [`DEVICEPOWER_SET_WAKEENABLED`] | Enables the specified device to wake the system. |
/// | [`DEVICEPOWER_CLEAR_WAKEENABLED`] | Stops the specified device from being able to wake the system. |
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-devicepowersetdevicestate
///
pub fn set_device_state(id: &U16CStr, flags: u32) -> Result<u32> {
    call! {
        DevicePowerSetDeviceState(id.as_ptr(), flags, ptr::null()) != 0
    }
}
