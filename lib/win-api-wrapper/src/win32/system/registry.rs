use crate::core::error::{Code, Error};
use crate::core::Result;
use crate::{call_WIN32_ERROR, default_sized, from_BOOL, to_BOOL};
use core::mem::size_of;
use core::ptr;
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::{
    ERROR_MORE_DATA, ERROR_NO_MORE_ITEMS, ERROR_SUCCESS, FILETIME,
};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::System::Registry::{
    RegCloseKey, RegConnectRegistryW, RegCopyTreeW, RegCreateKeyExW, RegCreateKeyTransactedW,
    RegDeleteKeyExW, RegDeleteKeyTransactedW, RegDeleteKeyValueW, RegDeleteTreeW, RegDeleteValueW,
    RegDisablePredefinedCacheEx, RegDisableReflectionKey, RegEnableReflectionKey, RegEnumKeyExW,
    RegEnumValueW, RegFlushKey, RegGetValueW, RegLoadKeyW, RegLoadMUIStringW,
    RegNotifyChangeKeyValue, RegOpenCurrentUser, RegOpenKeyExW, RegOpenKeyTransactedW,
    RegOpenUserClassesRoot, RegOverridePredefKey, RegQueryInfoKeyW, RegQueryMultipleValuesW,
    RegQueryReflectionKey, RegRenameKey, RegReplaceKeyW, RegRestoreKeyW, RegSaveKeyExW,
    RegSaveKeyW, RegSetKeyValueW, RegSetValueExW, RegUnLoadKeyW, REG_MUI_STRING_TRUNCATE,
};

pub use windows_sys::Win32::System::SystemServices::ACCESS_SYSTEM_SECURITY;

pub use windows_sys::Win32::System::Registry::{
    HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_CURRENT_USER_LOCAL_SETTINGS, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA,
    HKEY_PERFORMANCE_NLSTEXT, HKEY_PERFORMANCE_TEXT, HKEY_USERS, KEY_ALL_ACCESS, KEY_CREATE_LINK,
    KEY_CREATE_SUB_KEY, KEY_ENUMERATE_SUB_KEYS, KEY_EXECUTE, KEY_NOTIFY, KEY_QUERY_VALUE, KEY_READ,
    KEY_SET_VALUE, KEY_WOW64_32KEY, KEY_WOW64_64KEY, KEY_WOW64_RES, KEY_WRITE, REG_BINARY,
    REG_CREATED_NEW_KEY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_DWORD_LITTLE_ENDIAN, REG_EXPAND_SZ,
    REG_FORCE_RESTORE, REG_LATEST_FORMAT, REG_LINK, REG_MULTI_SZ, REG_NONE,
    REG_NOTIFY_CHANGE_ATTRIBUTES, REG_NOTIFY_CHANGE_LAST_SET, REG_NOTIFY_CHANGE_NAME,
    REG_NOTIFY_CHANGE_SECURITY, REG_NOTIFY_THREAD_AGNOSTIC, REG_NO_COMPRESSION,
    REG_OPENED_EXISTING_KEY, REG_OPTION_BACKUP_RESTORE, REG_OPTION_CREATE_LINK,
    REG_OPTION_NON_VOLATILE, REG_OPTION_OPEN_LINK, REG_OPTION_VOLATILE, REG_QWORD,
    REG_QWORD_LITTLE_ENDIAN, REG_ROUTINE_FLAGS, REG_SAM_FLAGS as RegistryAccessRights,
    REG_STANDARD_FORMAT, REG_SZ, REG_VALUE_TYPE, REG_WHOLE_HIVE_VOLATILE, RRF_NOEXPAND, RRF_RT_ANY,
    RRF_RT_DWORD, RRF_RT_QWORD, RRF_RT_REG_BINARY, RRF_RT_REG_DWORD, RRF_RT_REG_EXPAND_SZ,
    RRF_RT_REG_MULTI_SZ, RRF_RT_REG_NONE, RRF_RT_REG_QWORD, RRF_RT_REG_SZ, RRF_SUBKEY_WOW6432KEY,
    RRF_SUBKEY_WOW6464KEY, RRF_ZEROONFAILURE, VALENTW,
};

/// The right to delete an object.
pub const DELETE: u32 = 65536; // windows_sys::Win32::Storage::FileSystem::DELETE

/// Contains information about a registry key.
///
/// This information is returned by [`enum_subkeys`].
pub struct RegistryKeyEnumInfo {
    /// The name of the registry key.
    pub name: U16CString,
    /// The class of the registry key.
    pub class: U16CString,
    /// The time at which the key was last written.
    pub last_write_time: FILETIME,
}

impl core::fmt::Debug for RegistryKeyEnumInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegistryKeyEnumInfo")
            .field("name", &self.name)
            .field("class", &self.name)
            .field(
                "last_write_time.dwHighDateTime",
                &self.last_write_time.dwHighDateTime,
            )
            .field(
                "last_write_time.dwLowDateTime",
                &self.last_write_time.dwLowDateTime,
            )
            .finish()
    }
}

impl ::core::default::Default for RegistryKeyEnumInfo {
    fn default() -> Self {
        Self {
            name: U16CString::new(),
            class: U16CString::new(),
            last_write_time: default_sized!(FILETIME),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A registry value.
pub struct RegistryValue {
    /// The raw data of the value.
    pub data: Box<[u8]>,
    /// The type of the contained data.
    pub value_type: u32,
}

impl ::core::default::Default for RegistryValue {
    fn default() -> Self {
        Self {
            data: Box::new([]),
            value_type: 0,
        }
    }
}

#[derive(Debug)]
/// Contains information about a named registry value.
///
/// This information is returned by [`enum_values`].
pub struct RegistryValueEnumInfo {
    /// The name of the registry value.
    pub name: U16CString,
    /// Contains the raw data and the type of the registry value.
    pub value: RegistryValue,
}

impl ::core::default::Default for RegistryValueEnumInfo {
    fn default() -> Self {
        Self {
            name: U16CString::new(),
            value: RegistryValue::default(),
        }
    }
}

#[derive(Clone)]
/// Information about a registry key.
///
/// This information can be retrieved either by [`get_info`] or [`get_info_without_class`].
pub struct RegistryKeyInfo {
    /// The handle to the registry key.
    pub handle: isize,
    /// The class of `handle`.
    pub class: U16CString,
    /// The number of subkeys contained by the key.
    pub subkey_count: u32,
    /// The length of the key's longest subkey name.
    ///
    /// If the information is returned by [`get_info`] or [`get_info_without_class`],
    /// the field does not include the terminating null character in the length.
    pub max_sub_key_name_len: u32,
    /// The length of the key's longest subkey class.
    ///
    /// If the information is returned by [`get_info`] or [`get_info_without_class`],
    /// the field does not include the terminating null character in the length.
    pub max_sub_key_class_len: u32,
    /// The number of values associated with the key.
    pub value_count: u32,
    /// The length of the key's longest value name.
    ///
    /// If the information is returned by [`get_info`] or [`get_info_without_class`],
    /// the field does not include the terminating null character in the length.
    pub max_value_name_len: u32,
    /// The size of the longest data component among the key's values.
    pub max_value_len: u32,
    /// The size of the key's security descriptor in bytes.
    pub security_descriptor_size: u32,
    /// The last time that the key or any of its values entries were modified.
    pub last_write_time: FILETIME,
}

impl core::fmt::Debug for RegistryKeyInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegistryKeyEnumInfo")
            .field("handle", &self.handle)
            .field("class", &self.class)
            .field("subkey_count", &self.subkey_count)
            .field("max_sub_key_name_len", &self.max_sub_key_name_len)
            .field("max_sub_key_class_len", &self.max_sub_key_class_len)
            .field("value_count", &self.value_count)
            .field("max_value_len", &self.max_value_len)
            .field("max_value_len", &self.max_value_len)
            .field("security_descriptor_size", &self.security_descriptor_size)
            .field(
                "last_write_time.dwHighDateTime",
                &self.last_write_time.dwHighDateTime,
            )
            .field(
                "last_write_time.dwLowDateTime",
                &self.last_write_time.dwLowDateTime,
            )
            .finish()
    }
}

impl ::core::default::Default for RegistryKeyInfo {
    fn default() -> Self {
        RegistryKeyInfo {
            handle: 0,
            class: U16CString::new(),
            subkey_count: 0,
            max_sub_key_name_len: 0,
            max_sub_key_class_len: 0,
            value_count: 0,
            max_value_name_len: 0,
            max_value_len: 0,
            security_descriptor_size: 0,
            last_write_time: default_sized!(FILETIME),
        }
    }
}

/// Closes a handle to the specified registry key.
///
/// The handle for a specified key should not be used after it has been closed,
/// because it will no longer be valid. Key handles should not be left open any
/// longer than necessary.
///
/// This function does not necessarily write information to the registry
/// before returning; it can take as much as several seconds for the cache to be
/// flushed to the hard disk. If an application must explicitly write registry information
/// to the hard disk, it can use the [`flush`] function. [`flush`], however, uses
/// many system resources and should be called only when necessary.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey
///
pub fn close(handle: isize) -> Result<()> {
    call_WIN32_ERROR! { RegCloseKey(handle) }
}

/// Creates the specified registry key. If the key already exists, the function opens it.
/// Note that key names are not case sensitive.
///
/// The returned value contains the opened or created handle to the specified key and one
/// of the following values:
/// * [`REG_CREATED_NEW_KEY`]: The key did not exist and was created.
/// * [`REG_OPENED_EXISTING_KEY`]: The key existed and was simply opened without being changed.
///
///
/// The key that the [`create`] creates has no values.
/// An application can use the [`set_value`] function to set key values.
///
/// If the returned handle is not a predefined key and it is no longer needed,
/// it should be closed by calling [`close`].
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][create#allowed-predefined-keys] section.
/// * `subkey_name`: The name of the subkey that this function opens or creates.
///     * If this parameter is an empty string, the returned handle is a new handle
///       to the key specified by `key_handle`.
/// * `class`: The user-defined class type iof this key.
/// * `options`: The options to use when opening or createing the specified key.
///     * See [Options][create#options] for possible values.
/// * `access`: Sepcifies the access rights for the key to be created.
/// * `security_attributes`: An optional parameter that determines whether the returned handle can
///   be inheritied by child processes.
///     * If this parameter is
///         * `None`, the handle cannot be inherited and the key gets a default
///           security descriptor. The ACLs in a default security descriptor for a key are inherited from its
///           direct parent key.
///         * an instance of [`SECURITY_ATTRIBUTES`], the `lpSecurityDescriptor field` of the structure specifies a
///           security descriptor for the new key.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///  
/// # Options
///
/// | Option flag | Meaning |
/// |-------------|---------|
/// | [`REG_OPTION_BACKUP_RESTORE`] | If this flag is set, the function ignores the `access` parameter and attempts to open the key with the access required to backup or restore the key. If the calling thread has the [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privilege enabled, the key is opened with the [`ACCESS_SYSTEM_SECURITY`] and [`KEY_READ`] access rights. If the calling thread has the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME] privilege enabled, beginning with Windows Vista, the key is opened with the [`ACCESS_SYSTEM_SECURITY`], [`DELETE`] and [`KEY_WRITE`] access rights. If both privileges are enabled, the key has the combined access rights for both privileges. |
/// | [`REG_OPTION_CREATE_LINK`] | Registry symbolic links should only be used for for application compatibility when absolutely necessary. This key is a symbolic link. The target path is assigned to the "SymbolicLinkValue" value of the key. The target path must be an absolute registry path. |
/// | [`REG_OPTION_NON_VOLATILE`] | This key is not volatile; this is the default. The information is stored in a file and is preserved when the system is restarted. The [`save`] saves keys that are not volatile. |
/// | [`REG_OPTION_VOLATILE`] | All keys created by the function are volatile. The information is stored in memory and is not preserved when the corresponding registry hive is unloaded. For [`HKEY_LOCAL_MACHINE`], this occurs only when the system initiates a full shutdown. For registry keys loaded by the [`load`], this occurs when the corresponding [`unload`] is performed. The [`save`] does not save volatile keys. This flag is ignored for keys that already exist. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `key_handle` does not have [`KEY_CREATE_SUB_KEY`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexw
///
pub fn create(
    key_handle: isize,
    subkey_name: &U16CStr,
    class: &U16CStr,
    options: u32,
    access: RegistryAccessRights,
    security_attributes: Option<&SECURITY_ATTRIBUTES>,
) -> Result<(isize, u32)> {
    call_WIN32_ERROR! {
        RegCreateKeyExW(
            key_handle,
            subkey_name.as_ptr(),
            0,
            class.as_ptr(),
            options,
            access,
            security_attributes.map_or(ptr::null(), |attr| attr),
            &mut key,
            &mut disposition_info,
        ) -> mut (key, disposition_info): (isize, u32)
    }
}

/// Opens the specified registry key. Note that key names are not case sensitive.
///
/// If the returned handle is not a predefined key and it is no longer needed,
/// it should be closed by calling [`close`].
///
/// If your service or application impersonates different users,
/// do not use this function with [`HKEY_CURRENT_USER`]. Instead,
/// call [`open_current_user`].
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predefined keys listed
///    in the [Allowed predefined keys][open#allowed-predefined-keys] section.
/// * `subkey_name`: The name of the registry subkey to opened.
///     * If this parameter is an empty string and `key_handle` is a predefined key, then the system refreshes the
///       predefined key, then returns `key_handle`.
/// * `options`: Sepcified the option to apply when opening the key.
///     * Set this parameter to zero or the following:
///         * [`REG_OPTION_OPEN_LINK`]: The key is a symbolic link. Registry symbolic links should only be used when absolutely necessary.
/// * `access`: The desired access rights to the key to be opened.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///  
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw
///
pub fn open(
    key_handle: isize,
    subkey_name: &U16CStr,
    options: u32,
    access: RegistryAccessRights,
) -> Result<isize> {
    call_WIN32_ERROR! {
        RegOpenKeyExW(
            key_handle,
            subkey_name.as_ptr(),
            options,
            access,
            &mut opened_key_handle,
        ) -> mut opened_key_handle: isize
    }
}

/// Opens the specified registry key and associates it with a transaction.
/// Note that key names are not case sensitive.
///
/// If the returned handle is not a predefined key and it is no longer needed,
/// it should be closed by calling [`close`].
///
/// When a key is opened using this function, subsequent operations on the key are transacted.
/// If a non-transacted operation is performed on the key before the transaction is committed,
/// the transaction is rolled back. After a transaction is committed or rolled back, you must
/// re-open the key using the [`create_transacted`] or [`open_transacted`] with an active
/// transaction handle to make additional operations transacted.
///
/// Note that subsequent operations on subkeys of this key are not automatically transacted.
/// Therefore, the [`delete`] does not perform a transacted delete operation.
/// Instead, use the [`delete_transacted`] to perform a transacted delete operation
///
/// Unlike the [`create_transacted`], the RegOpenKeyTransacted function does not create
/// the specified key if the key does not exist in the registry.
///
/// If your service or application impersonates different users,
/// do not use this function with [`HKEY_CURRENT_USER`]. Instead,
/// call [`open_current_user`].
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predefined keys listed
///    in the [Allowed predefined keys][open_transacted#allowed-predefined-keys] section.
///     * If this parameter is a predefined key, the returned key handle is not included
///       in the provided transaction.
/// * `subkey_name`: The name of the registry subkey to opened.
///     * If this parameter is an empty string and `key_handle` is a predefined key, then
///       the system refreshes the predefined key, then returns `key_handle`.
/// * `access`: The desired access rights to the key to be opened.
/// * `transaction_handle`: A handle to an active transaction.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///  
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * `transaction_handle` is invalid.
/// * [`ERROR_NO_SYSTEM_RESOURCES`][windows_sys::Win32::Foundation::ERROR_NO_SYSTEM_RESOURCES]
///     * A single registry key was attempted to be opened for the `65535`th time.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeytransactedw
///
pub fn open_transacted(
    key_handle: isize,
    subkey_name: &U16CStr,
    options: u32,
    access: RegistryAccessRights,
    transaction_handle: isize,
) -> Result<isize> {
    call_WIN32_ERROR! {
        RegOpenKeyTransactedW(
            key_handle,
            subkey_name.as_ptr(),
            options,
            access,
            &mut opened_key_handle,
            transaction_handle,
            ptr::null(),
        ) -> mut opened_key_handle: isize
    }
}

/// Gets a handle to the [`HKEY_CURRENT_USER`] with the specified access rights for the
/// user the current thread is impersonating.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopencurrentuser
///
pub fn open_current_user(access: RegistryAccessRights) -> Result<isize> {
    call_WIN32_ERROR! {
        RegOpenCurrentUser(access, &mut key) -> mut key: isize
    }
}

/// Gets a handle to the [`HKEY_CLASSES_ROOT`] key for a specified user.
/// The user is identified by an access token. The returned key has a view of the registry that
/// merges the contents of the `HKEY_LOCAL_MACHINE\Software\Classes` key with the contents of the
/// `Software\Classes` keys in the user's registry hive.
///
/// This function enables you to retrieve the merged [`HKEY_CLASSES_ROOT`]
/// information for users other than the interactive user.
///
/// [`open_user_classes_root`] fails if the user profile for the specified user is not loaded.
/// When a user logs on interactively, the system automatically loads the user's profile. For
/// other users, you can call the `LoadUserProfile` function to load the user's profile. However,
/// `LoadUserProfile` can be very time-consuming, so do not call it for this purpose unless it is
/// absolutely necessary to have the user's merged [`HKEY_CLASSES_ROOT`] information.
///
/// Applications running in the security context of the interactively logged-on user do not need to
/// use [`open_user_classes_root`]. These applications can call the [`open`] function to retrieve a
/// merged view of the [`HKEY_CLASSES_ROOT`] key for the interactive user.
///
/// # Arguments
///
/// * `access_token_handle`: A handle to a primary or impersonation access token that identifies the user of interest.
/// * `access`: The desired access rights for the key.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `access_token_handle` is invalid.
/// * `access_token_handle` does not have [`TOKEN_QUERY`][windows_sys::Win32::Security::TOKEN_QUERY] access rights.
/// * The security descriptor of the key does not permit the requested access for the calling process.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenuserclassesroot
///
pub fn open_user_classes_root(
    access_token_handle: isize,
    access: RegistryAccessRights,
) -> Result<isize> {
    call_WIN32_ERROR! {
        RegOpenUserClassesRoot(
            access_token_handle,
            0,
            access,
            &mut key_handle,
        ) -> mut key_handle: isize
    }
}

/// Enumerates the values for the open registry key specified by `handle`.
/// The function copies and returns the indexed registry value's name and
/// optionally its data block in a [`RegistryValueEnumInfo`].
///
/// If the function succeeds and no more value is available, the return value
/// is `None`.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][enum_values#allowed-predefined-keys] section.
/// * `index`: The index of the value to be retrieved.
///     * This parameter should be `0` for the first call to [`enum_values`] and then incremented
///       for subsequent calls.
/// * `name_buffer`: The buffer used to retreive the name of the value in bytes.
/// * `data_buffer`: The buffer used to retreive the data of the value in bytes.
///     * If this parameter is `None`, the returned information's `value` field contains the default
///       value of [`RegistryValue`].
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
/// * [`ERROR_MORE_DATA`][`windows_sys::Win32::Foundation::ERROR_MORE_DATA`]
///     * `name_buffer` is too small to receive the name of the registry value.
///     * If `data_buffer` contains a buffer that is too small to receive class of the registry value.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumvaluew
///
pub fn enum_values(
    handle: isize,
    index: u32,
    name_buffer: &mut [u16],
    mut data_buffer: Option<&mut [u8]>,
) -> Result<Option<RegistryValueEnumInfo>> {
    let mut enum_info = RegistryValueEnumInfo::default();
    let mut name_len = name_buffer.len() as u32;
    let mut data_len = data_buffer.as_ref().map_or(0, |buffer| buffer.len() as u32);
    call_WIN32_ERROR! {
        RegEnumValueW(
            handle,
            index,
            name_buffer.as_mut_ptr(),
            &mut name_len,
            ptr::null(),
            &mut enum_info.value.value_type,
            data_buffer.as_mut().map_or(ptr::null_mut(), |buffer| buffer.as_mut_ptr()),
            &mut data_len,
        ) != ERROR_NO_MORE_ITEMS => None else return Error
    };
    enum_info.name =
        // Safety: `name_buffer` is a valid null-terminated string with `name_len` + 1 length and has no interior null values.
        unsafe { U16CString::from_ptr_unchecked(name_buffer.as_ptr(), name_len as usize) };
    if let Some(buffer) = data_buffer {
        enum_info.value.data = buffer[..data_len as usize].to_vec().into_boxed_slice();
    }
    Ok(Some(enum_info))
}

/// Enumerates the subkeys of the specified open registry key. The function
/// retrieves information about one subkey each time it is called.
///
/// If the function succeeds and no more subkey is available, the return value
/// is `None`.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][enum_subkeys#allowed-predefined-keys] section.
/// * `index`: The index of the value to be retrieved.
///     * This parameter should be `0` for the first call to [`enum_subkeys`] and then incremented
///       for subsequent calls.
/// * `name_buffer`: The buffer used to retreive the name of the subkey in bytes.
/// * `class_buffer`: An optional buffer used to retreive the class of the subkey in bytes.
///     * If this parameter is `None`, the returned information's `class` field will contain an empty string.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have the [`KEY_ENUMERATE_SUB_KEYS`] access right.
/// * [`ERROR_MORE_DATA`][`windows_sys::Win32::Foundation::ERROR_MORE_DATA`]
///     * `name_buffer` is too small to receive the name of the registry subkey.
///     * If `name_buffer` contains a buffer that is too small to receive class of the registry subkey.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyexw
///
pub fn enum_subkeys(
    handle: isize,
    index: u32,
    name_buffer: &mut [u16],
    mut class_buffer: Option<&mut [u16]>,
) -> Result<Option<RegistryKeyEnumInfo>> {
    let mut enum_info = RegistryKeyEnumInfo::default();
    let mut name_len = name_buffer.len() as u32;
    let mut class_len = class_buffer
        .as_ref()
        .map_or(0, |buffer| buffer.len() as u32);
    call_WIN32_ERROR! {
        RegEnumKeyExW(
            handle,
            index,
            name_buffer.as_mut_ptr(),
            &mut name_len,
            ptr::null(),
            class_buffer.as_mut().map_or(ptr::null_mut(), |buffer| buffer.as_mut_ptr()),
            &mut class_len,
            &mut enum_info.last_write_time,
        ) != ERROR_NO_MORE_ITEMS => None else return Error
    };
    // Safety: `name_buffer` is a valid null-terminated string with `name_len` + 1 length and has no interior null values.
    enum_info.name =
        unsafe { U16CString::from_ptr_unchecked(name_buffer.as_ptr(), name_len as usize) };
    if let Some(buffer) = class_buffer {
        // Safety: `class_buffer` is a valid null-terminated string with `class_len` + 1 length and has no interior null values.
        enum_info.class =
            unsafe { U16CString::from_ptr_unchecked(buffer.as_ptr(), class_len as usize) };
    }
    Ok(Some(enum_info))
}

/// Gets the length of the class of the key specified by `handle`.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_class_name_length#allowed-predefined-keys] section.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
pub fn get_class_name_length(handle: isize) -> Result<u32> {
    call_WIN32_ERROR! {
        RegQueryInfoKeyW(
            handle,
            ptr::null_mut(),
            &mut class_name_len,
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        ) -> mut class_name_len: u32
    }
}

/// Gets the class of the key specified by `handle`.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_class#allowed-predefined-keys] section.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
pub fn get_class(handle: isize) -> Result<U16CString> {
    let mut class_length = get_class_name_length(handle)? + 1;
    if class_length == 1 {
        return Ok(U16CString::new());
    }

    let mut buffer = vec![0; class_length as usize];
    call_WIN32_ERROR! {
        RegQueryInfoKeyW(
            handle,
            buffer.as_mut_ptr(),
            &mut class_length,
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        ) return Error
    };

    // Safety: `buffer` contains no interior null values
    Ok(unsafe { U16CString::from_vec_unchecked(buffer) })
}

/// Gets and copies the class of the key specified by `handle` into `buffer`.
/// The return value indicates the amount of characters (`u16`) that were copied to `buffer`.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_class#allowed-predefined-keys] section.
/// * `buffer`: The buffer that receives the wide character bytes of the key's class string.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not big enough to contain the characters of the class string
///       without counting the null character
///
/// # Examples
///
/// TODO
///
pub fn get_class_with_buffer(handle: isize, buffer: &mut [u16]) -> Result<u32> {
    call_WIN32_ERROR! {
        RegQueryInfoKeyW(
            handle,
            buffer.as_mut_ptr(),
            &mut class_length,
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        ) -> mut class_length = buffer.len() as u32
    }
}

/// Gets [`information`][`RegistryKeyInfo`] about the registry key specified by `handle`.
///
/// Note that the string lengths in the information do not include the terminating null
/// character.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_info#allowed-predefined-keys] section.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryinfokeyw
///
pub fn get_info(handle: isize) -> Result<RegistryKeyInfo> {
    let mut class_len = get_class_name_length(handle)? + 1;
    let mut reg_info = RegistryKeyInfo {
        handle,
        ..Default::default()
    };
    let mut buffer = vec![0; class_len as usize];
    call_WIN32_ERROR! {
        RegQueryInfoKeyW(
            handle,
            buffer.as_mut_ptr(),
            &mut class_len,
            ptr::null(),
            &mut reg_info.subkey_count,
            &mut reg_info.max_sub_key_name_len,
            &mut reg_info.max_sub_key_class_len,
            &mut reg_info.value_count,
            &mut reg_info.max_value_name_len,
            &mut reg_info.max_value_len,
            &mut reg_info.security_descriptor_size,
            &mut reg_info.last_write_time,
        ) return Error
    };
    reg_info.handle = handle;
    // Safety: `buffer` contains no interior null values
    reg_info.class = unsafe { U16CString::from_vec_unchecked(buffer) };
    Ok(reg_info)
}

/// Gets [`information`][`RegistryKeyInfo`] about the registry key specified by `handle` that does not include the
/// actual class name in [`RegistryKeyInfo`] instance just an empty string as opposed to [`get_info`].
///
/// Note that the string lengths in the information do not include the terminating null
/// character.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_info_without_class#allowed-predefined-keys] section.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryinfokeyw
///
pub fn get_info_without_class(handle: isize) -> Result<RegistryKeyInfo> {
    call_WIN32_ERROR! {
        RegQueryInfoKeyW(
            handle,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null(),
            &mut reg_info.subkey_count,
            &mut reg_info.max_sub_key_name_len,
            &mut reg_info.max_sub_key_class_len,
            &mut reg_info.value_count,
            &mut reg_info.max_value_name_len,
            &mut reg_info.max_value_len,
            &mut reg_info.security_descriptor_size,
            &mut reg_info.last_write_time,
        ) -> mut reg_info = RegistryKeyInfo { handle, ..Default::default() }
    }
}

/// Gets the type and data for the specified registry value, copies the retrieved
/// data to the provided `buffer` and returns the type and size of the copied data
/// in a `(u32, u32)`.
///
/// See [Registry value types][get_value_with_buffer#registry-value-types] for more information about the possible registry types.
///
/// If `handle` is [`HKEY_PERFORMANCE_DATA`], the size of the peformance data can
/// change from one call to the next. In this case, you must increase the buffer's
/// size and call the function again with the updated buffer.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_value_with_buffer#allowed-predefined-keys] section.
/// * `subkey`: The path of a registry key relative to the key specified by `handle`
///     * If this parameter is
///         * an empty string, the value will be read from the key specified by `handle`.
///         * not an empty string, the value will be read from `subkey`.
/// * `value_name`: The name of the registry key value.
///     * If this parameter is an empty string, the function retrieves the type and
///       data for the key's unnamed or default value. Keys do not automatically
///       have an unnamed or default value, and unnamed values can be of any type.
/// * `flags`: The flags that restrict the data type of value to be queried.
///     * See the [Flags][get_value_with_buffer#flags] section for possible values.
///     * If the type does not meet this criteria, the function fails.
/// * `buffer`: The buffer where the retrieved data will be copied to.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_PERFORMANCE_NLSTEXT`]
/// * [`HKEY_PERFORMANCE_TEXT`]
/// * [`HKEY_USERS`]
///
/// ## Flags
///
/// | Flag | Meaning |
/// |--------|--------|
/// | [`RRF_RT_ANY`] | No type restriction. |
/// | [`RRF_RT_DWORD`] | Restrict type to 32-bit [`RRF_RT_REG_BINARY`] or [`RRF_RT_REG_DWORD`]. |
/// | [`RRF_RT_QWORD`] | Restrict type to 32-bit [`RRF_RT_REG_BINARY`] or [`RRF_RT_REG_QWORD`]. |
/// | [`RRF_RT_REG_BINARY`] | Restrict type to [`REG_BINARY`]. |
/// | [`RRF_RT_REG_DWORD`] | Restrict type to [`REG_DWORD`]. |
/// | [`RRF_RT_REG_EXPAND_SZ`] | Restrict type to [`REG_EXPAND_SZ`]. |
/// | [`RRF_RT_REG_MULTI_SZ`] | Restrict type to [`REG_MULTI_SZ`]. |
/// | [`RRF_RT_REG_NONE`] | Restrict type to [`REG_NONE`]. |
/// | [`RRF_RT_REG_QWORD`] | Restrict type to [`REG_QWORD`]. |
/// | [`RRF_RT_REG_SZ`] | Restrict type to [`REG_SZ`]. |
/// | [`RRF_NOEXPAND`] | Do not automatically expand environment strings if the value is of type [`REG_EXPAND_SZ`]. |
/// | [`RRF_ZEROONFAILURE`] | Set the contents of the buffer, that used to retrieve the data, to zeroes on failure. |
/// | [`RRF_SUBKEY_WOW6464KEY`] | If `subkey` is not an empty string, open the subkey that `subkey` specifies with the [`KEY_WOW64_64KEY`] access rights. You cannot specify [`RRF_SUBKEY_WOW6464KEY`] in combination with [`RRF_SUBKEY_WOW6432KEY`]. |
/// | [`RRF_SUBKEY_WOW6432KEY`] | If `subkey` is not an empty string, open the subkey that `subkey` specifies with the [`KEY_WOW64_32KEY`] access rights. You cannot specify [`RRF_SUBKEY_WOW6432KEY`] in combination with [`RRF_SUBKEY_WOW6464KEY`]. |
///
/// # Registry value types
///
/// A registry value can have the following data types:
///
/// | Type | Description |
/// |--------|--------|
/// | [`REG_BINARY`] | Binary data in any form. |
/// | [`REG_DWORD`] | A 32-bit number. |
/// | [`REG_DWORD_LITTLE_ENDIAN`] | A 32-bit number in little-endian format. Windows is designed to run on little-endian computer architectures. Therefore, this value is defined as [`REG_DWORD`] in the Windows header files.|
/// | [`REG_DWORD_BIG_ENDIAN`] | A 32-bit number in big-endian format. Some UNIX systems support big-endian architectures. |
/// | [`REG_EXPAND_SZ`] | A null-terminated string that contains unexpanded references to environment variables, for example, %PATH%. It's either a Unicode or an ANSI string, depending on whether you use the Unicode or ANSI functions. |
/// | [`REG_LINK`] | A null-terminated Unicode string that contains the target path of a symbolic link that was created by calling [`create`] with [`REG_OPTION_CREATE_LINK`]. |
/// | [`REG_MULTI_SZ`] | A sequence of null-terminated strings, terminated by an empty string (`\0`). The following is an example: `"String1\0String2\0String3\0LastString\0\0"`. The first `\0` terminates the first string, the second-from-last `\0` terminates the last string, and the final `\0` terminates the sequence. Note that the final terminator must be factored into the length of the string. |
/// | [`REG_NONE`] | No defined value type. |
/// | [`REG_QWORD`] | A 64-bit number. |
/// | [`REG_QWORD_LITTLE_ENDIAN`] | A 64-bit number in little-endian format. Windows is designed to run on little-endian computer architectures. Therefore, this value is defined as [`REG_QWORD`] in the Windows header files. |
/// | [`REG_SZ`] | A null-terminated string. It's either a Unicode or an ANSI string, depending on whether you use the Unicode or ANSI functions. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `subkey` does not exist.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is not large enough to contain contain the whole data of
///       the specified registry key's value.
/// * [`ERROR_FILE_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND]
///     * The `value_name` does not exist.
/// * [`ERROR_INVALID_PARAMETER`][windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER]
///     * `flags` specifies a combination of both [`RRF_SUBKEY_WOW6464KEY`]
///       and [`RRF_SUBKEY_WOW6432KEY`].
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew
///
pub fn get_value_with_buffer(
    handle: isize,
    subkey: &U16CStr,
    value_name: &U16CStr,
    flags: u32,
    buffer: &mut [u8],
) -> Result<(u32, u32)> {
    call_WIN32_ERROR! {
        RegGetValueW(
            handle,
            subkey.as_ptr(),
            value_name.as_ptr(),
            flags,
            &mut reg_value_type,
            buffer.as_mut_ptr().cast(),
            &mut reg_value_size,
        ) -> mut (reg_value_type, reg_value_size) = (0, buffer.len() as u32)
    }
}

/// Gets the type and data for the specified registry value and returns a [`RegistryValue`]
/// instance.
///
/// See [Registry value types][get_value_with_buffer#registry-value-types] for more information about the possible registry types.
///
/// If `handle` is [`HKEY_PERFORMANCE_DATA`], the size of the peformance data can
/// change from one call to the next. In this case, you must increase the buffer's
/// size and call the function again with the updated buffer.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_value_with_buffer#allowed-predefined-keys] section.
/// * `subkey`: The path of a registry key relative to the key specified by `handle`
///     * If this parameter is
///         * an empty string, the value will be read from the key specified by `handle`.
///         * not an empty string, the value will be read from `subkey`.
/// * `value_name`: The name of the registry key value.
///     * If this parameter is an empty string, the function retrieves the type and
///       data for the key's unnamed or default value. Keys do not automatically
///       have an unnamed or default value, and unnamed values can be of any type.
/// * `flags`: The flags that restrict the data type of value to be queried.
///     * See the [Flags][get_value_with_buffer#flags] section for possible values.
///     * If the type does not meet this criteria, the function fails.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * `subkey` does not exist.
/// * `handle` does not have the [`KEY_QUERY_VALUE`] access right.
/// * [`ERROR_FILE_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND]
///     * The `value_name` does not exist.
/// * [`ERROR_INVALID_PARAMETER`][windows_sys::Win32::Foundation::ERROR_INVALID_PARAMETER]
///     * `flags` specifies a combination of both [`RRF_SUBKEY_WOW6464KEY`]
///       and [`RRF_SUBKEY_WOW6432KEY`].
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew
///
pub fn get_value(
    handle: isize,
    subkey: &U16CStr,
    value_name: &U16CStr,
    flags: u32,
) -> Result<RegistryValue> {
    let mut reg_value = RegistryValue::default();
    let mut max_value_len = get_info_without_class(handle)?.max_value_len;
    let mut buffer = vec![0_u8; max_value_len as usize];
    call_WIN32_ERROR! {
        RegGetValueW(
            handle,
            subkey.as_ptr(),
            value_name.as_ptr(),
            flags,
            &mut reg_value.value_type,
            buffer.as_mut_ptr().cast(),
            &mut max_value_len,
        ) return Error
    };

    buffer.truncate(max_value_len as usize);
    reg_value.data = buffer.into_boxed_slice();
    Ok(reg_value)
}

/// Gets the total size of the registry values in bytes that can be used with [`get_multiple_values`]
/// to provide a buffer with sufficient size.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_multiple_values#allowed-predefined-keys] section.
///      * This parameter can be a handle to a remote key.
/// * `value_entries`: an array of [`VALENTW`] instances that describe one or more value entries.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * The total size of the requested data is more than the system limit of `1MB`.
/// * [`ERROR_CANTREAD`][windows_sys::Win32::Foundation::ERROR_CANTREAD]
///     * The function cannot instantiate or access the provider of the dynamic key.
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is too small to contain the values.
///
/// # Examples
///
/// See the [first example][get_multiple_values#examples] of [`get_multiple_values`].
///
pub fn get_multiple_values_size(handle: isize, value_entries: &mut [VALENTW]) -> Result<u32> {
    let mut size = 0;
    let result = unsafe {
        RegQueryMultipleValuesW(
            handle,
            value_entries.as_mut_ptr(),
            value_entries.len() as u32,
            ptr::null_mut(),
            &mut size,
        )
    };
    if result != ERROR_MORE_DATA && result != ERROR_SUCCESS {
        Err(Error::from_code(Code::from_win32(result)))
    } else {
        Ok(size)
    }
}

/// Gets the type and data for a list of value names associated with an open registry key, copies
/// each data into the provided buffer and returns the number of bytes written to `buffer`.
/// If the function succeeds each value entry in `value_entries` will be filled with the
/// retrieved information for the specified value.
///
/// This function allows an application to query one or more values of a static or dynamic key.
/// If the target key is a static key, the system provides all of the values in an atomic fashion.
/// To prevent excessive serialization, the aggregate data returned by the function cannot exceed
/// `1MB`.
///
/// If the target key is a dynamic key, its provider must provide all the values in an atomic fashion.
/// This means the provider should fill the results buffer synchronously, providing a consistent view
/// of all the values in the buffer while avoiding excessive serialization. The provider can provide
/// at most one megabyte of total output data during an atomic call to this function.
///
/// Use [`get_multiple_values_size`] to get the required buffer size for `buffer`.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][get_multiple_values#allowed-predefined-keys] section.
///     * This parameter can be a handle to a remote key.
/// * `value_entries`: an array of [`VALENTW`] instances that describe one or more value entries.
/// * `buffer`: The buffer that receives the data for each value.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * The total size of the requested data is more than the system limit of `1MB`.
/// * [`ERROR_CANTREAD`][windows_sys::Win32::Foundation::ERROR_CANTREAD]
///     * The function cannot instantiate or access the provider of the dynamic key.
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * `buffer` is too small to contain the values.
///
/// # Examples
///
/// Getting all values of a registry key:
/// ```
/// # use crate::core::error::Error;
/// # use crate::win32::system::registry::{
/// #    self, HKEY_LOCAL_MACHINE, KEY_ENUMERATE_SUB_KEYS, KEY_QUERY_VALUE, VALENTW,
/// # };
/// # use crate::{u16cstr, U16CStr};
///
/// # fn main() -> Result<(), Error> {
/// let subkey_name = u16cstr!("SYSTEM\\CurrentControlSet\\Services\\WINUSB");
/// let handle = registry::open(
///     HKEY_LOCAL_MACHINE,
///     subkey_name,
///     0,
///     KEY_ENUMERATE_SUB_KEYS | KEY_QUERY_VALUE,
/// )?;
/// let key_info = registry::get_info_without_class(handle)?;
/// let mut value_name_buffer = vec![0_u16; key_info.max_value_name_len as usize + 1];
/// let mut value_names = Vec::with_capacity(key_info.value_count as usize);
/// let mut i = 0;
/// while let Some(value_info) =
///     registry::enum_values(handle, i, value_name_buffer.as_mut_slice(), None)?
/// {
///     value_names.push(value_info.name);
///     i += 1;
/// }
/// // creating `VALENTW` instances for `get_multiple_values` and `get_multiple_values_size`
/// let mut value_entries: Vec<VALENTW> = value_names
///     .iter()
///     .map(|name| VALENTW {
///         ve_type: 0,
///         ve_valuename: name.as_ptr() as *mut u16,
///         ve_valuelen: 0,
///         ve_valueptr: 0,
///     })
///     .collect();
/// let buffer_size = registry::get_multiple_values_size(handle, value_entries.as_mut_slice())?;
/// let mut buffer = vec![0; buffer_size as usize];
/// let bytes_written =
///     registry::get_multiple_values(handle, value_entries.as_mut_slice(), buffer.as_mut_slice())?;
/// // the returned size should be the same as the `buffer`'s length
/// assert_eq!(buffer_size, bytes_written);
/// for i in 0..value_names.len() {
///     let value_name = value_names[i].to_string_lossy();
///     let value_ptr = value_entries[i].ve_valueptr as *mut u8;
///     match value_entries[i].ve_type {
///         registry::REG_SZ | registry::REG_EXPAND_SZ => println!(
///             "{}: {}",
///             value_name,
///             unsafe { U16CStr::from_ptr_str(value_ptr as *mut u16) }.to_string_lossy()
///         ),
///         registry::REG_DWORD | registry::REG_BINARY => {
///             let slice = unsafe {
///                 core::slice::from_raw_parts(value_ptr, value_entries[i].ve_valuelen as usize)
///             };
///             println!("{}: {:?}", value_name, slice); // printing the bytes of the data
///         }
///         _ => println!("{}: other data type", value_name), // ignoring other data types for now
///     }
/// }
/// // closing the handle to the registry key as it is no longer needed
/// registry::close(handle)?;
/// # Ok(())
/// # }
/// ```
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regquerymultiplevaluesw
///
pub fn get_multiple_values(
    handle: isize,
    value_entries: &mut [VALENTW],
    buffer: &mut [u8],
) -> Result<u32> {
    call_WIN32_ERROR! {
        RegQueryMultipleValuesW(
            handle,
            value_entries.as_mut_ptr(),
            value_entries.len() as u32,
            buffer.as_mut_ptr().cast(),
            &mut bytes_used,
        ) -> mut bytes_used = buffer.len() as u32
    }
}

/// Sets the data and type of a specified value under the registry key specified by `handle`.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][set_value#allowed-predefined-keys] section.
/// * `value_name`: The name of the value to be set.
///     * If a `value_name` is not already present in the key, the function add it
///       to the key.
///     * If this parameter is an empty string, the function sets the type and
///       data for the key's unnamed or default value.
/// * `value`: The data and its type to be set for the specified value.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_NLSTEXT`]
/// * [`HKEY_PERFORMANCE_TEXT`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `handle` is invalid.
/// * If `value_type` field of `value` is a [`REG_SZ`] and the `data` field of
///   the parameter is not null-terminated.
/// * If `value_type` field of `value` is a [`REG_MULTI_SZ`] and the `data` field of
///   the parameter is not double null-terminated.
/// * `handle` does not have the [`KEY_SET_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw
///
pub fn set_value(handle: isize, value_name: &U16CStr, value: &RegistryValue) -> Result<()> {
    call_WIN32_ERROR! {
        RegSetValueExW(
            handle,
            value_name.as_ptr(),
            0,
            value.value_type,
            value.data.as_ptr().cast(),
            value.data.len() as u32,
        )
    }
}

/// Sets the data for the value specified by `value_name` in the specified registry key and subkey.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][set_subkey_value#allowed-predefined-keys] section.
/// * `subkey_name`: The name of the subkey relative to the key identified by `key_handle`.
/// * `value_name`: The name of the value to be set.
///     * If a `value_name` is not already present in the key, the function add it
///       to the key.
///     * If this parameter is an empty string, the function sets the type and
///       data for the key's unnamed or default value.
/// * `value`: The data and its type to be set for the specified value.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_NLSTEXT`]
/// * [`HKEY_PERFORMANCE_TEXT`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `key_handle` is invalid.
/// * If `value_type` field of `value` is a [`REG_SZ`] and the `data` field of
///   the parameter is not null-terminated.
/// * If `value_type` field of `value` is a [`REG_MULTI_SZ`] and the `data` field of
///   the parameter is not double null-terminated.
/// * `key_handle` does not have the [`KEY_SET_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetkeyvaluew
///
pub fn set_subkey_value(
    key_handle: isize,
    subkey_name: &U16CStr,
    value_name: &U16CStr,
    value: &RegistryValue,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegSetKeyValueW(
            key_handle,
            subkey_name.as_ptr(),
            value_name.as_ptr(),
            value.value_type,
            value.data.as_ptr().cast(),
            value.data.len() as u32,
        )
    }
}

/// Establishes a connection to a predefined registry key on another computer and returns
/// the predefined handle on the remote computer.
///
/// This function requires the `Remote Registry` service to be running on the remote computer.
/// By default, this service is configured to be started manually.
/// To configure the `Remote Registry` service to start automatically, run `Services.msc` and change
/// the `Startup Type` of the service to `Automatic`.
///
/// If the returned handle is no longer needed, it should be closed by calling [`close`].
///
/// # Arguments
///
/// * `machine_name`: The name of the remote machine.
///     * The string has the following form: "\\computername".
///     * If this parameter is an empty string, the local computer name is used.
///     * The caller must have access to the remove computer.
/// * `predifined_key_handle`: One of the predefined keys listed in
///   [Allowed predefined keys][connect#allowed-predefined-keys] section.
/// * `value`: The data and its type to be set for the specified value.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `predifined_key_handle` is not a handle to a predefined key.
/// * The caller does not access to the remote computer specified by `machine_name`.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regconnectregistryw
///
pub fn connect(machine_name: &U16CStr, predifined_key_handle: isize) -> Result<isize> {
    call_WIN32_ERROR! {
        RegConnectRegistryW(
            machine_name.as_ptr(),
            predifined_key_handle,
            &mut remote_handle,
        ) -> mut remote_handle: isize
    }
}

/// Copies the specified registry key, along with its values and subkeys, to the destination key.
///
/// This function also copies the security descriptor for the key.
///
/// # Arguments
///
/// * `src_key_handle`: A handle to an open registry key or one of the predefined keys.
/// * `subkey_name`: The name of the subkey of the source key specified by `src_key_handle`.
///     * This parameter can be an empty string.
/// * `dest_key_handle`: A handle to the destination key. This parameter can be one of the predefined keys.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `src_key_handle` is invalid.
/// * `dest_key_handle` is invalid.
/// * `src_key_handle` does not have [`KEY_CREATE_SUB_KEY`] or [`KEY_READ`] access rights
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcopytreew
///
pub fn copy_tree(
    src_key_handle: isize,
    subkey_name: &U16CStr,
    dest_key_handle: isize,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegCopyTreeW(
            src_key_handle,
            subkey_name.as_ptr(),
            dest_key_handle
        )
    }
}

/// Creates the specified registry key and associates it with a transaction. If the key already exists,
/// the function opens it.
/// Note that key names are not case sensitive.
///
/// The returned value contains the opened or created handle to the specified key and one
/// of the following values:
/// * [`REG_CREATED_NEW_KEY`]: The key did not exist and was created.
/// * [`REG_OPENED_EXISTING_KEY`]: The key existed and was simply opened without being changed.
///
/// The key that the [`create_transacted`] creates has no values.
/// An application can use [`set_value`] to set key values.
///
/// Thia function creates all missing keys in the specified path. An application can take
/// advantage of this behavior to create several keys at once.
///
/// If the returned handle is not a predefined key and it is no longer needed,
/// it should be closed by calling [`close`].
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][create_transacted#allowed-predefined-keys] section.
/// * `subkey_name`: The name of the subkey that this function opens or creates
///     * If this parameter is an empty string, the returned key is a new handle to the key specified
///       by `key_handle`.
/// * `class`: The user-defined class of this key.
///     * This parameter is ignored if this parameter is an empty string.
/// * `options`: The options to use when opening or createing the specified key.
///     * See [Options][create#options] for possible values.
/// * `access`: Specifies the desired access rigths for the key to be created.
/// * `security_attributes`: An optional parameter that determines whether the returned handle can
///   be inheritied by child processes.
///     * If this parameter is
///         * `None`, the handle cannot be inherited and the key gets a default
///           security descriptor. The ACLs in a default security descriptor for a key are inherited from its
///           direct parent key.
///         * an instance of [`SECURITY_ATTRIBUTES`], the `lpSecurityDescriptor field` of the structure specifies a
///           security descriptor for the new key.
/// * `transaction_handle`: A handle to an active transaction that was created by `CreateTransaction`.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * `transaction_handle` is invalid.
/// * `key_handle` does not have [`KEY_CREATE_SUB_KEY`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeytransactedw
///
pub fn create_transacted(
    key_handle: isize,
    subkey_name: &U16CStr,
    class: &U16CStr,
    options: u32,
    access: RegistryAccessRights,
    security_attributes: Option<&SECURITY_ATTRIBUTES>,
    transaction_handle: isize,
) -> Result<(isize, u32)> {
    call_WIN32_ERROR! {
        RegCreateKeyTransactedW(
            key_handle,
            subkey_name.as_ptr(),
            0,
            class.as_ptr(),
            options,
            access,
            security_attributes.map_or(ptr::null(), |attr| attr),
            &mut key,
            &mut disposition,
            transaction_handle,
            ptr::null(),
        ) -> mut (key, disposition): (isize, u32)
    }
}

/// Deletes a subkey and its values from the specified platform-specific view of the registry.
/// Note that key names are not case sensitive.
///
/// A deleted key is not removed until the last handle to it is closed.
///
/// If the function succeeds, [`delete`] removes the specified key from the registry.
/// The entire key, including all of its values, is removed.
///
/// To delete a subkey as a transacted operation, call [`delete_transacted`].
///
/// The subkey to be deleted must not have subkeys. To delete a key and all its subkeys,
/// you need to enumerate the subkeys and delete them individually. To delete keys recursively,
/// use [`delete_tree`].
///
/// On `WOW64`, 32-bit applications view a registry tree that is separate from the registry tree
/// that 64-bit applications view. This function enables an application to delete an entry in the
/// alternate registry view.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///                 [Allowed predefined keys][delete#allowed-predefined-keys] section.
///     * The access rights of this key do not affect the delete operation.
/// * `subkey_name`: The name of the key to be deleted.
///     * This key must be a subkey of the key specified by `key_handle`.
///     * The function opens the subkey with [`DELETE`] access right.
/// * `access`: Specifies the platform-specific view of the registry.
///     * This parameter can have the following values:
///         * [`KEY_WOW64_32KEY`]: Delete the key from the 32-bit registry view.
///         * [`KEY_WOW64_64KEY`]: Delete the key from the 64-bit registry view.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * `subkey_name` is not a subkey of `key_handle`.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyexw
///
pub fn delete(
    key_handle: isize,
    subkey_name: &U16CStr,
    access: RegistryAccessRights,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegDeleteKeyExW(
            key_handle,
            subkey_name.as_ptr(),
            access,
            0,
        )
    }
}

/// Deletes a subkey and its values from the specified platform-specific view of the registry
/// as a transacted operation. Note that key names are not case sensitive.
///
/// A deleted key is not removed until the last handle to it is closed.
///
/// If the function succeeds, [`delete_transacted`] removes the specified key from the registry.
/// The entire key, including all of its values, is removed. To remove the entire tree as a
/// transacted operation, use [`delete_tree`] with a handle returned from [`create_transacted`]
/// or [`open_transacted`].
///
/// On `WOW64`, 32-bit applications view a registry tree that is separate from the registry tree
/// that 64-bit applications view. This function enables an application to delete an entry in the
/// alternate registry view.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][delete#allowed-predefined-keys] section.
///     * The access rights of this key do not affect the delete operation.
/// * `subkey_name`: The name of the key to be deleted.
///     * This key must be a subkey of the key specified by `key_handle`.
///     * The function opens the subkey with [`DELETE`] access right.
/// * `access`: Specifies the platform-specific view of the registry.
///     * This parameter can have the following values:
///         * [`KEY_WOW64_32KEY`]: Delete the key from the 32-bit registry view.
///         * [`KEY_WOW64_64KEY`]: Delete the key from the 64-bit registry view.
/// * `transaction_handle`: A handle to an active transaction that was created by `CreateTransaction`.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * `subkey_name` is not a subkey of `key_handle`.
/// * `transaction_handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeytransactedw
///
pub fn delete_transacted(
    key_handle: isize,
    subkey_name: &U16CStr,
    access: RegistryAccessRights,
    transaction_handle: isize,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegDeleteKeyTransactedW(
            key_handle,
            subkey_name.as_ptr(),
            access,
            0,
            transaction_handle,
            ptr::null(),
        )
    }
}

/// Removes a named value from the registry key specified by `handle`.
/// Note that value names are not case sensitive.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][delete_value#allowed-predefined-keys] section.
/// * `value_name`: The registry value to be removed.
///     * If this parameter is an empty string, the value set by [`set_value`] is removed.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
/// * `value_name` is invalid.
/// * `handle` does not have [`KEY_SET_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletevaluew
///
pub fn delete_value(handle: isize, value_name: &U16CStr) -> Result<()> {
    call_WIN32_ERROR! {
        RegDeleteValueW(
            handle,
            value_name.as_ptr(),
        )
    }
}

/// Removes the specified value from the specified registry key and subkey.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][delete_subkey_value#allowed-predefined-keys] section.
/// * `subkey_name`: The name of the registry key, that is subkey of `key_handle`.
/// * `value_name`: The registry value to be removed from the key.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * `value_name` is invalid.
/// * `key_handle` does not have [`KEY_SET_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyvaluew
///
pub fn delete_subkey_value(
    key_handle: isize,
    subkey_name: &U16CStr,
    value_name: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegDeleteKeyValueW(
            key_handle,
            subkey_name.as_ptr(),
            value_name.as_ptr(),
        )
    }
}

/// Deletes the subkeys and values of the specified key recursively.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][delete_tree#allowed-predefined-keys] section.
/// * `subkey_name`: The name of the registry key, that is subkey of `key_handle`.
///     * If this parameter is an empty string, subkeys and values of `key_handle` are
///       deleted.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * `key_handle` does not have [`DELETE`], [`KEY_ENUMERATE_SUB_KEYS`] and [`KEY_QUERY_VALUE`] access rights.
/// * If the specified key has values, it must have [`KEY_SET_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletetreew
///
pub fn delete_tree(key_handle: isize, subkey_name: &U16CStr) -> Result<()> {
    call_WIN32_ERROR! {
        RegDeleteTreeW(key_handle, subkey_name.as_ptr())
    }
}

/// Restores registry reflection for the disabled key specified by `handle`.
/// Restoring reflection for a key does not affect reflection of any subkeys.
///
/// # Arguments
///
/// * `handle`: A handle to the registry key that was previously disabled using
///   the [`disable_reflection`].
///     * This parameter cannot specify a key on a remote computer/
///     * If this key is not on the reflection list, this function
///       succeeds, but has no effect.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenablereflectionkey
///
pub fn enable_reflection(handle: isize) -> Result<()> {
    call_WIN32_ERROR! {
        RegEnableReflectionKey(handle)
    }
}

/// Disables registry reflection for the key specified by `handle`.
/// Disabling reflection for a key does not affect reflection of any subkeys.
///
/// To restore registry reflection for a disabled key, use [`enable_reflection`].
///
/// # Arguments
///
/// * `handle`: A handle to the registry key.
///     * This parameter cannot specify a key on a remote computer/
///     * If this key is not on the reflection list, this function
///       succeeds, but has no effect.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablereflectionkey
///
pub fn disable_reflection(handle: isize) -> Result<()> {
    call_WIN32_ERROR! {
        RegDisableReflectionKey(handle)
    }
}

/// Disables handle caching for all predefined registry handles for the current process.
///
/// This function does not work on remote computers.
///
/// Services that change impersonation should call this function before using any
/// of the predefined key handles. For example, any access of [`HKEY_CURRENT_USER`]
/// after this function is called results in open and close operations being
/// performed on `HKEY_USERS\SID_of_current_user`, or on `HKEY_USERS.DEFAULT` if the
/// current user's hive is not loaded.
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
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablepredefinedcacheex
///
pub fn disable_predefined_cache() -> Result<()> {
    call_WIN32_ERROR! {
        RegDisablePredefinedCacheEx()
    }
}

/// Writes all the attributes of the specified open registry key into the registry.
///
/// Calling [`flush`] function is an expensive operation that significantly affects
/// system-wide performance as it consumes disk bandwith and blocks modifications
/// to all keys by all processes in the registry hive that is being flushed until
/// flush operation completes. This function should only be called explicitly
/// when an application must guarantee that registry changes are persisted to
/// disk immediately after modification. All modifications made to keys are
/// visible to other processes without the need to flush them to disk.
///
/// Alternatively, the registry has a 'lazy flush' mechanism that flushes registry
/// modifications to disk at regular intervals of time. In addition to this regular
/// flush operation, registry changes are also flushed to disk at system shutdown.
/// Allowing the 'lazy flush' to flush registry changes is the most efficient way
/// to manage registry writes to the registry store on disk.
///
/// [`flush`] returns only when all the data for the hive that contains
/// the specified key has been written to the registry store on disk.
///
/// [`flush`] writes out the data for other keys in the hive that have
/// been modified since the last lazy flush or system start.
///
/// After [`flush`] returns, use [`close`] to close the handle to the registry
/// key.
///
/// # Arguments
///
/// * `handle`: A handle to the registry key or one of the predifined keys listed in
///             [Allowed predefined keys][flush#allowed-predefined-keys] section.
///     * This parameter cannot specify a key on a remote computer/
///     * If this key is not on the reflection list, this function
///       succeeds, but has no effect.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_PERFORMANCE_DATA`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
/// * `handle` does not have [`KEY_QUERY_VALUE`] access right.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regflushkey
///
pub fn flush(handle: isize) -> Result<()> {
    call_WIN32_ERROR! { RegFlushKey(handle) }
}

/// Creates a subkey under [`HKEY_USERS`] or [`HKEY_LOCAL_MACHINE`] and loads the data from the
/// specified registry hive into that subkey.
///
/// The functions always loads information at the top of the registry hierarchy.
///
/// There are two registry hive file formats. Registry hives created on current operating
/// systems typically cannot be loaded by earlier ones.
///
/// The calling process must have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
/// and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges on the computer in
/// which the registry resides.
///
/// # Arguments
///
/// * `key_handle`: A handle to the key where the subkey will be created or one of the predifined keys listed in
///             [Allowed predefined keys][load#allowed-predefined-keys] section.
///     * This parameter can be a handle returned by [`connect`].
/// * `subkey_name`: The name of the key to be created under `key_handle` where the registration
///                  information from the file will be loaded.
/// * `file_name`: The name of the file contatining the registry data. This file must be local
///                file that was created with [`save`].
///     * If this file does not exist, a file is created with the specified name.
///     * If `key_handle` is returned by [`connect`], then the path specified by this parameter
///       is relative to the remote computer.
///
///
/// ## Allowed predefined keys
///
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * The calling process does not have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
///   and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges enabled on the computer in
///   which the registry resides.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regloadkeyw
///
pub fn load(key_handle: isize, subkey_name: &U16CStr, file_name: &U16CStr) -> Result<()> {
    call_WIN32_ERROR! {
        RegLoadKeyW(
            key_handle,
            subkey_name.as_ptr(),
            file_name.as_ptr(),
        )
    }
}

/// Gets the length of the specified MUI string from the specified key and subkey.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][load_mui_string#allowed-predefined-keys] section.
/// * `value_name`: The name of the registry value.
/// * `directory`: The directory path.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `key_handle` does not have [`KEY_QUERY_VALUE`] access right.
/// * `value_name` does not exist.
///
/// # Examples
///
/// TODO
///
pub fn get_mui_string_len(
    key_handle: isize,
    value_name: &U16CStr,
    directory: &U16CStr,
) -> Result<u32> {
    let mut size = 0;
    let result = unsafe {
        RegLoadMUIStringW(
            key_handle,
            value_name.as_ptr(),
            ptr::null_mut(),
            0,
            &mut size,
            0,
            directory.as_ptr(),
        )
    };
    if result != ERROR_MORE_DATA && result != ERROR_SUCCESS {
        Err(Error::from_code(Code::from_win32(result)))
    } else {
        Ok(size / size_of::<u16>() as u32)
    }
}

/// Loads and returns the specified MUI string from the specified key and subkey.
///
/// Strings of the following form receive special handling:
///
/// `@[path]\dllname,-strID`
///
/// The string with identifier strID is loaded from `dllname` and the path is optional.
/// If `directory` parameter is not an empty string, the directory is prepended to the path
/// specified in the registry data. Note that `dllname` can contain environment variables
/// to be expanded.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][load_mui_string#allowed-predefined-keys] section.
/// * `value_name`: The name of the registry value.
/// * `directory`: The directory path.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `key_handle` does not have [`KEY_QUERY_VALUE`] access right.
/// * `value_name` does not exist.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regloadmuistringw
///
pub fn load_mui_string(
    key_handle: isize,
    value_name: &U16CStr,
    directory: &U16CStr,
) -> Result<U16CString> {
    let mut buffer_len = get_mui_string_len(key_handle, value_name, directory)?;
    let mut buffer = vec![0; buffer_len as usize];
    call_WIN32_ERROR! {
        RegLoadMUIStringW(
            key_handle,
            value_name.as_ptr(),
            buffer.as_mut_ptr(),
            buffer_len * size_of::<u16>() as u32,
            &mut buffer_len,
            0,
            directory.as_ptr(),
        ) return Error
    };
    // Safety: `buffer` does not contain interior null values.
    Ok(unsafe { U16CString::from_vec_unchecked(buffer) })
}

/// Loads the specified MUI string from the specified key and subkey, copies it into `buffer` and
/// returns the length of the copied string.
///
/// Use [`get_mui_string_len`] to get length of the MUI string in characters.
///
/// Strings of the following form receive special handling:
///
/// `@[path]\dllname,-strID`
///
/// The string with identifier strID is loaded from `dllname` and the path is optional.
/// If `directory` parameter is not an empty string, the directory is prepended to the path
/// specified in the registry data. Note that `dllname` can contain environment variables
/// to be expanded.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][load_mui_string#allowed-predefined-keys] section.
/// * `value_name`: The name of the registry value.
/// * `directory`: The directory path.
/// * `trancate`: Specifies whether the string is truncated to fit the size of `buffer`.
///     * If this parameter is `false` and `buffer` is not big enough to contain the
///       string, the function fails.
/// * `buffer`: The buffer where the loaded MUI string gets copied to.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `key_handle` does not have [`KEY_QUERY_VALUE`] access right.
/// * `value_name` does not exist.
/// * [`ERROR_MORE_DATA`]
///     * `buffer` is not big enough to contain the specified MUI string.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regloadmuistringw
///
pub fn load_mui_string_with_buffer(
    key_handle: isize,
    value_name: &U16CStr,
    directory: &U16CStr,
    trancate: bool,
    buffer: &mut [u16],
) -> Result<u32> {
    let mut string_size = 0;
    call_WIN32_ERROR! {
        RegLoadMUIStringW(
            key_handle,
            value_name.as_ptr(),
            buffer.as_mut_ptr(),
            (buffer.len() * size_of::<u16>()) as u32,
            &mut string_size,
            if trancate { REG_MUI_STRING_TRUNCATE } else { 0 },
            directory.as_ptr(),
        ) return Error
    };
    Ok(string_size / size_of::<u16>() as u32)
}

/// Notifies the caller about changes to the attributes or contents of the registry key specified
/// by `handle`. This function detects a single change. After the caller receives a notification
/// event, it should call the function again to receive the next notification.
///
/// [`notify_on_change`] should not be called multiple times with the same value for the hKey but
/// different values for the `watch_sub_tree` and `notify_filter` parameters. The function will
/// succeed but the changes will be ignored. To change the watch parameters, you must first close
/// the key handle by calling [`close`], reopen the key handle by calling [`open`], and then call
/// this function with the new parameters. Each time a process calls this function with the same
/// set of parameters, it establishes another wait operation, creating a resource leak. Therefore,
/// check that you are not calling this function with the same parameters until the previous wait
/// operation has completed.
///
/// This function cannot be used to detect changes to the registry that result from using
/// [`restore`].
///
/// If the specified key is closed, the event is signaled. This means that an application should
/// not depend on the key being open after returning from a wait operation on this event.
///
/// The [`REG_NOTIFY_THREAD_AGNOSTIC`] flag introduced in Windows 8 enables the use of [`notify_on_change`]
/// for `ThreadPool` threads.
///
/// If the thread that called [`notify_on_change`] exits, the event is signaled. To continue
/// to monitor additional changes in the value of the key, call this function again
/// from another thread.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][notify_on_change#allowed-predefined-keys] section.
///     * This parameter must be a local handle.
/// * `watch_sub_tree`: Specifies whether the function should report changes in the specified key's
///                     subkeys.
/// * `notify_filter`: A value that indicates the changes that should be reported.
///     * See [Notify filter flags][notify_on_change$notify-filter-flags] for possinle values.
/// * `event_handle`: A handle to an event.
///     * If `call_async` parameter is `false`, this parameter is ignored.
///     * If this parameter does not specify a valid event, `call_async` cannot be `true`.
/// * `call_async`: Specifies whether to call this function asynchronously.
///     * If this parameter is
///         * `true`, the function returns immediately and reports changes by signaling the
///           specified event.
///         * `false`, the function does not return until a change has occurred.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// ## Notify filters flags
///
/// | Filter | Description |
/// |--------|-------------|
/// | [`REG_NOTIFY_CHANGE_NAME`] | Notify the caller if a subkey is added or deleted. |
/// | [`REG_NOTIFY_CHANGE_ATTRIBUTES`] | Notify the caller of changes to the attributes of the key, such as the security descriptor information. |
/// | [`REG_NOTIFY_CHANGE_LAST_SET`] | Notify the caller of changes to a value of the key. This can include adding or deleting a value, or changing an existing value. |
/// | [`REG_NOTIFY_CHANGE_SECURITY`] | Notify the caller of changes to the security descriptor of the key. |
/// | [`REG_NOTIFY_THREAD_AGNOSTIC`] | Indicates that the lifetime of the registration must not be tied to the lifetime of the thread issuing the [`notify_on_change`] call. This flag value is only supported in Windows 8 and later. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` does not have [`KEY_NOTIFY`] access right.
/// * [`ERROR_INVALID_HANDLE`][windows_sys::Win32::Foundation::ERROR_INVALID_HANDLE]
///     * `handle` is invalid.
///     * `handle` is a remote handle.
/// * `event_handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regnotifychangekeyvalue
///
pub fn notify_on_change(
    handle: isize,
    watch_sub_tree: bool,
    notify_filter: u32,
    event_handle: isize,
    call_async: bool,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegNotifyChangeKeyValue(
            handle,
            to_BOOL!(watch_sub_tree),
            notify_filter,
            event_handle,
            to_BOOL!(call_async),
        )
    }
}

/// Maps a predefined registry key to the specified registry key.
///
/// This function function is intended for software installation programs. It allows them to remap
/// a predefined key, load a DLL component that will be installed on the system, call an entry point in the
/// DLL, and examine the changes to the registry that the component attempted to make. The installation program
/// can then write those changes to the locations intended by the DLL, or make changes to the data before
/// writing it.
///
/// After the call to [`override_predefined`], you can safely call [`close`] to close `new_key_handle`,
/// because the system maintains its own reference to it.
///
/// # Arguments
///
/// * `predifined_key_handle`: A handle to one of the following predefined keys:
///     * [`HKEY_CLASSES_ROOT`]
///     * [`HKEY_CURRENT_CONFIG`]
///     * [`HKEY_CURRENT_USER`]
///     * [`HKEY_LOCAL_MACHINE`]
///     * [`HKEY_PERFORMANCE_DATA`]
///     * [`HKEY_USERS`]
/// * `new_key_handle`: A handle to an open registry key.
///     * It cannot be one of the predefined keys.
///     * The function maps `predifined_key_handle` to refer to `new_key_handle`. This
///       affect only the calling process.
///     * If this parameter is `0`, the function restores the default mapping of the
///       predefined key.    
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `predifined_key_handle` is invalid.
/// * `new_key_handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regoverridepredefkey
///
pub fn override_predefined(predifined_key_handle: isize, new_key_handle: isize) -> Result<()> {
    call_WIN32_ERROR! { RegOverridePredefKey(predifined_key_handle, new_key_handle) }
}

/// Indicates whether reflection has been disabled through [`disable_reflection`].
///
/// To disable registry reflection, use [`disable_reflection`] and
/// to restore reflection for a disabled key, use [`enable_reflection`].
///
/// # Arguments
///
/// * `key_handle`: A handle to the registry key.
///     * The parameter cannot specify a key on a remote computer.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryreflectionkey
///
pub fn is_reflection_disabled(handle: isize) -> Result<bool> {
    let mut is_reflection_disabled: i32 = 0;
    call_WIN32_ERROR! {
        RegQueryReflectionKey(
            handle,
            &mut is_reflection_disabled,
        ) return Error
    };
    Ok(from_BOOL!(is_reflection_disabled))
}

/// Changes the name of the specified registry key.
///
/// This function can be used to rename an entire registry subtree. The caller must have
/// [`KEY_CREATE_SUB_KEY`] access to the parent of the specified key and [`DELETE`] access to
/// the entire subtree being renamed.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][replace#allowed-predefined-keys] section.
/// * `sub_key_name`: The name of the subkey to be renamed.
/// * `new_key_name`: The new name of the key that does not already exist.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * [`STATUS_ACCESS_DENIED`][windows_sys::Win32::Foundation::STATUS_ACCESS_DENIED]
///     * `key_handle` does not have [`KEY_WRITE`] access right.
///     * The caller does not have [`KEY_CREATE_SUB_KEY`] access to the parent of the specified key
///       and [`DELETE`] access to the entire subtree being renamed.
/// * `subkey_name` does not exist.
/// * `new_key_name` is an already existing key name.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrenamekey
///
pub fn rename(key_handle: isize, subkey_name: &U16CStr, new_key_name: &U16CStr) -> Result<()> {
    call_WIN32_ERROR! {
        RegRenameKey(key_handle, subkey_name.as_ptr(), new_key_name.as_ptr())
    }
}

/// Replaces the file backing a registry key and all its subkeys with another file, so that when
/// the system is next started, the key and subkeys will have the values stored in the new file.
///
/// There are two different registry hive file formats. Registry hives created on current operating
/// systems typically cannot be loaded by earlier ones.
///
/// The calling process must have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
/// and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges enabled on the
/// computer in which the registry resides.
///
/// # Arguments
///
/// * `key_handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][replace#allowed-predefined-keys] section.
///     * If this parameter is a handle returned by [`connect`], then the paths specified in
///       `new_file` and old_file` are relative to the remote computer.
/// * `sub_key_name`: The name of the registry key whose subkeys and values are to be replaced.
///     * If the key exists, it must be a subkey of the key identified by `key_handle`.
///     * If the subkey does not exist, it is created.
///     * If the specified subkey is not the root of a hive, this function traverses up the hive
///       tree structure until it encounters a hive root, then it replaces the contents of that
///       hive with the contents of the data file specified by `new_file`.
/// * `new_file`: The name of the file with the registry information.
///     * This file is typically created by using [`save`].
/// * `old_file`: The name of the file that receives a backup copy of the registry information
///               being replaced.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `old_file` does not exist.
/// * `new_file` does not exist.
/// * The calling process does not have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
///   and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges enabled on the
///   computer in which the registry resides.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regreplacekeyw
///
pub fn replace(
    key_handle: isize,
    subkey_name: &U16CStr,
    new_file: &U16CStr,
    old_file: &U16CStr,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegReplaceKeyW(
            key_handle,
            subkey_name.as_ptr(),
            new_file.as_ptr(),
            old_file.as_ptr(),
        )
    }
}

/// Reads the registry information in the file specified by `file_name` and copies it over to the key
/// identified by `handle`. This registry information may be in the form of a key and multiple levels
/// of subkeys. The new information in the file specified by `file_name` overwrites the contents of
/// the key specified by the `handle` parameter, except for the key name.
///
/// There are two different registry hive file formats. Registry hives created on current operating
/// systems typically cannot be loaded by earlier ones.
///
/// If any subkeys of the `handle` parameter are open, [`restore`] fails.
///
/// The calling process must have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
/// and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges enabled on the
/// computer in which the registry resides.
///
/// # Arguments
///
/// * `handle`: A handle to the registry key to be unloaded or one of the predifined keys listed in
///             [Allowed predefined keys][restore#allowed-predefined-keys] section.
///     * Any information contained in this key and its descendent keys is overwriteen by the
///       information in the file specified by the `file_name` parameter.
/// * `file_name`: The name of the file with the registry information.
///     * The file is typically created using [`save`].
///     * If `handle` represents a key in a remote computer, the path described by this parameter
///       is relative to the remote computer.
/// * flags: The flags that indicate how the key or keys are to be restored.
///     * See [Flags][restore#restore-flags] for possible values.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_CONFIG`]
/// * [`HKEY_CURRENT_USER`]
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// ## Restore flags
///
/// | Flag | Meaning |
/// |------|---------|
/// | [`REG_FORCE_RESTORE`] | If specified, the restore operation is executed even if open handles exist at or beneath the location in the registry hierarchy which the `handle` parameter specifies. |
/// | [`REG_WHOLE_HIVE_VOLATILE`] | If specified, a new, volatile (memory only) set of registry information, or hive, is created. If [`REG_WHOLE_HIVE_VOLATILE`] is specified, the key identified by the `handle` parameter must be either the [`HKEY_USERS`] or [`HKEY_LOCAL_MACHINE`] value. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `file_name` does not exist.
/// * The calling process does not have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
///   and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges enabled on the
///   computer in which the registry resides.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrestorekeyw
///
pub fn restore(handle: isize, file_name: &U16CStr, flags: i32) -> Result<()> {
    call_WIN32_ERROR! {
        RegRestoreKeyW(
            handle,
            file_name.as_ptr(),
            flags,
        )
    }
}

/// Saves the specified key and all of its subkeys and values to a new file, in the standard format.
///
/// To specify the format for the saved key or hive, use [`save_with_flags`].
/// Unlike [`save_with_flags`], this function supports the [`HKEY_CLASSES_ROOT`] predefined key.
///
/// This function saves only nonvolatile keys. It does not save volatile keys. A key is made volatile or
/// nonvolatile at its creation.
///
/// You can use the file created by [`save`] in subsequent calls to the [`load`], [`replace`],
/// or [`restore`] function. If [`save`] fails partway through its operation, the file will
/// be corrupt and subsequent calls to [`load`], [`replace`], or [`restore`] for the file will fail.
///
/// Using [`save`] together with [`restore`] to copy subtrees in the registry is not recommended.
/// This function does not trigger notifications and can invalidate handles used by other applications.
/// Instead, use [`copy_tree`].
///
/// The calling process must have the [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME]
/// privilege enabled.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or one of the predifined keys listed in
///             [Allowed predefined keys][save#allowed-predefined-keys] section.
/// * `file_name`: The name of the file in which the specified key and subkeys are to be saved.
///     * If the file already exists, the functions fails.
///     * The new file has the archive attribute.
///     * If the string does not include a path, the file is created in the current directory
///       of the calling process for a local key, or in the `%systemroot%\system32` directory for
///       a remote key.
/// * `security_attributes`: An optional parameter that specifies a security descriptor for the new file.
///     * If this parameter is `None`, the file gets a default security descriptor. The ACLs in a default
///       security descriptor for a file are inherited from its parent directory.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_CLASSES_ROOT`]
/// * [`HKEY_CURRENT_USER`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
/// * If the `file_name` parameter specifies an existing file.
/// * The calling process does not have the [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME]
///   privilege enabled.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyw
///
pub fn save(
    handle: isize,
    file_name: &U16CStr,
    security_attributes: Option<&SECURITY_ATTRIBUTES>,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegSaveKeyW(
            handle,
            file_name.as_ptr(),
            security_attributes.map_or(ptr::null(), |attr| attr),
        )
    }
}

/// Saves the specified key and all of its subkeys and values to a registry file, in the specified format.
///
/// Unlike [`save`], this function does not support the [`HKEY_CLASSES_ROOT`] predefined key.
///
/// This function saves only nonvolatile keys. It does not save volatile keys. A key is made volatile or
/// nonvolatile at its creation.
///
/// You can use the file created by [`save_with_flags`] in subsequent calls to the [`load`], [`replace`],
/// or [`restore`] function. If [`save_with_flags`] fails partway through its operation, the file will
/// be corrupt and subsequent calls to [`load`], [`replace`], or [`restore`] for the file will fail.
///
/// Using [`save_with_flags`] together with [`restore`] to copy subtrees in the registry is not recommended.
/// This function does not trigger notifications and can invalidate handles used by other applications.
/// Instead, use [`copy_tree`].
///
/// The calling process must have the [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME]
/// privilege enabled.
///
/// # Arguments
///
/// * `handle`: A handle to an open registry key or [`HKEY_CURRENT_USER`].
/// * `file_name`: The name of the file in which the specified key and subkeys are to be saved.
///     * If the file already exists, the functions fails.
///     * The new file has the archive attribute.
///     * If the string does not include a path, the file is created in the current directory
///       of the calling process for a local key, or in the `%systemroot%\system32` directory for
///       a remote key.
/// * `security_attributes`: An optional parameter that specifies a security descriptor for the new file.
///     * If this parameter is `None`, the file gets a default security descriptor. The ACLs in a default
///       security descriptor for a file are inherited from its parent directory.
/// * flags: The format of the saved key or hive.
///     * See [Format flags][save_with_flags#format-flags] for possible values.
///
/// ## Format flags
///
/// | Flag | Meaning |
/// |------|---------|
/// | [`REG_STANDARD_FORMAT`] | The key or hive is saved in standard format. The standard format is the only format supported by Windows 2000. |
/// | [`REG_LATEST_FORMAT`] | The key or hive is saved in the latest format. The latest format is supported starting with Windows XP. After the key or hive is saved in this format, it cannot be loaded on an earlier system. |
/// | [`REG_NO_COMPRESSION`] | The hive is saved with no compression, for faster save operations. The `handle` parameter must specify the root of a hive under [`HKEY_LOCAL_MACHINE`] or [`HKEY_USERS`]. For example, `HKLM\SOFTWARE` is the root of a hive. |
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `handle` is invalid.
/// * `handle` is the [`HKEY_CLASSES_ROOT`] predefined key.
/// * If the `file_name` parameter specifies an existing file.
/// * The calling process does not have the [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME]
///   privilege enabled.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyexw
///
pub fn save_with_flags(
    handle: isize,
    file_name: &U16CStr,
    security_attributes: Option<&SECURITY_ATTRIBUTES>,
    flags: u32,
) -> Result<()> {
    call_WIN32_ERROR! {
        RegSaveKeyExW(
            handle,
            file_name.as_ptr(),
            security_attributes.map_or(ptr::null(), |attr| attr),
            flags,
        )
    }
}

/// Unloads the specified registry key specified and its subkeys from the registry.
///
/// This function removes a hive from the registry but does not modify the file
/// containing the registry information. A hive is a discrete body of keys, subkeys,
/// and values that is rooted at the top of the registry hierarchy.
///
/// The calling process must have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
/// and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges enabled on the
/// computer in which the registry resides.
///
/// # Arguments
///
/// * `key_handle`: A handle to the registry key to be unloaded or one of the predifined keys listed in
///             [Allowed predefined keys][unload#allowed-predefined-keys] section.
///     * This parameter can be a handle returned by [`connect`].
/// * `subkey_name`: The name of the subkey to be unloaded.
///     * The key referred to by this parameter must have been created by using [`load`].
///     * Key names are not case sensitive.
///
/// ## Allowed predefined keys
///
/// * [`HKEY_LOCAL_MACHINE`]
/// * [`HKEY_USERS`]
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `key_handle` is invalid.
/// * `subkey_name` does not exist.
/// * The calling process does not have the [`SE_RESTORE_NAME`][windows_sys::Win32::Security::SE_RESTORE_NAME]
///   and [`SE_BACKUP_NAME`][windows_sys::Win32::Security::SE_BACKUP_NAME] privileges enabled on the computer in
///   which the registry resides.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regunloadkeyw
///
pub fn unload(key_handle: isize, subkey_name: &U16CStr) -> Result<()> {
    call_WIN32_ERROR! { RegUnLoadKeyW(key_handle, subkey_name.as_ptr()) }
}
