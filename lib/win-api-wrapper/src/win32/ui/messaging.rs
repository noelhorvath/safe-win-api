use crate::core::Result;
use crate::{call, call_BOOL, default_sized, from_BOOL};
use core::ptr;
use widestring::U16CStr;
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DefWindowProcW, DispatchMessageW, GetMessageExtraInfo, GetMessagePos,
    GetMessageTime, GetMessageW, InSendMessageEx, PeekMessageW, PostMessageW, PostQuitMessage,
    PostThreadMessageW, RegisterWindowMessageW, ReplyMessage, SendMessageCallbackW,
    SendMessageTimeoutW, SendMessageW, SendNotifyMessageW, SetMessageExtraInfo, SetWindowsHookExW,
    TranslateMessage, UnhookWindowsHookEx, WaitMessage,
};

pub use windows_sys::Win32::UI::WindowsAndMessaging::{
    HWND_TOPMOST, ISMEX_NOSEND, ISMEX_REPLIED, ISMEX_SEND, MSG,
    PEEK_MESSAGE_REMOVE_TYPE as PeekMessageRemoveType,
    SEND_MESSAGE_TIMEOUT_FLAGS as SendMessageTimoutFlags, SMTO_ABORTIFHUNG, SMTO_BLOCK,
    SMTO_ERRORONEXIT, SMTO_NORMAL, SMTO_NOTIMEOUTIFNOTHUNG, WH_CALLWNDPROC, WH_CALLWNDPROCRET,
    WH_CBT, WH_DEBUG, WH_FOREGROUNDIDLE, WH_GETMESSAGE, WH_KEYBOARD, WH_KEYBOARD_LL, WH_MOUSE,
    WH_MOUSE_LL, WH_MSGFILTER, WH_SHELL, WH_SYSMSGFILTER, WINDOWS_HOOK_ID, WM_QUIT,
};

pub type HookProcedure =
    unsafe extern "system" fn(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;

pub type SendAsyncCallback =
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: usize, param3: LRESULT) -> ();

pub fn call_next_hook(hook_handle: isize, hook_code: i32, parameters: (usize, isize)) -> isize {
    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        CallNextHookEx(hook_handle, hook_code, parameters.0, parameters.1)
    }
}

pub fn dispatch(message: &MSG) -> isize {
    unsafe { DispatchMessageW(message) }
}

pub fn get(window_handle: isize, message_filter_min: u32, message_filter_max: u32) -> Result<MSG> {
    call! {
        GetMessageW(
            &mut message,
            window_handle,
            message_filter_min,
            message_filter_max,
        ) != -1 => mut message = default_sized!(MSG)
    }
}

pub fn get_last_time() -> i32 {
    unsafe { GetMessageTime() }
}

pub fn get_last_cursor_position() -> POINT {
    let result = unsafe { GetMessagePos() };
    POINT {
        x: (result >> u16::BITS) as i32,
        y: result as i32,
    }
}

pub fn get_extra_info() -> isize {
    unsafe { GetMessageExtraInfo() }
}

pub fn set_extra_info(extra_info: isize) -> isize {
    unsafe { SetMessageExtraInfo(extra_info) }
}

pub fn peek(
    window_handle: isize,
    filter_min: u32,
    filter_max: u32,
    message_remove_type: PeekMessageRemoveType,
) -> Option<MSG> {
    call_BOOL! {
        PeekMessageW(
            &mut message,
            window_handle,
            filter_min,
            filter_max,
            message_remove_type,
        ) -> Option { mut message = default_sized!(MSG) }
    }
}

pub fn post(window_handle: isize, message: u32, parameters: (usize, isize)) -> Result<()> {
    call_BOOL! {
        PostMessageW(window_handle, message, parameters.0, parameters.1)
    }
}

pub fn post_quit(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) }
}

pub fn post_thread(thread_id: u32, message: u32, parameters: (usize, isize)) -> Result<()> {
    call_BOOL! {
        PostThreadMessageW(thread_id, message, parameters.0, parameters.1)
    }
}

pub fn reply(processing_result: isize) -> bool {
    from_BOOL!(unsafe { ReplyMessage(processing_result) })
}

pub fn send(window_handle: isize, message: u32, parameters: (usize, isize)) -> isize {
    unsafe { SendMessageW(window_handle, message, parameters.0, parameters.1) }
}

pub fn send_notify(window_handle: isize, message: u32, parameters: (usize, isize)) -> Result<()> {
    call_BOOL! {
        SendNotifyMessageW(window_handle, message, parameters.0, parameters.1)
    }
}

pub fn send_timeout(
    window_handle: isize,
    message: u32,
    parameters: (usize, isize),
    timeout_flags: SendMessageTimoutFlags,
    timeout: u32,
) -> Result<usize> {
    call! {
        SendMessageTimeoutW(
            window_handle,
            message,
            parameters.0,
            parameters.1,
            timeout_flags,
            timeout,
            &mut processing_result,
        ) == 0 => mut processing_result: usize
    }
}

pub fn send_callback(
    window_handle: isize,
    message: u32,
    parameters: (usize, isize),
    callback: SendAsyncCallback,
    callback_parameter: usize,
) -> Result<()> {
    call_BOOL! {
        SendMessageCallbackW(
            window_handle,
            message,
            parameters.0,
            parameters.1,
            Some(callback),
            callback_parameter,
        )
    }
}

#[inline]
pub fn in_send() -> u32 {
    unsafe { InSendMessageEx(ptr::null()) }
}

#[inline]
pub fn is_sender_blocked() -> bool {
    in_send() & (ISMEX_REPLIED | ISMEX_SEND) == ISMEX_SEND
}

pub fn translate(message: MSG) -> bool {
    from_BOOL!(unsafe { TranslateMessage(&message) })
}

pub fn use_default_procedure(
    window_handle: isize,
    message: u32,
    parameters: (usize, isize),
) -> isize {
    unsafe { DefWindowProcW(window_handle, message, parameters.0, parameters.1) }
}

pub fn set_windows_hook(
    hook_id: WINDOWS_HOOK_ID,
    hook_procedure: HookProcedure,
    module_handle: isize,
    thread_id: u32,
) -> Result<isize> {
    call! {
        SetWindowsHookExW(
            hook_id,
            Some(hook_procedure),
            module_handle,
            thread_id,
        ) != 0
    }
}

pub fn unhook_windows_hook(hook_handle: isize) -> Result<()> {
    call_BOOL! { UnhookWindowsHookEx(hook_handle) }
}

pub fn wait() -> Result<()> {
    call_BOOL! { WaitMessage() }
}

pub fn register_message(message: &U16CStr) -> Result<u32> {
    call! { RegisterWindowMessageW(message.as_ptr()) != 0 }
}
