use crate::alloc::{self};
use crate::core::Result;
use crate::U16CString;
use crate::{call, default_sized, from_BOOL, to_BOOL};
use core::ptr;
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, DragQueryPoint};

#[allow(clippy::undocumented_unsafe_blocks)]
#[inline]
/// Registers whether a window accepts dropped files
///
/// An application that calls [`accept_files`] with the `accept` set to `true`
/// has identified itself as able to process the [`WM_DROPFILES`][crate::win32::ui::messaging::WM_DROPFILES]
/// message from `File Manager`. To discontinue accepting dropped files, call the function with
/// `accept` set to `false`.
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragacceptfiles
///
pub fn accept_files(window_handle: isize, accept: bool) {
    unsafe { DragAcceptFiles(window_handle, to_BOOL!(accept)) }
}

#[allow(clippy::undocumented_unsafe_blocks)]
#[inline]
/// Releases memory that the system allocated for use in transferring file names to the application.
///
/// # Arguments
///
/// * `drop_handle`: Identifier of the structure that describes dropped files.
///     * This handle is retrieved from the `wParam` parameter of the
///       [`WM_DROPFILES`][crate::win32::ui::messaging::WM_DROPFILES] message.
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragfinish
///
pub fn finish(drop_handle: isize) {
    unsafe { DragFinish(drop_handle) }
}

/// Gets the count of the files dropped.
///
/// # Arguments
///
/// * `drop_handle`: Identifier of the structure that describes dropped files.
/// * `index`: Index of the file to query.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
pub fn get_file_count(drop_handle: isize) -> Result<u32> {
    call! {
        DragQueryFileW(
            drop_handle,
            u32::MAX,
            ptr::null_mut(),
            0,
        ) != 0
    }
}

/// Gets length of the dropped file's name in chars on `index`.
/// The null terminator is not included in the length.
///
/// # Arguments
///
/// * `drop_handle`: Identifier of the structure that describes dropped files.
/// * `index`: Index of the file to query.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
pub fn get_file_len(drop_handle: isize, index: u32) -> Result<u32> {
    call! {
        DragQueryFileW(
            drop_handle,
            index,
            ptr::null_mut(),
            0,
        ) != 0
    }
}

/// Gets the names of dropped files that result from a successful drag-and-drop operation.
///
/// # Arguments
///
/// * `drop_handle`: Identifier of the structure that describes dropped files.
/// * `index`: Index of the file to query.
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
///
pub fn get_file(drop_handle: isize, index: u32) -> Result<U16CString> {
    // The passed length to `DragQueryFileW` needs to include the null terminator.
    let file_len = get_file_len(drop_handle, index)? + 1;
    let mut buffer = alloc::vec![0; file_len as usize];
    call! {
        DragQueryFileW(
            drop_handle,
            index,
            buffer.as_mut_ptr(),
            file_len,
        ) == 0 => return Error
    };
    // Safety: `buffer` does not contain any interior null values.
    Ok(unsafe { U16CString::from_vec_unchecked(buffer) })
}

#[allow(clippy::undocumented_unsafe_blocks)]
/// Gets coordinates of the mouse pointer at the time the file was dropped
/// and a value specifying whether the dropped file occured in the client area.
///
/// # Arguments
///
/// * `drop_handle`: Handle of the drop structure that describes the dropped file..
///
/// # Errors
///
/// If the function fails an [error][crate::core::error::Error] is returned providing information about the cause of the failure.
///
/// # Examples
/// TODO
///
/// For more information, see the official [documentation].
///
/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint
///
pub fn get_point(drop_handle: isize) -> (POINT, bool) {
    let mut point = default_sized!(POINT);
    let occured_in_client_area = unsafe { DragQueryPoint(drop_handle, &mut point) };
    (point, from_BOOL!(occured_in_client_area))
}
