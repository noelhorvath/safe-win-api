use crate::common::ToDebugString;
use crate::win32::core::Result;
use crate::{call_WIN32_ERROR, default_sized};
use core::ptr;
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::FILETIME;
use windows_sys::Win32::System::Registry::{
    RegCloseKey, RegEnumKeyExW, RegEnumValueW, RegOpenKeyExW, RegQueryInfoKeyW, RegQueryValueExW,
};

pub use windows_sys::Win32::System::Registry::{
    HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_CURRENT_USER_LOCAL_SETTINGS, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA,
    HKEY_PERFORMANCE_NLSTEXT, HKEY_PERFORMANCE_TEXT, HKEY_USERS, KEY_ALL_ACCESS, KEY_CREATE_LINK,
    KEY_CREATE_SUB_KEY, KEY_ENUMERATE_SUB_KEYS, KEY_EXECUTE, KEY_NOTIFY, KEY_QUERY_VALUE, KEY_READ,
    KEY_SET_VALUE, KEY_WOW64_32KEY, KEY_WOW64_64KEY, KEY_WOW64_RES, KEY_WRITE, REG_BINARY,
    REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_DWORD_LITTLE_ENDIAN, REG_EXPAND_SZ, REG_LINK,
    REG_MULTI_SZ, REG_NONE, REG_OPTION_OPEN_LINK, REG_QWORD, REG_QWORD_LITTLE_ENDIAN,
    REG_ROUTINE_FLAGS, REG_SAM_FLAGS as RegistryAccessRights, REG_SZ, REG_VALUE_TYPE,
};

pub struct RegistryKeyEnumInfo {
    pub name: U16CString,
    pub class: U16CString,
    pub last_write_time: FILETIME,
}

impl ::core::fmt::Debug for RegistryKeyEnumInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistryKeyEnumInfo")
            .field("name", &self.name)
            .field("class", &self.name)
            .field("last_write_time", &self.last_write_time.to_debug_string())
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
pub struct RegistryValue {
    pub data: Box<[u8]>,
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
pub struct RegistryValueEnumInfo {
    pub name: U16CString,
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

pub fn close_key(handle: isize) -> Result<()> {
    call_WIN32_ERROR! { RegCloseKey(handle) }
}

/// Opens an existing thread object.
///
/// # Arguments
///
/// * `id`: The thread identifier
/// * `access`: One or more [access rights][`RegistryAccessRights`] to the registry key.
/// * `inherit_handle`: Specifies whether processes created by this thread should inherit it's handle
///
/// # Result
///
/// An open handle to the specified thread.
///
/// # Remarks
///
/// * If the handle is not needed anymore close it using [`close_handle`](crate::win32::foundation::close_handle).
///  
/// # Errors
///
/// Returns a [`Win32Error`][crate::win32::core::Win32Error] if the function fails.
///
/// # Possible error
///
/// * `id` is not a valid thread identifier.
///
/// # Examples
///
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread
///
pub fn open_key(
    parent_key_handle: isize,
    sub_key_name: Option<&U16CStr>,
    options: u32,
    access: RegistryAccessRights,
) -> Result<isize> {
    call_WIN32_ERROR! {
        RegOpenKeyExW(
            parent_key_handle,
            sub_key_name.map_or(ptr::null(), |str| str.as_ptr()),
            options,
            access,
            &mut opened_key_handle,
        ) -> mut opened_key_handle: isize
    }
}

pub fn enum_values(
    handle: isize,
    index: u32,
    name_buffer: &mut [u16],
    value_buffer: &mut [u8],
) -> Option<RegistryValueEnumInfo> {
    let mut enum_info = RegistryValueEnumInfo::default();
    let mut name_len = name_buffer.len() as u32;
    let mut value_len = value_buffer.len() as u32;
    call_WIN32_ERROR! {
        RegEnumValueW(
            handle,
            index,
            name_buffer.as_mut_ptr(),
            &mut name_len,
            ptr::null(),
            &mut enum_info.value.value_type,
            value_buffer.as_mut_ptr(),
            &mut value_len,
        ) return Error as Option
    };
    enum_info.name =
        // Safety: `name_buffer` is a valid null-terminated string with `name_len` + 1 length and has no interior null values.
        unsafe { U16CString::from_ptr_unchecked(name_buffer.as_ptr(), name_len as usize) };
    enum_info.value.data = value_buffer[..value_len as usize]
        .to_vec()
        .into_boxed_slice();
    Some(enum_info)
}

pub fn enum_subkeys(
    handle: isize,
    index: u32,
    name_buffer: &mut [u16],
    class_buffer: &mut [u16],
) -> Option<RegistryKeyEnumInfo> {
    let mut enum_info = RegistryKeyEnumInfo::default();
    let mut name_len = name_buffer.len() as u32;
    let mut class_len = class_buffer.len() as u32;
    call_WIN32_ERROR! {
        RegEnumKeyExW(
            handle,
            index,
            name_buffer.as_mut_ptr(),
            &mut name_len,
            ptr::null(),
            class_buffer.as_mut_ptr(),
            &mut class_len,
            &mut enum_info.last_write_time,
        ) return Error as Option
    };
    // Safety: `name_buffer` is a valid null-terminated string with `name_len` + 1 length and has no interior null values.
    enum_info.name =
        unsafe { U16CString::from_ptr_unchecked(name_buffer.as_ptr(), name_len as usize) };
    // Safety: `class_buffer` is a valid null-terminated string with `class_len` + 1 length and has no interior null values.
    enum_info.class =
        unsafe { U16CString::from_ptr_unchecked(class_buffer.as_ptr(), class_len as usize) };
    Some(enum_info)
}

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

#[derive(Clone)]
pub struct RegistryKeyInfo {
    pub class: U16CString,
    pub sub_keys: u32,
    pub max_sub_key_len: u32,
    pub max_class_len: u32,
    pub values: u32,
    pub max_value_name_len: u32,
    pub max_value_len: u32,
    pub security_descriptor: u32,
    pub last_write_time: FILETIME,
}

impl ::core::fmt::Debug for RegistryKeyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistryKeyEnumInfo")
            .field("class", &self.class)
            .field("sub_keys", &self.sub_keys)
            .field("max_sub_key_len", &self.max_sub_key_len)
            .field("max_class_len", &self.max_class_len)
            .field("values", &self.values)
            .field("max_value_len", &self.max_value_len)
            .field("max_value_len", &self.max_value_len)
            .field("security_descriptor", &self.security_descriptor)
            .field("last_write_time", &self.last_write_time.to_debug_string())
            .finish()
    }
}

impl ::core::default::Default for RegistryKeyInfo {
    fn default() -> Self {
        RegistryKeyInfo {
            class: U16CString::new(),
            sub_keys: 0,
            max_sub_key_len: 0,
            max_class_len: 0,
            values: 0,
            max_value_name_len: 0,
            max_value_len: 0,
            security_descriptor: 0,
            last_write_time: default_sized!(FILETIME),
        }
    }
}

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

pub fn get_info(handle: isize) -> Result<RegistryKeyInfo> {
    let mut class_len = get_class_name_length(handle)? + 1;
    let mut reg_info = RegistryKeyInfo::default();
    let mut buffer = vec![0; class_len as usize];
    call_WIN32_ERROR! {
        RegQueryInfoKeyW(
            handle,
            buffer.as_mut_ptr(),
            &mut class_len,
            ptr::null(),
            &mut reg_info.sub_keys,
            &mut reg_info.max_sub_key_len,
            &mut reg_info.max_class_len,
            &mut reg_info.values,
            &mut reg_info.max_value_name_len,
            &mut reg_info.max_value_len,
            &mut reg_info.security_descriptor,
            &mut reg_info.last_write_time,
        ) return Error
    };
    // Safety: `buffer` contains no interior null values
    reg_info.class = unsafe { U16CString::from_vec_unchecked(buffer) };
    Ok(reg_info)
}

pub fn get_info_without_class(handle: isize) -> Result<RegistryKeyInfo> {
    call_WIN32_ERROR! {
        RegQueryInfoKeyW(
            handle,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null(),
            &mut reg_info.sub_keys,
            &mut reg_info.max_sub_key_len,
            &mut reg_info.max_class_len,
            &mut reg_info.values,
            &mut reg_info.max_value_name_len,
            &mut reg_info.max_value_len,
            &mut reg_info.security_descriptor,
            &mut reg_info.last_write_time,
        ) -> mut reg_info: RegistryKeyInfo
    }
}

pub fn get_raw_value(
    handle: isize,
    value_name: Option<&U16CStr>,
    mut max_value_len: u32,
) -> Result<RegistryValue> {
    let mut reg_value = RegistryValue::default();
    let mut buffer = vec![0; max_value_len as usize];
    call_WIN32_ERROR! {
        RegQueryValueExW(
            handle,
            value_name.map_or(ptr::null(), |name| name.as_ptr()),
            ptr::null(),
            &mut reg_value.value_type,
            buffer.as_mut_ptr(),
            &mut max_value_len,
        ) return Error
    };

    buffer.truncate(max_value_len as usize);
    reg_value.data = buffer.into_boxed_slice();
    Ok(reg_value)
}
