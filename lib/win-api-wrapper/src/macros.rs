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
            if $crate::from_BOOL!(res) {
                Ok($ret_expr)
            } else {
                Err($crate::last_error!())
            }
        }
    };
    ($func:ident($($arg:expr), *) return Error) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if $crate::from_BOOL!(!res) {
                return Err($crate::last_error!())
            }
        }
    };
    ($func:ident($($arg:expr), *) if Error == $error_val:tt return else return $def_ret_val:expr) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if $crate::from_BOOL!(res) {
                return Ok($def_ret_val);
            } else {
                let error = $crate::last_error!();
                if error.code == $error_val {
                    return Err(error);
                }
            }
        }
    };
    ($func:ident($($arg:expr), *) Option { $ret_expr:expr }) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if $crate::from_BOOL!(res) {
                Some($ret_expr)
            } else {
                None
            }
        }
    };
    ($func:ident($($arg:expr), *) Result<Option> { $ret_expr:expr, $win_error:tt => None }) => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            if $crate::from_BOOL!(res) {
                Ok(Some($ret_expr))
            } else {
                let error = $crate::last_error!();
                if error.code == $win_error {
                    Ok(None)
                } else {
                    Err(error)
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_num {
    { $func:ident($($arg:expr), * $(,)?) != $check_val:expr } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(res, !=, $check_val, Ok(res), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) == $check_val:expr } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(res, ==, $check_val, Ok(()), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) $op:tt $check_val:expr => To } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(res, $op, $check_val, Ok(res.to()), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) $op:tt $check_val:expr => Option} => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(res, $op, $check_val, Some(res), $error_val, None)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) $op:tt $check_val:expr => as $ret_type:ty } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(res, $op, $error_val, Ok(res as $ret_type), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) == $error_val:expr => return Error $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_val = unsafe { $func($($arg),*) };
            $crate::handle_num!(res_val, ==, $error_val, res_val, return Err($crate::last_error!()))
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! last_error {
    () => {
        $crate::win32::core::Win32Error::get_last()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_from {
    ($code:expr) => {
        $crate::win32::core::Win32Error::from_code($code)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_num {
    ($res_val:ident, $op:tt, $check_val:expr, $ret_expr:expr, $ret_error:expr) => {
        if $res_val $op $check_val {
            $ret_expr
        } else {
            $ret_error
        }
    };
    ($res_val:ident, $op:tt, $check_val:expr, $ret_expr:expr, $ret_error_statement:stmt) => {
        if $res_val $op $check_val {
            ret_error_statement;
        } else {
            $res_val
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_WIN32_ERROR {
    { $func:ident($($arg:expr), * $(,)?) } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_error_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(res_error_code, ())
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident: $ret_type:ty $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let mut $ret_val = <$ret_type>::default();
            let res_error_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(res_error_code, $ret_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) return Error $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_error_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(return => Err($crate::error_from!(res_error_code)), res_error_code)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) return Error as Option $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_error_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(return => None, res_error_code)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_WIN32_ERROR {
    ($res_error_code:ident, $ret_expr:expr) => {
        if $res_error_code == windows_sys::Win32::Foundation::ERROR_SUCCESS {
            Err($crate::error_from!($res_error_code))
        } else {
            Ok($ret_expr)
        }
    };
    (return => $ret_error:expr, $res_error_code:ident) => {
        if $res_error_code != windows_sys::Win32::Foundation::ERROR_SUCCESS {
            return $ret_error;
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
