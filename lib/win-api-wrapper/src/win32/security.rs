use crate::call_BOOL;
use crate::win32::core::Result;
use core::ffi::c_void;
use core::mem::size_of_val;
use core::ptr::addr_of_mut;
use windows_sys::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_INFORMATION_CLASS};

pub use windows_sys::Win32::Security::{
    TokenSource, TOKEN_ELEVATION, TOKEN_QUERY, TOKEN_QUERY_SOURCE,
};

/// Information that [`get_token_information`] can return about an access token.
pub trait TokenInformation {
    /// Gets the associated [`TOKEN_INFORMATION_CLASS`] of the type.
    fn token_information_class() -> TOKEN_INFORMATION_CLASS;
}

impl TokenInformation for TOKEN_ELEVATION {
    fn token_information_class() -> TOKEN_INFORMATION_CLASS {
        TokenElevation
    }
}

/// Gets a specified type of information about an access token.
///
/// # Errors
///
/// Returns a [`Win32Error`][`crate::win32::core::Win32Error`] if the function fails.
///
/// # Possible errors
///
/// - `handle` is invalid.
/// - `handle` doesn't have [`TOKEN_QUERY`] access right.
/// - `handle` doesn't have [`TOKEN_QUERY_SOURCE`] access right if the class returned by [`get_token_information`] specifies a [`TokenSource`].
///
/// # Examples
/// TODO
///
/// For more information see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation
pub fn get_token_information<T>(handle: isize) -> Result<T>
where
    T: Default + Copy + TokenInformation,
{
    call_BOOL! {
        GetTokenInformation(
            handle,
            T::token_information_class(),
            addr_of_mut!(token_info).cast::<c_void>(),
            size_of_val(&token_info) as u32,
            &mut 0) -> mut token_info: T
    }
}
