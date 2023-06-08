use crate::common::{pcwstr_to_u16_string, To};
use crate::win32::core::Result;
use crate::win32::system::memory::{get_local_handle, local_free};
use core::ffi::c_void;
use core::ptr::{self, addr_of, addr_of_mut};
use widestring::U16String;
use windows_sys::Win32::System::Diagnostics::Debug::{
    FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_ARGUMENT_ARRAY,
    FORMAT_MESSAGE_FROM_HMODULE, FORMAT_MESSAGE_FROM_STRING, FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS, FORMAT_MESSAGE_OPTIONS,
};

/// A [`FORMAT_MESSAGE_OPTIONS`] flag that is used to tell [`FormatMessageW`] to
/// remove regular line breaks (`\r\n` or `\n`) from the formatted message.
pub const FORMAT_MESSAGE_IGNORE_REGULAR_LINE_BREAKS: u32 = 0x000000FF;

use crate::call_num;

/// A marker trait for types that can be used as a source in [`format_message_with_source`].
pub trait FormatSource {
    /// Gets the associated [`FORMAT_MESSAGE_OPTIONS`] flag for the type.
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
/// Specifies options for message formatting.
/// It can be passed to both [`format_message`] and [`format_message_with_buffer`] as the `options` argument.
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
pub struct System;

impl FormatSource for U16String {
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS {
        FORMAT_MESSAGE_FROM_STRING
    }
}

impl FormatSource for isize {
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS {
        FORMAT_MESSAGE_FROM_HMODULE
    }
}

impl FormatSource for System {
    fn format_message_options_flag() -> FORMAT_MESSAGE_OPTIONS {
        FORMAT_MESSAGE_FROM_SYSTEM
    }
}

/// Gets the non-null-terminated formatted message string from the `source` location.
///
/// # Arguments
/// * `source`: The location of the message definition.
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
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
pub fn format_message<T>(
    source: T,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
) -> Result<U16String>
where
    T: FormatSource,
{
    let mut buffer = ptr::null_mut::<u16>();
    let buffer_size = 0;
    let source_ptr = if T::format_message_options_flag() == FORMAT_MESSAGE_FROM_SYSTEM {
        ptr::null()
    } else {
        addr_of!(source).cast()
    };
    let args_flag = if args.is_none() {
        0
    } else {
        FORMAT_MESSAGE_ARGUMENT_ARRAY
    };
    let option_flags = FORMAT_MESSAGE_ALLOCATE_BUFFER
        | T::format_message_options_flag()
        | args_flag
        | options.to();
    let buffer_len = call_num! {
        (FormatMessageW(
            option_flags,
            source_ptr ,
            id,
            lang_id,
            addr_of_mut!(buffer).cast(),
            buffer_size,
            if let Some(args) = args { args.as_ptr() } else { ptr::null() }
        ) == 0) => return Error;
    };
    // Safety: `buffer` contains a valid system allocated `buffer_len` + 1 (including null termination).
    let can_trim_end = (options.to() & FormatMessagetOptions::RegularLineBreaks.to())
        == FormatMessagetOptions::RegularLineBreaks.to();
    let message = unsafe { pcwstr_to_u16_string(buffer, buffer_len as usize, can_trim_end) };
    let handle = get_local_handle(buffer.cast::<c_void>())?;
    local_free(handle)?;
    Ok(message)
}

/// Gets the non-null-terminated formatted message string retrieved from the `source` location and copies into `buffer`.
///
/// # Arguments
/// * `source`: The location of the message definition.
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
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
/// # Possible errors
///  
/// * `buffer` is not large enough to store the formatted message. ([`ERROR_INSUFFICIENT_BUFFER`][`windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER`])
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation] of [`FormatMessageW`].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
/// [security-remarks]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#security-remarks
pub fn format_message_with_buffer<T>(
    source: T,
    id: u32,
    lang_id: u32,
    args: Option<&[*const i8]>,
    options: FormatMessagetOptions,
    buffer: &mut [u16],
) -> Result<u32>
where
    T: FormatSource,
{
    let source_ptr = if T::format_message_options_flag() == FORMAT_MESSAGE_FROM_SYSTEM {
        ptr::null()
    } else {
        addr_of!(source).cast()
    };
    let args_flag = if args.is_none() {
        0
    } else {
        FORMAT_MESSAGE_ARGUMENT_ARRAY
    };
    let option_flags = T::format_message_options_flag() | args_flag | options.to();
    call_num! {
        FormatMessageW(
            option_flags,
            source_ptr,
            id,
            lang_id,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            if let Some(args) = args { args.as_ptr() } else { ptr::null() }
        ) != 0
    }
}
