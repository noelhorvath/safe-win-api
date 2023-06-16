use crate::core::Result;
use crate::{call, call_BOOL, from_BOOL, to_BOOL};
use core::ptr;
use widestring::{U16CStr, U16CString};
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Foundation::{HWND, LPARAM, POINT};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AllowSetForegroundWindow, ArrangeIconicWindows, BringWindowToTop, ChildWindowFromPointEx,
    CloseWindow, DestroyIcon, DestroyWindow, EnumChildWindows, EnumThreadWindows, EnumWindows,
    FindWindowExW, GetAncestor, GetDesktopWindow, GetParent, GetShellWindow, GetTopWindow,
    GetWindow, GetWindowTextLengthW, GetWindowTextW, IsChild, IsIconic, IsWindow, IsWindowVisible,
    IsZoomed, LogicalToPhysicalPoint, MoveWindow, OpenIcon, RealChildWindowFromPoint, SetParent,
    SetWindowTextW, ShowWindow, ShowWindowAsync,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, SetForegroundWindow, WindowFromPhysicalPoint,
    WindowFromPoint,
};

pub use windows_sys::Win32::UI::WindowsAndMessaging::{
    CWP_ALL, CWP_FLAGS, CWP_SKIPDISABLED, CWP_SKIPINVISIBLE, CWP_SKIPTRANSPARENT, GA_PARENT,
    GA_ROOT, GA_ROOTOWNER, GET_ANCESTOR_FLAGS, GET_WINDOW_CMD, GW_CHILD, GW_ENABLEDPOPUP,
    GW_HWNDFIRST, GW_HWNDLAST, GW_HWNDNEXT, GW_HWNDPREV, GW_OWNER, SHOW_WINDOW_CMD,
    SW_FORCEMINIMIZE, SW_HIDE, SW_MAX, SW_MAXIMIZE, SW_MINIMIZE, SW_NORMAL, SW_RESTORE, SW_SHOW,
    SW_SHOWDEFAULT, SW_SHOWMINIMIZED, SW_SHOWMINNOACTIVE, SW_SHOWNA, SW_SHOWNOACTIVATE,
};

pub type EnumWindowsCallback = unsafe extern "system" fn(param0: HWND, param1: LPARAM) -> BOOL;

pub fn enum_child_windows(handle: isize, callback: EnumWindowsCallback, parameter: isize) {
    unsafe { EnumChildWindows(handle, Some(callback), parameter) };
}

pub fn enum_thread_windows(
    thread_id: u32,
    callback: EnumWindowsCallback,
    parameter: isize,
) -> bool {
    from_BOOL!(unsafe { EnumThreadWindows(thread_id, Some(callback), parameter) })
}

pub fn enum_windows(callback: EnumWindowsCallback, parameter: isize) -> Result<()> {
    call_BOOL! { EnumWindows(Some(callback), parameter) }
}

pub fn find(
    parent_handle: isize,
    child_handle: isize,
    class_name: Option<&U16CStr>,
    title: Option<&U16CStr>,
) -> Option<isize> {
    {
        call! {
            FindWindowExW(
                parent_handle,
                child_handle,
                class_name.map_or(ptr::null(), |name| name.as_ptr()),
                title.map_or(ptr::null(), |title| title.as_ptr()),
            ) != 0 => Option
        }
    }
}

/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow
pub fn get_foreground() -> Option<isize> {
    call! { GetForegroundWindow() != 0 => Option }
}

pub fn set_foreground(handle: isize) -> Result<()> {
    call_BOOL! { SetForegroundWindow(handle) }
}

/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint
pub fn from_point(point: POINT) -> Option<isize> {
    call! { WindowFromPoint(point) != 0 => Option }
}

/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint
pub fn from_physical_point(point: POINT) -> Option<isize> {
    call! { WindowFromPhysicalPoint(point) != 0 => Option }
}

/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow
pub fn get_desktop_window() -> Option<isize> {
    call! { GetDesktopWindow() != 0 => Option }
}

/// [documentation]: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent
pub fn get_paranet(handle: isize) -> Result<Option<isize>> {
    call! { GetParent(handle) != 0 => Result<Option> }
}

pub fn get_shell_window() -> Option<isize> {
    call! { GetShellWindow() != 0 => Option }
}

pub fn get_ancestor(handle: isize, flags: GET_ANCESTOR_FLAGS) -> Option<isize> {
    call! { GetAncestor(handle, flags) != 0 => Option }
}

pub fn get_process_and_thread_ids(handle: isize) -> Result<(u32, u32)> {
    let mut pid = 0;
    let thread_id = call! {
        GetWindowThreadProcessId(
            handle,
            &mut pid,
        ) == 0 => return if Error
    };
    Ok((pid, thread_id))
}

pub fn get_text_len(handle: isize) -> Result<i32> {
    call! { GetWindowTextLengthW(handle) != 0 => SetError }
}

pub fn get_text(handle: isize) -> Result<U16CString> {
    let mut len = call! { GetWindowTextLengthW(handle) != 0 => SetError }?;
    let mut buffer = vec![0; len as usize];
    len = call! {
        GetWindowTextW(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as i32
        ) == 0 => return if Error;
    };
    // Safety: `buffer` is valid pointer to a null-terminated string that has no interior null values
    Ok(unsafe { U16CString::from_ptr_unchecked(buffer.as_ptr(), len as usize) })
}

pub fn get_text_with_buffer(handle: isize, buffer: &mut [u16]) -> Result<u32> {
    call! {
        GetWindowTextW(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as i32,
        ) != 0 => u32
    }
}

pub fn get_top_child(handle: isize) -> Result<Option<isize>> {
    call! { GetTopWindow(handle) != 0 => Result<Option> }
}

pub fn get_window(handle: isize, flags: GET_WINDOW_CMD) -> Result<Option<isize>> {
    call! { GetWindow(handle, flags) != 0 => Result<Option> }
}

pub fn allow_set_foreground(pid: u32) -> Result<()> {
    call_BOOL! { AllowSetForegroundWindow(pid) }
}

pub fn is_child(parent_handle: isize, child_handle: isize) -> bool {
    from_BOOL!(unsafe { IsChild(parent_handle, child_handle) })
}

pub fn is_minimized(handle: isize) -> bool {
    from_BOOL!(unsafe { IsIconic(handle) })
}

pub fn is_visible(handle: isize) -> bool {
    from_BOOL!(unsafe { IsWindowVisible(handle) })
}

pub fn is_maximized(handle: isize) -> bool {
    from_BOOL!(unsafe { IsZoomed(handle) })
}

pub fn exists(handle: isize) -> bool {
    from_BOOL!(unsafe { IsWindow(handle) })
}

pub fn arrange_iconic_windows(handle: isize) -> Result<u32> {
    call! { ArrangeIconicWindows(handle) != 0 }
}

pub fn logical_to_physical_point(handle: isize, point: POINT) -> Result<POINT> {
    call_BOOL! {
        LogicalToPhysicalPoint(
            handle,
            &mut converted,
        ) -> mut converted = point
    }
}

pub fn open_minimized(handle: isize) -> Result<()> {
    call_BOOL! { OpenIcon(handle) }
}

pub fn r#move(handle: isize, point: POINT, height: i32, width: i32, redraw: bool) -> Result<()> {
    call_BOOL! { MoveWindow(handle, point.x, point.y, height, width, to_BOOL!(redraw)) }
}

pub fn to_top(handle: isize) -> Result<()> {
    call_BOOL! { BringWindowToTop(handle) }
}

pub fn child_from_point(handle: isize, point: POINT, flags: CWP_FLAGS) -> Result<Option<isize>> {
    call! {
        ChildWindowFromPointEx(handle, point, flags) != 0 => Result<Option>
    }
}

pub fn real_chold_from_point(handle: isize, point: POINT) -> Result<Option<isize>> {
    call! { RealChildWindowFromPoint(handle, point) != 0 => Result<Option> }
}

pub fn minimize(handle: isize) -> Result<()> {
    call_BOOL! { CloseWindow(handle) }
}

pub fn close(handle: isize) -> Result<()> {
    call_BOOL! { DestroyWindow(handle) }
}

pub fn set_text(handle: isize, text: &U16CStr) -> Result<()> {
    call_BOOL! { SetWindowTextW(handle, text.as_ptr()) }
}

pub fn set_parent(child_handle: isize, new_parent_handle: isize) -> Result<isize> {
    call! { SetParent(child_handle, new_parent_handle) != 0 }
}

pub fn show(handle: isize, show_command: SHOW_WINDOW_CMD) -> Result<bool> {
    call_BOOL! { ShowWindow(handle, show_command) -> bool }
}

pub fn show_async(handle: isize, show_command: SHOW_WINDOW_CMD) -> Result<()> {
    call_BOOL! { ShowWindowAsync(handle, show_command) }
}

pub fn destroy_icon(icon_handle: isize) -> Result<()> {
    call_BOOL! { DestroyIcon(icon_handle) }
}
