use core::slice;
use std::ffi::{OsStr, OsString};
use std::marker::PhantomData;
use std::os::windows::prelude::OsStringExt;
use std::time::SystemTime;
use win_api_wrapper::core::{pwstr_to_boxed_osstr, WinError};
use win_api_wrapper::win32::system::registry::{
    self, RegistryAccessRights, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_CURRENT_USER_LOCAL_SETTINGS, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA,
    HKEY_PERFORMANCE_NLSTEXT, HKEY_PERFORMANCE_TEXT, HKEY_USERS, KEY_ENUMERATE_SUB_KEYS,
    KEY_QUERY_VALUE, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_ENUM_VALUE_INFO,
    REG_EXPAND_SZ, REG_KEY_INFO, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD, REG_SZ, REG_VALUE,
    REG_VALUE_TYPE,
};
use win_api_wrapper::win32::system::time::file_time_to_std_system_time;

use crate::common::As;

type Result<T> = ::core::result::Result<T, RegistryError>;

#[derive(Debug)]
pub enum RegistryError {
    WinError {
        code: i32,
        message: String,
    },
    CreateEnumeratorError {
        key: RegistryKey,
        code: i32,
        message: String,
    },
    InvalidValueType,
    InvalidValueSize,
}

impl From<WinError> for RegistryError {
    fn from(value: WinError) -> Self {
        Self::WinError {
            code: value.code().0,
            message: value.message().to_string_lossy(),
        }
    }
}

impl From<RegistryKeyHandleError> for RegistryError {
    fn from(value: RegistryKeyHandleError) -> Self {
        Self::WinError {
            code: value.code,
            message: value.message,
        }
    }
}

impl From<(RegistryKey, WinError)> for RegistryError {
    fn from(value: (RegistryKey, WinError)) -> Self {
        Self::CreateEnumeratorError {
            key: value.0,
            code: value.1.code().0,
            message: value.1.message().to_string_lossy(),
        }
    }
}

#[derive(Debug)]
pub struct RegistryKeyHandleError {
    code: i32,
    message: String,
}

impl From<WinError> for RegistryKeyHandleError {
    fn from(value: WinError) -> Self {
        Self {
            code: value.code().0,
            message: value.message().to_string_lossy(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RegistryKeyInfo {
    sub_key_count: u32,
    max_sub_key_len: u32,
    max_class_len: u32,
    value_count: u32,
    max_value_name_len: u32,
    max_value_len: u32,
    security_descriptor: u32,
    last_write_time: SystemTime,
}

impl RegistryKeyInfo {
    pub fn sub_key_count(&self) -> u32 {
        self.sub_key_count
    }

    pub fn max_sub_key_len(&self) -> u32 {
        self.max_sub_key_len
    }

    pub fn max_class_len(&self) -> u32 {
        self.max_class_len
    }

    pub fn value_count(&self) -> u32 {
        self.value_count
    }

    pub fn max_value_name_len(&self) -> u32 {
        self.max_value_name_len
    }

    pub fn max_value_len(&self) -> u32 {
        self.max_value_len
    }

    pub fn security_descriptor(&self) -> u32 {
        self.security_descriptor
    }

    pub fn last_write_time(&self) -> SystemTime {
        self.last_write_time
    }
}

impl From<REG_KEY_INFO> for RegistryKeyInfo {
    fn from(value: REG_KEY_INFO) -> Self {
        RegistryKeyInfo {
            sub_key_count: value.sub_keys,
            max_sub_key_len: value.max_sub_key_len,
            max_class_len: value.max_class_len,
            value_count: value.values,
            max_value_name_len: value.max_value_name_len,
            max_value_len: value.max_value_len,
            security_descriptor: value.security_descriptor,
            last_write_time: file_time_to_std_system_time(value.last_write_time),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegistryRootKey {
    ClassesRoot = HKEY_CLASSES_ROOT.0,
    CurrentConfig = HKEY_CURRENT_CONFIG.0,
    CurrentUser = HKEY_CURRENT_USER.0,
    CurrentUserLocalSettings = HKEY_CURRENT_USER_LOCAL_SETTINGS.0,
    LocalMachine = HKEY_LOCAL_MACHINE.0,
    PerformanceData = HKEY_PERFORMANCE_DATA.0,
    PerformanceNlstext = HKEY_PERFORMANCE_NLSTEXT.0,
    PerformanceText = HKEY_PERFORMANCE_TEXT.0,
    Users = HKEY_USERS.0,
}

impl RegistryRootKey {
    fn name(&self) -> &OsStr {
        match self {
            Self::ClassesRoot => OsStr::new("HKEY_CLASSES_ROOT"),
            Self::CurrentConfig => OsStr::new("HKEY_CURRENT_CONFIG"),
            Self::CurrentUser => OsStr::new("HKEY_CURRENT_USER"),
            Self::CurrentUserLocalSettings => OsStr::new("HKEY_CURRENT_USER_LOCAL_SETTINGS"),
            Self::LocalMachine => OsStr::new("HKEY_LOCAL_MACHINE"),
            Self::PerformanceData => OsStr::new("HKEY_PERFORMANCE_DATA"),
            Self::PerformanceNlstext => OsStr::new("HKEY_PERFORMANCE_NLSTEXT"),
            Self::PerformanceText => OsStr::new("HKEY_PERFORMANCE_TEXT"),
            Self::Users => OsStr::new("HKEY_USERS"),
        }
    }

    pub fn sub_key(self, sub_key_name: Box<OsStr>) -> Result<RegistryKey> {
        let mut key = RegistryKey::new(
            self,
            Some(sub_key_name),
            OsString::new().into_boxed_os_str(),
        );
        let handle = key.try_open_handle_with_query_access()?;
        key.class = registry::get_class(handle.id)?;
        Ok(key)
    }

    #[inline]
    pub fn to_key(self) -> RegistryKey {
        RegistryKey::from(self)
    }

    #[inline]
    pub fn try_to_enum<T: RegistryEnumerable>(self) -> Result<RegistryEnumarator<SubKeys>> {
        self.try_into()
    }
}

impl As<isize> for RegistryRootKey {
    fn as_type(&self) -> isize {
        *self as isize
    }
}

pub struct RegistryKeyHandle {
    id: isize,
    is_root: bool,
}

impl RegistryKeyHandle {
    fn new(id: isize, is_root: bool) -> Self {
        RegistryKeyHandle { id, is_root }
    }

    fn new_sub(id: isize) -> Self {
        RegistryKeyHandle::new(id, false)
    }

    fn try_open(
        key: &RegistryKey,
        access: RegistryAccessRights,
    ) -> ::core::result::Result<Self, RegistryKeyHandleError> {
        if key.name.is_some() {
            registry::open_key(key.root.as_type(), key.name.as_deref(), 0, access)
                .map(Self::new_sub)
                .map_err(RegistryKeyHandleError::from)
        } else {
            Ok(key.root.into())
        }
    }
}

impl From<RegistryRootKey> for RegistryKeyHandle {
    fn from(value: RegistryRootKey) -> Self {
        Self::new(value.as_type(), true)
    }
}

impl Drop for RegistryKeyHandle {
    fn drop(&mut self) {
        if !self.is_root {
            if let Err(error) = registry::close_key(self.id) {
                println!(
                    "Error occured while closing registry key handle: {}!",
                    error.message().to_string_lossy(),
                );
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RegistryValueType {
    None = REG_NONE.0 as u8,
    String = REG_SZ.0 as u8,
    ExpandedString = REG_EXPAND_SZ.0 as u8,
    #[default]
    Binary = REG_BINARY.0 as u8,
    Dword = REG_DWORD.0 as u8,
    DwordBigEndian = REG_DWORD_BIG_ENDIAN.0 as u8,
    Link = REG_LINK.0 as u8,
    MultiString = REG_MULTI_SZ.0 as u8,
    Qword = REG_QWORD.0 as u8,
}

impl As<u32> for RegistryValueType {
    fn as_type(&self) -> u32 {
        *self as u32
    }
}

impl From<REG_VALUE_TYPE> for RegistryValueType {
    fn from(value: REG_VALUE_TYPE) -> Self {
        match value {
            REG_NONE => Self::None,
            REG_SZ => Self::String,
            REG_EXPAND_SZ => Self::ExpandedString,
            REG_BINARY => Self::Binary,
            REG_DWORD => Self::Dword,
            REG_DWORD_BIG_ENDIAN => Self::DwordBigEndian,
            REG_LINK => Self::Link,
            REG_MULTI_SZ => Self::String,
            REG_QWORD => Self::Qword,
            _ => Self::Binary,
        }
    }
}

impl From<RegistryValueType> for REG_VALUE_TYPE {
    fn from(value: RegistryValueType) -> Self {
        REG_VALUE_TYPE(value.as_type())
    }
}

#[derive(Debug)]
pub struct RegistryValue {
    raw_value: Box<[u8]>,
    value_type: RegistryValueType,
}

impl RegistryValue {
    fn new(raw_value: Box<[u8]>, value_type: RegistryValueType) -> Self {
        RegistryValue {
            raw_value,
            value_type,
        }
    }

    pub fn value<T: TryFromRegistryValue<Error = RegistryError>>(&self) -> Result<T> {
        T::try_from_reg_value(self)
    }

    pub fn raw_value(&self) -> &[u8] {
        &self.raw_value
    }

    pub fn value_type(&self) -> RegistryValueType {
        self.value_type
    }
}

impl From<REG_VALUE> for RegistryValue {
    fn from(value: REG_VALUE) -> Self {
        let len = value.len as usize;
        println!("len: {}", len);
        let boxed = unsafe { Vec::from_raw_parts(value.data, len, len) }.into_boxed_slice();
        //vec![0; 0].into_boxed_slice();
        RegistryValue::new(boxed, value.value_type.into())
    }
}

impl From<RegistryValue> for Box<[u8]> {
    fn from(value: RegistryValue) -> Self {
        value.raw_value
    }
}

#[derive(Debug)]
pub struct RegistryValueInfo {
    name: Box<OsStr>,
    value: RegistryValue,
}

impl RegistryValueInfo {
    fn new(name: Box<OsStr>, value: RegistryValue) -> Self {
        RegistryValueInfo { name, value }
    }

    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn value(&self) -> &RegistryValue {
        &self.value
    }
}

impl From<REG_ENUM_VALUE_INFO> for RegistryValueInfo {
    fn from(value: REG_ENUM_VALUE_INFO) -> Self {
        RegistryValueInfo::new(pwstr_to_boxed_osstr(value.name), value.value.into())
    }
}

pub trait TryFromRegistryValue: Sized {
    type Error;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self>;
}

impl TryFromRegistryValue for u32 {
    type Error = RegistryError;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self> {
        let type_size = core::mem::size_of::<Self>();
        if value.raw_value.len() < type_size {
            return Err(RegistryError::InvalidValueSize);
        }

        let mut bytes = [0; core::mem::size_of::<Self>()];
        bytes.copy_from_slice(&value.raw_value[..type_size]);
        match value.value_type {
            RegistryValueType::Dword => Ok(u32::from_ne_bytes(bytes)),
            RegistryValueType::DwordBigEndian => Ok(u32::from_be_bytes(bytes)),
            _ => Err(RegistryError::InvalidValueType),
        }
    }
}

impl TryFromRegistryValue for u64 {
    type Error = RegistryError;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self> {
        let type_size = core::mem::size_of::<Self>();
        if value.raw_value.len() < type_size {
            return Err(RegistryError::InvalidValueSize);
        }

        let mut bytes = [0; core::mem::size_of::<Self>()];
        bytes.copy_from_slice(&value.raw_value[..type_size]);
        match value.value_type {
            RegistryValueType::Qword => Ok(u64::from_ne_bytes(bytes)),
            _ => Err(RegistryError::InvalidValueType),
        }
    }
}

impl TryFromRegistryValue for OsString {
    type Error = RegistryError;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self> {
        match value.value_type {
            RegistryValueType::String
            | RegistryValueType::ExpandedString
            | RegistryValueType::Link
                if value.raw_value.len() == 0 =>
            {
                Ok(OsString::new())
            }
            RegistryValueType::String
            | RegistryValueType::ExpandedString
            | RegistryValueType::Link
                if value.raw_value.len() % 2 == 0 =>
            {
                let utf16_bytes = unsafe {
                    slice::from_raw_parts(
                        value.raw_value.as_ptr() as *const u16,
                        value.raw_value.len() / 2,
                    )
                };
                let string: OsString = OsStringExt::from_wide(utf16_bytes);
                Ok(string)
            }
            RegistryValueType::String
            | RegistryValueType::ExpandedString
            | RegistryValueType::Link => Err(RegistryError::InvalidValueSize),
            _ => Err(RegistryError::InvalidValueType),
        }
    }
}

impl TryFromRegistryValue for Box<OsStr> {
    type Error = RegistryError;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self> {
        OsString::try_from_reg_value(value).map(Box::<OsStr>::from)
    }
}

impl TryFromRegistryValue for Vec<Box<OsStr>> {
    type Error = RegistryError;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self> {
        match value.value_type {
            RegistryValueType::MultiString if value.raw_value.len() == 0 => Ok(Vec::new()),
            RegistryValueType::MultiString if value.raw_value.len() % 2 == 0 => {
                let slice = unsafe {
                    slice::from_raw_parts(
                        value.raw_value.as_ptr() as *const u16,
                        value.raw_value.len() / 2,
                    )
                };
                let null_terminator_char = '\0' as u16;
                let non_terminated_slice =
                    if slice.ends_with(&[null_terminator_char, null_terminator_char]) {
                        slice
                    } else {
                        &slice[..slice.len() - 1]
                    };
                let string_count = non_terminated_slice
                    .iter()
                    .filter(|&&char| char == null_terminator_char)
                    .count();
                let mut strings = Vec::with_capacity(string_count);
                let mut buffer =
                    Vec::with_capacity((non_terminated_slice.len() - string_count) / string_count);
                let mut buffer_position = 0;
                for &char in non_terminated_slice {
                    if char != null_terminator_char {
                        buffer.push(char);
                    } else {
                        strings.push(
                            OsString::from_wide(&buffer[..buffer_position]).into_boxed_os_str(),
                        );
                        buffer_position = 0;
                    }
                }

                Ok(strings)
            }
            RegistryValueType::MultiString => Err(RegistryError::InvalidValueSize),
            _ => Err(RegistryError::InvalidValueType),
        }
    }
}

impl TryFromRegistryValue for Box<[Box<OsStr>]> {
    type Error = RegistryError;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self> {
        Vec::<Box<OsStr>>::try_from_reg_value(value).map(Box::<[Box<OsStr>]>::from)
    }
}

impl TryFromRegistryValue for () {
    type Error = RegistryError;
    fn try_from_reg_value(value: &RegistryValue) -> Result<Self> {
        match value.value_type {
            RegistryValueType::None => Ok(()),
            _ => Err(RegistryError::InvalidValueType),
        }
    }
}

#[derive(Debug)]
pub struct RegistryKey {
    root: RegistryRootKey,
    name: Option<Box<OsStr>>,
    class: Box<OsStr>,
}

impl RegistryKey {
    fn new(root: RegistryRootKey, name: Option<Box<OsStr>>, class: Box<OsStr>) -> Self {
        RegistryKey { root, name, class }
    }

    fn new_root(root: RegistryRootKey) -> Self {
        RegistryKey::new(root, None, OsString::new().into_boxed_os_str())
    }

    fn try_open_handle_with_query_access(
        &self,
    ) -> ::core::result::Result<RegistryKeyHandle, RegistryKeyHandleError> {
        RegistryKeyHandle::try_open(self, KEY_QUERY_VALUE)
    }

    fn try_open_handle_with_query_and_enum_access(
        &self,
    ) -> ::core::result::Result<RegistryKeyHandle, RegistryKeyHandleError> {
        RegistryKeyHandle::try_open(self, KEY_QUERY_VALUE | KEY_ENUMERATE_SUB_KEYS)
    }

    pub fn is_root(&self) -> bool {
        self.name.is_none()
    }

    pub fn class(&self) -> Result<Box<OsStr>> {
        let handle = self.try_open_handle_with_query_access()?;
        registry::get_class(handle.id).map_err(RegistryError::from)
    }

    pub fn raw_default_value(&self) -> Result<Box<[u8]>> {
        let handle = self.try_open_handle_with_query_access()?;
        let max_value_len = self.info()?.max_value_len;
        let value: RegistryValue = registry::get_raw_value(handle.id, None, max_value_len)?.into();
        Ok(value.into())
    }

    pub fn default_value<T: TryFromRegistryValue<Error = RegistryError>>(&self) -> Result<T> {
        let handle = self.try_open_handle_with_query_access()?;
        let max_value_len = self.info()?.max_value_len;
        let value: RegistryValue = registry::get_raw_value(handle.id, None, max_value_len)?.into();
        value.value()
    }

    pub fn raw_value(&self, value_name: Box<OsStr>) -> Result<Box<[u8]>> {
        let handle = self.try_open_handle_with_query_access()?;
        let max_value_len = self.info()?.max_value_len;
        let value: RegistryValue =
            registry::get_raw_value(handle.id, Some(&value_name), max_value_len)?.into();
        Ok(value.into())
    }

    pub fn value<T: TryFromRegistryValue<Error = RegistryError>>(
        &self,
        value_name: Box<OsStr>,
    ) -> Result<T> {
        let handle = self.try_open_handle_with_query_access()?;
        let max_value_len = self.info()?.max_value_len;
        let value: RegistryValue =
            registry::get_raw_value(handle.id, Some(&value_name), max_value_len)?.into();
        value.value()
    }

    pub fn name(&self) -> &OsStr {
        if self.is_root() {
            self.root.name()
        } else {
            // name is only none when the regkey is a root key
            unsafe { self.name.as_deref().unwrap_unchecked() }
        }
    }

    pub fn info(&self) -> Result<RegistryKeyInfo> {
        let handle = self.try_open_handle_with_query_access()?;
        registry::get_info_without_class(handle.id)
            .map(RegistryKeyInfo::from)
            .map_err(RegistryError::from)
    }

    pub fn try_into_enum<T: RegistryEnumerable>(self) -> Result<RegistryEnumarator<T>> {
        self.try_into()
    }
}

impl From<RegistryRootKey> for RegistryKey {
    fn from(value: RegistryRootKey) -> Self {
        RegistryKey::new_root(value)
    }
}

pub trait RegistryEnumerable {}
pub struct SubKeys;
impl RegistryEnumerable for SubKeys {}
pub struct Values;
impl RegistryEnumerable for Values {}

pub struct RegistryEnumarator<T>
where
    T: RegistryEnumerable,
{
    handle: RegistryKeyHandle,
    key: RegistryKey,
    index: u32,
    key_info: RegistryKeyInfo,
    phantom_data: PhantomData<T>,
}

impl<T> RegistryEnumarator<T>
where
    T: RegistryEnumerable,
{
    const fn new(handle: RegistryKeyHandle, key: RegistryKey, key_info: RegistryKeyInfo) -> Self {
        Self {
            handle,
            key,
            index: 0,
            key_info,
            phantom_data: PhantomData,
        }
    }

    pub fn try_create(key: RegistryKey) -> Result<Self> {
        let handle = match key.try_open_handle_with_query_and_enum_access() {
            Ok(handle) => handle,
            Err(error) => return Err(error.into()),
        };
        let key_info = match registry::get_info(handle.id) {
            Ok(info) => info,
            Err(error) => return Err((key, error).into()),
        };
        Ok(Self::new(handle, key, key_info.into()))
    }

    pub fn key(&self) -> &RegistryKey {
        &self.key
    }

    pub fn key_info(&self) -> RegistryKeyInfo {
        self.key_info
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn into_key(self) -> RegistryKey {
        self.key
    }
}

impl<T> ::core::fmt::Debug for RegistryEnumarator<T>
where
    T: RegistryEnumerable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistryEnumarator")
            .field("index", &self.index)
            .field("key_info", &self.key_info)
            .finish()
    }
}

impl Iterator for RegistryEnumarator<SubKeys> {
    type Item = RegistryKey;
    fn next(&mut self) -> Option<Self::Item> {
        let key = registry::enum_subkeys(
            self.handle.id,
            self.index,
            self.key_info.max_sub_key_len + 1,
            self.key_info.max_class_len + 1,
        )
        .map(|info| {
            RegistryKey::new(
                self.key.root,
                Some(pwstr_to_boxed_osstr(info.name)),
                pwstr_to_boxed_osstr(info.class),
            )
        });
        self.index += 1;
        key
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.key_info.sub_key_count as usize))
    }
}

impl Iterator for RegistryEnumarator<Values> {
    type Item = RegistryValueInfo;
    fn next(&mut self) -> Option<Self::Item> {
        let item = registry::enum_values(
            self.handle.id,
            self.index,
            self.key_info.max_value_name_len + 1,
            self.key_info.max_value_len,
        );
        self.index += 1;
        item.map(RegistryValueInfo::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.key_info.value_count as usize,
            Some(self.key_info.value_count as usize),
        )
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.key_info.value_count as usize
    }
}

impl<T> TryFrom<RegistryKey> for RegistryEnumarator<T>
where
    T: RegistryEnumerable,
{
    type Error = RegistryError;
    fn try_from(value: RegistryKey) -> Result<Self> {
        RegistryEnumarator::<T>::try_create(value)
    }
}

impl<T> TryFrom<RegistryRootKey> for RegistryEnumarator<T>
where
    T: RegistryEnumerable,
{
    type Error = RegistryError;
    fn try_from(value: RegistryRootKey) -> Result<Self> {
        value.to_key().try_into_enum()
    }
}
