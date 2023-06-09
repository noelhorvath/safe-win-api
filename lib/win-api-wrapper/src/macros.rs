#[doc(hidden)]
#[macro_export]
macro_rules! call_BOOL {
    { $func:ident($($arg:expr), * $(,)?) } => {
        $crate::handle_BOOL!($func($($arg), *) -> ())
    };
    { $func:ident($($arg:expr), * $(,)?) return Error $(;)? } => {
        $crate::handle_BOOL!($func($($arg), *) return Error )
    };
    { $func:ident($($arg:expr), * $(,)?) -> if Error == $error_val:tt return $(;)? else return $def_ret_val:expr $(;)? } => {
        $crate::handle_BOOL!($func($($arg), *) if Error == $error_val return else return $def_ret_val)
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut !$res:ident } => {
        {
            let mut $res = 0;
            $crate::handle_BOOL!($func($($arg), *) -> $crate::from_BOOL!(!$res))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $res:ident } => {
        {
            let mut $res = 0;
            $crate::handle_BOOL!($func($($arg), *) -> $crate::from_BOOL!($res))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $res:ident: $res_type:ty } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let mut $res = $crate::default_sized!($res_type);
            $crate::handle_BOOL!($func($($arg), *) -> $res)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $res:ident: $res_type:ty as $ret_type:ty } => {
        {
            let mut $res = <$res_type>::default();
            $crate::handle_BOOL!($func($($arg), *) -> $res as $ty)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $res:ident: $res_type:ty } => {
        {
            let mut $res = <$res_type>::default();
            $crate::handle_BOOL!($func($($arg), *) -> $res.to())
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $res:ident = $init_val:expr} => {
        {
            let mut $res = $init_val;
            $crate::handle_BOOL!($func($($arg), *) -> $res)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut ($($res_var:ident), *): $res_tuple_type:ty } => {
        {
            let ($(mut $res_var), *) = <$res_tuple_type>::default();
            $crate::handle_BOOL!($func($($arg), *) -> ($($res_var), *))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> To { mut $res:ident: $res_type:ty $(;)? } } => {
        {
            let mut $res = <$res_type>::default();
            $crate::handle_BOOL!($func($($arg), *) -> $res.to())
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> From { mut $res:ident = $init_val:expr $(;)? } } => {
        {
            let mut $res = $init_val;
            $crate::handle_BOOL!($func($($arg), *) -> $res.into())
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> Option { mut $res:ident = $init_val:expr $(;)? } } => {
        {
            let mut $res = $init_val;
            $crate::handle_BOOL!($func($($arg), *) Option { $res })
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> Result<Option>
        {
            mut $res:ident = $init_val:expr;
            $win_error:tt => None $(;)?
        }
    } => {
        {
            let mut $res = $init_val;
            $crate::handle_BOOL!($func($($arg), *) Result<Option> { $res, $win_error => None })
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_BOOL {
    ($func:ident($($arg:expr), *) -> $ret_expr:expr) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if res == 0 {
                Err($crate::win32::core::Win32Error::get_last())
            } else {
                Ok($ret_expr)
            }
        }
    };
    ($func:ident($($arg:expr), *) return Error) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if res == 0 {
                return Err($crate::win32::core::Win32Error::get_last())
            }
        }
    };
    ($func:ident($($arg:expr), *) if Error == $error_val:tt return else return $def_ret_val:expr) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if res == 0 {
                let error = $crate::win32::core::Win32Error::get_last();
                if error.code == $error_val {
                    return Err(error);
                }
            } else {
                return Ok($def_ret_val);
            }
        }
    };
    ($func:ident($($arg:expr), *) Option { $ret_expr:expr }) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if res == 0 {
                None
            } else {
                Some($ret_expr)
            }
        }
    };
    ($func:ident($($arg:expr), *) Result<Option> { $ret_expr:expr, $win_error:tt => None }) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            match res as u32 {
                0 => Ok(Some($ret_expr)),
                $win_error => Ok(None),
                _ => Err($crate::win32::core::Win32Error::get_last())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_num {
    { $func:ident($($arg:expr), * $(,)?) != $error_val:literal } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(Ok => res, res, $error_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) != $error_val:literal To } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(Ok => res, res.to(), $error_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) != $error_val:literal as $ret_type:ty } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(Ok => res, res as $ret_type, $error_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) != $error_val:expr } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(Ok => res, res, $error_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) == $success_val:literal } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(Err => res, $success_val)
        }
    };
    { ($func:ident($($arg:expr), * $(,)?) == $error_val:expr) return Error $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_val = unsafe { $func($($arg),*) };
            $crate::handle_num!(return Err => res_val, $error_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident: $ret_type:ty $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let $ret_val = <$ret_type>::default();
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(Err => res, $ret_val, windows_sys::Win32::Foundation::ERROR_SUCCESS)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) return Error $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_val = unsafe { $func($($arg),*) };
            $crate::handle_num!(return Err $res_val:ident)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_num {
    (Err => $res_val:ident, $success_val:literal) => {
        if $res_val == $success_val {
            Ok(())
        } else {
            Err($crate::win32::core::Win32Error::get_last())
        }
    };
    (Ok => $res_val:ident, $ret_expr:expr, $error_val:expr) => {
        if $res_val != $error_val {
            Ok($ret_expr)
        } else {
            Err($crate::win32::core::Win32Error::get_last())
        }
    };
    (return Err => $res_val:ident, $error_val:expr) => {
        if $res_val == $error_val {
            return Err($crate::win32::core::Win32Error::get_last());
        } else {
            $res_val
        }
    };
    (return Err $res_val:ident) => {
        if $res_val != windows_sys::Win32::Foundation::ERROR_SUCCESS {
            return Err($crate::win32::core::Win32Error::from_code($res_val));
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! default_sized {
    ($sized:ty) => {
        // Safety: The sized type is not a reference or a pointer.
        unsafe { core::mem::zeroed::<$sized>() }
    };
    (mut $entry:ty: SnapshotEntry) => {{
        // Safety: The sized type is not a reference or a pointer.
        let mut entry = unsafe { core::mem::zeroed::<$entry>() };
        entry.dwSize = size_of::<$entry>() as u32;
        entry
    }};
}
