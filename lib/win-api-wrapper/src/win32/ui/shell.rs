use crate::core::Result;
use crate::{call_BOOL, call_HRESULT, call_num, default_sized, free, from_BOOL, to_BOOL};
use core::ptr::{self, addr_of};
use core::slice;
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::{POINT, RECT};
use windows_sys::Win32::UI::Shell::{
    AssocQueryStringW, CommandLineToArgvW, DragAcceptFiles, DragFinish, DragQueryFileW,
    DragQueryPoint, DuplicateIcon, ExtractIconExW, ExtractIconW, InitNetworkAddressControl,
    SHAppBarMessage, SHEmptyRecycleBinW, SHGetDiskFreeSpaceExW, SHGetDriveMedia,
    SHQueryRecycleBinW, SHQueryUserNotificationState, ShellAboutW, ShellExecuteExW,
    ShellMessageBoxW, Shell_NotifyIconGetRect, Shell_NotifyIconW, APPBARDATA, ASSOCF, ASSOCSTR,
    NOTIFYICONDATAW, NOTIFYICONIDENTIFIER, SHQUERYRBINFO,
};

pub use windows_sys::Win32::UI::Shell::{
    SEE_MASK_ASYNCOK, SEE_MASK_CLASSKEY, SEE_MASK_CLASSNAME, SEE_MASK_CONNECTNETDRV,
    SHELLEXECUTEINFOW,
};

pub fn parse_args(commandline: &U16CStr) -> Result<Vec<U16CString>> {
    let mut args_count = 0;
    let arg_ptrs = call_num! {
        CommandLineToArgvW(
            commandline.as_ptr(),
            &mut args_count,
        ) == ptr::null_mut::<*mut u16> () => return if Error
    };
    println!("args count: {}", args_count);
    let mut args = if args_count == 0 {
        free!(Local: arg_ptrs);
        return Ok(Vec::new());
    } else {
        Vec::with_capacity(args_count as usize)
    };
    // Safety: `arg_ptrs` is valid pointer to an array of `*mut u16` pointers for `args_count` length. If length is 0 an empty slice will be crated which is safe.
    let arg_ptrs_slice = unsafe { slice::from_raw_parts(arg_ptrs, args_count as usize) };
    for &arg in arg_ptrs_slice {
        // Safety: each `arg_ptr` is a pointer to a null-terminated Unicode string.
        args.push(unsafe { U16CString::from_ptr_str(arg) });
    }

    free!(Local: arg_ptrs);
    Ok(args)
}

pub fn drag_accept_files(window_handle: isize, accept: bool) {
    unsafe { DragAcceptFiles(window_handle, to_BOOL!(accept)) }
}

pub fn drag_finish(drop_handle: isize) {
    unsafe { DragFinish(drop_handle) }
}

pub fn drag_query_file_count(drop_handle: isize) -> Result<u32> {
    call_num! {
        DragQueryFileW(
            drop_handle,
            u32::MAX,
            ptr::null_mut(),
            0,
        ) != 0
    }
}

pub fn drag_query_file_len(drop_handle: isize, index: u32) -> Result<u32> {
    call_num! {
        DragQueryFileW(
            drop_handle,
            index,
            ptr::null_mut(),
            0,
        ) != 0
    }
}

pub fn drag_query_file(drop_handle: isize, index: u32) -> Result<U16CString> {
    let file_len = drag_query_file_len(drop_handle, index)? + 1; // + null-term
    let mut buffer = vec![0; file_len as usize];
    call_num! {
        DragQueryFileW(
            drop_handle,
            index,
            buffer.as_mut_ptr(),
            file_len,
        ) == 0 => return Error
    };
    // Safety: `buffer` doesn't contain any interior null values.
    Ok(unsafe { U16CString::from_vec_unchecked(buffer) })
}

pub fn drag_query_point(drop_handle: isize) -> (POINT, bool) {
    let mut point = default_sized!(POINT);
    let occured_in_client_area = unsafe { DragQueryPoint(drop_handle, &mut point) };
    (point, from_BOOL!(occured_in_client_area))
}

pub fn duplicate_icon(instance_handle: isize, icon_handle: isize) -> isize {
    unsafe { DuplicateIcon(instance_handle, icon_handle) }
}

pub fn extract_icon(instance_handle: isize, file_name: &U16CStr, index: u32) -> Option<isize> {
    let icon_handle = unsafe { ExtractIconW(instance_handle, file_name.as_ptr(), index) };
    if icon_handle == 0 {
        None
    } else {
        Some(icon_handle)
    }
}

pub fn extract_icon_count(file_name: &U16CStr) -> Result<u32> {
    call_num! {
        ExtractIconExW(
            file_name.as_ptr(),
            -1,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        ) != u32::MAX
    }
}

pub fn extract_small_icons(file_name: &U16CStr) -> Result<Vec<isize>> {
    let mut icon_count = extract_icon_count(file_name)?;
    if icon_count == 0 {
        return Ok(Vec::new());
    }

    let mut buffer = vec![0; icon_count as usize];
    icon_count = call_num! {
        ExtractIconExW(
            file_name.as_ptr(),
            0,
            buffer.as_mut_ptr(),
            ptr::null_mut(),
            0,
        ) == u32::MAX => return if Error
    };
    buffer.truncate(icon_count as usize);
    Ok(buffer)
}

pub fn extract_large_icons(file_name: &U16CStr) -> Result<Vec<isize>> {
    let mut icon_count = extract_icon_count(file_name)?;
    if icon_count == 0 {
        return Ok(Vec::new());
    }

    let mut buffer = vec![0; icon_count as usize];
    icon_count = call_num! {
        ExtractIconExW(
            file_name.as_ptr(),
            0,
            ptr::null_mut(),
            buffer.as_mut_ptr(),
            0,
        ) == u32::MAX => return if Error
    };
    buffer.truncate(icon_count as usize);
    Ok(buffer)
}

pub fn init_network_address_control() -> Result<()> {
    call_BOOL! { InitNetworkAddressControl() }
}

pub fn send_app_bar_message(message: u32, app_bar_data: &mut APPBARDATA) -> usize {
    unsafe { SHAppBarMessage(message, app_bar_data) }
}

pub fn get_asssoc_string(
    search_flags: ASSOCF,
    string_type: ASSOCSTR,
    association_string: &U16CStr,
    extra: Option<&U16CStr>,
) -> Result<U16CString> {
    let mut char_count = get_assoc_string_len(search_flags, string_type, association_string)?;
    let mut buffer = vec![0_u16; char_count as usize];
    call_HRESULT! {
        AssocQueryStringW(
            search_flags,
            string_type,
            association_string.as_ptr(),
            extra.map_or(ptr::null(), |ext| ext.as_ptr()),
            buffer.as_mut_ptr(),
            &mut char_count,
        ) return Error;
    }
    // Safety: `buffer` contains no interior null values
    Ok(unsafe { U16CString::from_vec_unchecked(buffer) })
}

pub fn get_notify_icon_rect(notify_icon_id: NOTIFYICONIDENTIFIER) -> Result<RECT> {
    call_HRESULT!(
        Shell_NotifyIconGetRect(
            addr_of!(notify_icon_id),
            &mut rect,
        ) -> mut rect = default_sized!(RECT)
    )
}

pub fn send_notify_icon_message(message: u32, data: &NOTIFYICONDATAW) -> Result<()> {
    call_BOOL! {
        Shell_NotifyIconW(message, data)
    }
}

pub fn show_about(
    window_handle: isize,
    title: &U16CStr,
    text: &U16CStr,
    icon_handle: isize,
) -> Result<()> {
    call_BOOL! {
        ShellAboutW(window_handle, title.as_ptr(), text.as_ptr(), icon_handle)
    }
}

/// `style` == MESSAGEBOX_STYLE
pub fn show_message_box(
    instance_handle: isize,
    window_handle: isize,
    message: &U16CStr,
    title: &U16CStr,
    style: u32,
) -> Result<i32> {
    call_num! {
        ShellMessageBoxW(
            instance_handle,
            window_handle,
            message.as_ptr(),
            title.as_ptr(),
            style
        ) != 0
    }
}

pub fn empty_bin(window_handle: isize, root_drive_path: &U16CStr, flags: u32) -> Result<()> {
    call_HRESULT! { SHEmptyRecycleBinW(window_handle, root_drive_path.as_ptr(), flags) }
}

pub fn get_disk_free_space(directory_name: &U16CStr) -> Result<(u64, u64, u64)> {
    call_BOOL! {
        SHGetDiskFreeSpaceExW(
            directory_name.as_ptr(),
            &mut available_bytes,
            &mut total_bytes,
            &mut total_free_bytes
        ) -> mut (available_bytes, total_bytes, total_free_bytes): (u64, u64, u64)
    }
}

pub fn get_assoc_string_len(
    search_flags: ASSOCF,
    string_type: ASSOCSTR,
    association_string: &U16CStr,
) -> Result<u32> {
    call_HRESULT! {
        AssocQueryStringW(
            search_flags,
            string_type,
            association_string.as_ptr(),
            ptr::null(),
            ptr::null_mut(),
            &mut char_count,
        ) -> mut char_count: u32
    }
}

pub fn get_drive_media(drive: &U16CStr) -> Result<u32> {
    call_HRESULT! {
        SHGetDriveMedia(
            drive.as_ptr(),
            &mut media_content
        ) -> mut media_content: u32
    }
}

/// `hIcon` member of the `SHFILEINFO` must be freed with `DestroyIcon`
pub fn query_bin(root_path: &U16CStr) -> Result<SHQUERYRBINFO> {
    call_HRESULT! {
        SHQueryRecycleBinW(
            root_path.as_ptr(),
            &mut bin_info,
        ) -> mut bin_info = default_sized!(SHQUERYRBINFO)
    }
}

pub fn query_user_notification_state() -> Result<i32> {
    call_HRESULT! {
        SHQueryUserNotificationState(&mut state) -> mut state: i32
    }
}

#[inline]
pub fn execute(execute_info: &mut SHELLEXECUTEINFOW) -> Result<()> {
    call_BOOL! { ShellExecuteExW(execute_info) }
}
