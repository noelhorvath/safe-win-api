use std::ffi::OsStr;
use std::sync::mpsc::Sender;

use win_api_wrapper::core::WinError;
use win_api_wrapper::win32::foundation::LRESULT;`
use win_api_wrapper::win32::system::library_loader;
use win_api_wrapper::win32::ui::messaging::{
    HookProcedure, PeekMessageRemoveType, SendMessageTimoutFlags, WINDOWS_HOOK_ID,
};
use win_api_wrapper::win32::{
    foundation::{HWND, LPARAM, NULL_HANDLE, WPARAM},
    ui::messaging::{
        self, MSG, WH_CALLWNDPROC, WH_CALLWNDPROCRET, WH_CBT, WH_DEBUG, WH_FOREGROUNDIDLE,
        WH_GETMESSAGE, WH_KEYBOARD, WH_KEYBOARD_LL, WH_MOUSE, WH_MOUSE_LL, WH_MSGFILTER, WH_SHELL,
        WH_SYSMSGFILTER,
    },
};

use crate::common::{As, ConversionError};
use crate::thread::Thread;
use crate::{common::Point, window::Window};

type Result<T> = ::core::result::Result<T, MessagingError>;

#[derive(Debug)]
pub enum MessagingError {
    WinError { code: i32, message: String },
}

impl From<WinError> for MessagingError {
    fn from(value: WinError) -> Self {
        MessagingError::WinError {
            code: value.code().0,
            message: value.message().to_string_lossy(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Message {
    window: Option<Window>,
    message_id: u32,
    parameters: (usize, isize),
    time: u32,
    cursor_position: Point<i32>,
}

impl Message {
    pub fn new(
        window: Option<Window>,
        message_id: u32,
        parameters: (usize, isize),
        time: u32,
        cursor_position: Point<i32>,
    ) -> Self {
        Message {
            window,
            message_id,
            parameters,
            time,
            cursor_position,
        }
    }

    pub fn is_partial(&self) -> bool {
        self.time == 0 && self.cursor_position == Point::default()
    }

    pub fn is_hook_message(&self) -> bool {
        self.window.is_none() && self.is_partial()
    }

    pub fn new_broadcast(message_id: u32, parameters: (usize, isize)) -> Self {
        Message::new_partial(Some(Window::BROADCAST_WINDOW), message_id, parameters)
    }

    pub fn is_broadcast(&self) -> bool {
        self.window()
            .is_some_and(|window| window == Window::BROADCAST_WINDOW)
    }

    pub fn new_partial(
        window: Option<Window>,
        message_id: u32,
        parameters: (usize, isize),
    ) -> Self {
        Message::new(window, message_id, parameters, 0, Point::default())
    }

    pub fn get(window: Option<Window>, min_filter: u32, max_filter: u32) -> Result<Message> {
        messaging::get(window.map_or(0, |w| w.handle()), min_filter, max_filter)
            .map(Message::from)
            .map_err(MessagingError::from)
    }

    pub fn peek(
        window: Option<Window>,
        min_filter: u32,
        max_filter: u32,
        remove_type: PeekMessageRemoveType,
    ) -> Option<Message> {
        messaging::peek(
            window.map_or(0, |w| w.handle()),
            min_filter,
            max_filter,
            remove_type,
        )
        .map(Message::from)
    }

    pub fn window(&self) -> Option<Window> {
        self.window
    }

    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    pub fn parameters(&self) -> (usize, isize) {
        self.parameters
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn cursor_position(&self) -> Point<i32> {
        self.cursor_position
    }

    pub fn dispatch(self) -> isize {
        messaging::dispatch(self.into())
    }

    pub fn post(self) -> Result<()> {
        messaging::post(
            self.window.map_or(0, |w| w.handle()),
            self.message_id,
            self.parameters,
        )
        .map_err(MessagingError::from)
    }

    pub fn post_thread(self, thread: Thread) -> Result<()> {
        messaging::post_thread(thread.id(), self.message_id, self.parameters)
            .map_err(MessagingError::from)
    }

    pub fn process_with_default_procedure(&self) -> isize {
        messaging::use_default_procedure(
            self.window.map_or(0, |w| w.handle()),
            self.message_id,
            self.parameters,
        )
    }

    pub fn send(self) -> isize {
        messaging::send(
            self.window.map_or(0, |w| w.handle()),
            self.message_id,
            self.parameters,
        )
    }

    pub fn send_notify(self) -> Result<()> {
        messaging::send_notify(
            self.window.map_or(0, |w| w.handle()),
            self.message_id,
            self.parameters,
        )
        .map_err(MessagingError::from)
    }

    unsafe extern "system" fn send_async_callback(
        hwnd: HWND,
        message: u32,
        callback_parameter: usize,
        processing_result: LRESULT,
    ) {
        let sender = (*(callback_parameter as *const Sender<Message>)).clone();
        if let Err(error) = sender.send(Message::new_partial(
            if hwnd.0 != NULL_HANDLE {
                Some(Window::new(hwnd.0))
            } else {
                None
            },
            message,
            (0, processing_result.0),
        )) {
            println!(
                "SendMessageCallback -> Error sending handle to receiver: {:?}",
                error
            );
        }
    }

    pub fn send_callback_with_sender(self, sender: &Sender<Message>) -> Result<()> {
        messaging::send_callback(
            self.window.map_or(0, |w| w.handle()),
            self.message_id,
            self.parameters,
            Self::send_async_callback,
            sender as *const Sender<Message> as usize,
        )
        .map_err(MessagingError::from)
    }

    pub fn send_timeout(self, timeout: u32, timout_flags: SendMessageTimoutFlags) -> Result<usize> {
        messaging::send_timeout(
            self.window.map_or(0, |w| w.handle()),
            self.message_id,
            self.parameters,
            timout_flags,
            timeout,
        )
        .map_err(MessagingError::from)
    }

    pub fn translate(self) -> bool {
        messaging::translate(self.into())
    }
}

impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("window", &self.window.is_some())
            .field("message_id", &self.message_id)
            .field("paramters", &self.parameters)
            .field("time", &self.time)
            .field("cursor_position", &self.cursor_position)
            .finish()
    }
}

impl From<MSG> for Message {
    fn from(value: MSG) -> Self {
        Message::new(
            if value.hwnd.0 != 0 {
                Some(Window::new(value.hwnd.0))
            } else {
                None
            },
            value.message,
            (value.wParam.0, value.lParam.0),
            value.time,
            value.pt.into(),
        )
    }
}

impl From<Message> for MSG {
    fn from(value: Message) -> Self {
        MSG {
            hwnd: HWND(value.window.map_or(NULL_HANDLE, |window| window.handle())),
            message: value.message_id,
            wParam: WPARAM(value.parameters.0),
            lParam: LPARAM(value.parameters.1),
            time: value.time,
            pt: value.cursor_position.into(),
        }
    }
}

pub fn last_sent_from_other_thread_info() -> u32 {
    messaging::in_send()
}

pub fn is_sender_blocked() -> bool {
    messaging::is_sender_blocked()
}

pub fn last_message_time() -> i32 {
    messaging::get_last_time()
}

pub fn last_cursor_position() -> Point<i32> {
    messaging::get_last_cursor_position().into()
}

pub fn post_quit(exit_code: i32) {
    messaging::post_quit(exit_code)
}

pub fn reply(processing_result: isize) -> bool {
    messaging::reply(processing_result)
}

pub fn register_message(message: Box<OsStr>) -> Result<u32> {
    messaging::register_message(message).map_err(MessagingError::from)
}

pub fn wait_for_message() -> Result<()> {
    messaging::wait().map_err(MessagingError::from)
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookId {
    MessageFilter = WH_MSGFILTER.0,
    Keyboard = WH_KEYBOARD.0,
    GetMessage,
    CallWindowProcedure,
    Cbt,
    SystemMessageFilter,
    Mouse,
    Debug = WH_DEBUG.0,
    Shell,
    ForegroundIdle,
    CallWindowProcedureReturn,
    KeyboardLowLevel,
    MouseLowLevel,
}

impl As<i32> for HookId {
    fn as_type(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<WINDOWS_HOOK_ID> for HookId {
    type Error = ConversionError;
    fn try_from(value: WINDOWS_HOOK_ID) -> ::core::result::Result<Self, Self::Error> {
        match value {
            WH_MSGFILTER => Ok(Self::MessageFilter),
            WH_KEYBOARD => Ok(Self::Keyboard),
            WH_GETMESSAGE => Ok(Self::GetMessage),
            WH_CALLWNDPROC => Ok(Self::CallWindowProcedure),
            WH_CBT => Ok(Self::Cbt),
            WH_SYSMSGFILTER => Ok(Self::SystemMessageFilter),
            WH_MOUSE => Ok(Self::Mouse),
            WH_DEBUG => Ok(Self::Debug),
            WH_SHELL => Ok(Self::Shell),
            WH_FOREGROUNDIDLE => Ok(Self::ForegroundIdle),
            WH_CALLWNDPROCRET => Ok(Self::CallWindowProcedureReturn),
            WH_KEYBOARD_LL => Ok(Self::KeyboardLowLevel),
            WH_MOUSE_LL => Ok(Self::MouseLowLevel),
            _ => Err(ConversionError::InvalidValue),
        }
    }
}

impl From<HookId> for WINDOWS_HOOK_ID {
    fn from(value: HookId) -> Self {
        WINDOWS_HOOK_ID(value.as_type())
    }
}

pub struct Hook {
    handle: isize,
    id: HookId,
    procedure: HookProcedure,
    thread: Option<Thread>,
}

impl Hook {
    fn new(handle: isize, id: HookId, procedure: HookProcedure, thread: Option<Thread>) -> Self {
        Hook {
            handle,
            id,
            procedure,
            thread,
        }
    }

    pub fn set(
        id: HookId,
        procedure: HookProcedure,
        procedure_module_name: Option<Box<OsStr>>,
        associated_thread: Option<Thread>,
    ) -> Result<Hook> {
        messaging::set_windows_hook(
            id.into(),
            procedure,
            library_loader::get_module_handle(procedure_module_name)?,
            associated_thread.map_or(0, |thread| thread.id()),
        )
        .map(|handle| Hook::new(handle, id, procedure, associated_thread))
        .map_err(MessagingError::from)
    }

    pub fn id(&self) -> HookId {
        self.id
    }

    pub fn procedure(&self) -> HookProcedure {
        self.procedure
    }

    pub fn thread(&self) -> Option<Thread> {
        self.thread
    }

    fn unhook(&self) -> Result<()> {
        messaging::unhook_windows_hook(self.handle).map_err(MessagingError::from)
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        if let Err(error) = self.unhook() {
            println!("Failed to unhook a {:?} hook: {:?}", self.id, error)
        }
    }
}
