use crate::core::{Result, To};
use crate::{call, free};
use core::ffi::c_void;
use core::ptr::{self, addr_of, addr_of_mut};
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_ARGUMENT_ARRAY,
    FORMAT_MESSAGE_FROM_HMODULE, FORMAT_MESSAGE_FROM_STRING, FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS, FORMAT_MESSAGE_OPTIONS,
};

pub use windows_sys::Win32::System::Diagnostics::Debug::{FACILITY_NT_BIT, FACILITY_WIN32};

/// A [`FORMAT_MESSAGE_OPTIONS`] flag that is used to tell [`FormatMessageW`] to
/// remove regular line breaks (`\r\n` or `\n`) from the formatted message.
pub const FORMAT_MESSAGE_IGNORE_REGULAR_LINE_BREAKS: u32 = 0x000000FF;

/// A marker trait for types that can be used as a source when formatting a message.
trait FormatSource {
    /// Gets the associated [`FORMAT_MESSAGE_OPTIONS`] flag for the type.
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS;

    /// Converts the borrowed `self` to an immutable [`c_void`] pointer.
    /// This is a free `Borrowed` -> `Borrowed` conversion.
    fn as_c_void_ptr(&self) -> *const c_void;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Specifies the options for message formatting.
pub enum FormatMessagetOptions {
    /// Do not ignore anything.
    None,
    /// Ignore regular line breaks (`\r\n` or `\n`).
    RegularLineBreaks = FORMAT_MESSAGE_IGNORE_REGULAR_LINE_BREAKS,
    /// Ignore message inserts (e.g. `%1`, `%2`, etc.).
    Inserts = FORMAT_MESSAGE_IGNORE_INSERTS,
    // Ignore both regular line breaks and message inserts.
    All = Self::Inserts as u32 | Self::RegularLineBreaks as u32,
}

impl To<u32> for FormatMessagetOptions {
    #[inline]
    fn to(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug)]
/// The `System` source that can be used with [`format_message`] and [`format_message_with_buffer`] as a `source` argument.
struct System;

impl FormatSource for &U16CStr {
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS {
        FORMAT_MESSAGE_FROM_STRING
    }

    fn as_c_void_ptr(&self) -> *const c_void {
        self.as_ptr().cast()
    }
}

impl FormatSource for isize {
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS {
        FORMAT_MESSAGE_FROM_HMODULE
    }

    fn as_c_void_ptr(&self) -> *const c_void {
        addr_of!(*self).cast()
    }
}

impl FormatSource for System {
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS {
        FORMAT_MESSAGE_FROM_SYSTEM
    }

    fn as_c_void_ptr(&self) -> *const c_void {
        ptr::null()
    }
}

/// Gets the null-terminated formatted message string from the system message table.
///
/// # Arguments
///
/// * `id`: The message identifier for the requested message.
/// * `lang_id`: The language identifier for the requested message.
/// * `args`: An array of values that are used as insert values in the formatted message.
/// * `options`: The formatting options.
///
/// # Security
///
/// See the [security-remarks] section in the official [documentation].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_RESOURCE_LANG_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * The function failed to find a message for the `LANGID` specified by `lang_id`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
///
pub fn format_message(
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
) -> Result<U16CString> {
    internal_format_message(System, id, lang_id, args, options, false)
}

/// Gets the null-terminated formatted message string from the specified module.
///
/// # Arguments
///
/// * `source`: The handle to the source module that contains the message definition.
/// * `id`: The message identifier for the requested message.
/// * `lang_id`: The language identifier for the requested message.
/// * `args`: An array of values that are used as insert values in the formatted message.
/// * `options`: The formatting options.
/// * `can_search_system`: Determines whether the function should search the system message table if the message is not found in the module specified.
///
/// # Remarks
///
/// * If `source` is a null handle (0), then the current process's application image file will be searched.
///
/// # Security
///
/// See the [security-remarks] section in the official [documentation].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Possible errors
///
/// * `source` is an invalid module handle.
/// * [`ERROR_RESOURCE_TYPE_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_TYPE_NOT_FOUND]
///     * `source` has no message table resource.
/// * [`ERROR_RESOURCE_LANG_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * The function failed to find a message for the `LANGID` specified by `lang_id`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
///
pub fn format_message_from_module(
    source: isize,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
    can_search_system: bool,
) -> Result<U16CString> {
    internal_format_message(source, id, lang_id, args, options, can_search_system)
}

/// Gets the null-terminated formatted message string from the specified string.
///
/// # Arguments
///
/// * `source`: The source string that contains the message definition.
/// * `id`: The message identifier for the requested message.
/// * `lang_id`: The language identifier for the requested message.
/// * `args`: An array of values that are used as insert values in the formatted message.
/// * `options`: The formatting options.
///
/// # Security
///
/// See the [security-remarks] section in the official [documentation].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_RESOURCE_LANG_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * The function failed to find a message for the `LANGID` specified by `lang_id`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
///
pub fn format_message_from_str(
    source: &U16CStr,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
) -> Result<U16CString> {
    internal_format_message(source, id, lang_id, args, options, false)
}

#[inline]
#[doc(hidden)]
fn internal_format_message<T>(
    source: T,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
    can_search_system: bool, // ignored if T is not `isize` (module handle)
) -> Result<U16CString>
where
    T: FormatSource,
{
    let mut buffer = ptr::null_mut::<u16>();
    let buffer_len = 0;
    let args_flag = if args.is_none() {
        0
    } else {
        FORMAT_MESSAGE_ARGUMENT_ARRAY
    };
    let system_flag =
        if can_search_system && T::format_message_options_flag() == FORMAT_MESSAGE_FROM_HMODULE {
            FORMAT_MESSAGE_FROM_SYSTEM
        } else {
            0
        };
    let option_flags = FORMAT_MESSAGE_ALLOCATE_BUFFER
        | T::format_message_options_flag()
        | args_flag
        | options.to()
        | system_flag;
    call! {
        FormatMessageW(
            option_flags,
            T::as_c_void_ptr(&source),
            id,
            lang_id,
            addr_of_mut!(buffer).cast(),
            buffer_len as u32,
            if let Some(args) = args { args.as_ptr() } else { ptr::null() },
        ) == 0 => return Error;
    };
    // Safety: `buffer` contains a valid system allocated `buffer_len` + 1 (including null termination).
    let message = unsafe { U16CString::from_ptr_str(buffer) };
    free!(Local: buffer);
    Ok(message)
}

/// Gets the null-terminated formatted message string from the system message table and copies it into `buffer`.
///
/// # Arguments
///
/// * `id`: The message identifier for the requested message.
/// * `lang_id`: The language identifier for the requested message.
/// * `args`: An array of values that are used as insert values in the formatted message.
/// * `options`: The formatting options.
/// * `buffer`: The buffer where the message gets copied to.
///
/// # Security
///
/// See the [security-remarks] section in the official [documentation].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///  
/// * [`ERROR_INSUFFICIENT_BUFFER`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * `buffer` is not large enough to store the formatted message.
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * The length of the formatted message exceeds `128K` bytes.
/// * [`ERROR_RESOURCE_LANG_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * The function failed to find a message for the `LANGID` specified by `lang_id`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
///
pub fn format_message_with_buffer(
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
    buffer: &mut [u16],
) -> Result<u32> {
    internal_format_message_with_buffer(System, id, lang_id, args, options, buffer, false)
}

/// Gets the formatted message string from the specified module and copies it into `buffer`.
///
/// # Arguments
///
/// * `source`: The handle to the source module that contains the message definition.
/// * `id`: The message identifier for the requested message.
/// * `lang_id`: The language identifier for the requested message.
/// * `args`: An array of values that are used as insert values in the formatted message.
/// * `options`: The formatting options.
/// * `buffer`: The buffer where the message gets copied to.
/// * `can_search_system`: Determines whether the function should search the system message table if the message is not found in the module specified.
///
/// # Security
///
/// See the [security-remarks] section in the official [documentation].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * `source` is an invalid module handle.
/// * [`ERROR_RESOURCE_TYPE_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_TYPE_NOT_FOUND]
///     * `source` has no message table resource.
/// * [`ERROR_INSUFFICIENT_BUFFER`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * `buffer` is not large enough to store the formatted message.
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * The length of the formatted message exceeds `128K` bytes.
/// * [`ERROR_RESOURCE_LANG_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * The function failed to find a message for the `LANGID` specified by `lang_id`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
///
pub fn format_message_from_module_with_buffer(
    source: isize,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
    buffer: &mut [u16],
    can_search_system: bool,
) -> Result<u32> {
    internal_format_message_with_buffer(
        source,
        id,
        lang_id,
        args,
        options,
        buffer,
        can_search_system,
    )
}

/// Gets the formatted message string from the specified string and copies it into `buffer`.
///
/// # Arguments
///
/// * `source`: The source string that contains the message definition.
/// * `id`: The message identifier for the requested message.
/// * `lang_id`: The language identifier for the requested message.
/// * `args`: An array of values that are used as insert values in the formatted message.
/// * `options`: The formatting options.
/// * `buffer`: The buffer where the message gets copied to.
///
/// # Security
///
/// See the [security-remarks] section in the official [documentation].
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// ## Possible errors
///
/// * [`ERROR_INSUFFICIENT_BUFFER`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * `buffer` is not large enough to store the formatted message.
/// * [`ERROR_MORE_DATA`][windows_sys::Win32::Foundation::ERROR_MORE_DATA]
///     * The length of the formatted message exceeds `128K` bytes.
/// * [`ERROR_RESOURCE_LANG_NOT_FOUND`][windows_sys::Win32::Foundation::ERROR_RESOURCE_LANG_NOT_FOUND]
///     * The function failed to find a message for the `LANGID` specified by `lang_id`.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
///
pub fn format_message_from_str_with_buffer(
    source: &U16CStr,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
    buffer: &mut [u16],
) -> Result<u32> {
    internal_format_message_with_buffer(source, id, lang_id, args, options, buffer, false)
}

#[inline]
#[doc(hidden)]
fn internal_format_message_with_buffer<T>(
    source: T,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
    buffer: &mut [u16],
    can_search_system: bool, // ignored if T is not `isize` (module handle)
) -> Result<u32>
where
    T: FormatSource,
{
    let args_flag = if args.is_none() {
        0
    } else {
        FORMAT_MESSAGE_ARGUMENT_ARRAY
    };
    let system_flag =
        if can_search_system && T::format_message_options_flag() == FORMAT_MESSAGE_FROM_HMODULE {
            FORMAT_MESSAGE_FROM_SYSTEM
        } else {
            0
        };
    let option_flags = T::format_message_options_flag() | args_flag | options.to() | system_flag;
    call! {
        FormatMessageW(
            option_flags,
            T::as_c_void_ptr(&source),
            id,
            lang_id,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            if let Some(args) = args { args.as_ptr() } else { ptr::null() },
        ) != 0
    }
}
