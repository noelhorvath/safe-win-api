#[doc(hidden)]
#[macro_export]
macro_rules! call_BOOL {
    { $func:ident($($arg:expr), * $(,)?) } => {
        #[allow(clippy::undocumented_unsafe_blocks)]
        let res = unsafe { $func($($arg),*) };
        $crate::handle_BOOL!(res, Ok(()), Err($crate::last_error!()))
    };
    { $func:ident($($arg:expr), * $(,)?) return Error $(;)? } => {
        #[allow(clippy::undocumented_unsafe_blocks)]
        let res = unsafe { $func($($arg),*) };
        $crate::handle_BOOL!(return res, Err($crate::last_error!()))
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut !$ret_val:ident } => {
        {
            let mut $ret_val = 0;
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($crate::from_BOOL!($ret_val)), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident } => {
        {
            let mut $ret_val = 0;
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($crate::from_BOOL!($ret_val)), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident: $ret_type:ty } => {
        #[allow(clippy::undocumented_unsafe_blocks)]
        {
            let mut $ret_val = $crate::default_sized!($ret_type);
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($ret_val), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident: $init_type:ty as $ret_type:ty } => {
        {
            let mut $ret_val = <$init_type>::default() as $ty;
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($ret_val), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident: $ret_type:ty } => {
        {
            let mut $ret_val = <$ret_type>::default();
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($ret_val), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident = $init_val:expr} => {
        {
            let mut $ret_val = $init_val;
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($ret_val), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut ($($ret_val:ident), *): $ret_tuple_type:ty } => {
        {
            let ($(mut $ret_val), *) = <$ret_tuple_type>::default();
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok(($($ret_val), *)), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> To { mut $ret_val:ident: $ret_type:ty $(;)? } } => {
        {
            let mut $res = <$res_type>::default();
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($ret_val.to()), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> From { mut $ret_val:ident = $init_val:expr $(;)? } } => {
        {
            let mut $ret_val = $init_val;
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Ok($ret_val.into()), Err($crate::last_error!()))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> Option { mut $ret_val:ident = $init_val:expr $(;)? } } => {
        {
            let mut $res = $init_val;
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(res, Some($ret_val), None)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> Result<Option>
        {
            mut $ret_val:ident = $init_val:expr;
            $error_val:tt => None $(;)?
        }
    } => {
        {
            let mut $ret_val = $init_val;
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_BOOL!(Option<Result> => res, $ret_val, $error_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> bool } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = $crate::from_BOOL!(unsafe { $func($($arg),*) });
            $crate::handle_BOOL!(bool => res, Ok(res))
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> if $error_val:tt return $(;)? else return $def_ret_val:expr $(;)? } => {
        #[allow(clippy::undocumented_unsafe_blocks)]
        let res = unsafe { $func($($arg),*) };
        $crate::handle_BOOL!(res if $error_val return else return $def_ret_val)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_BOOL {
    ($res:ident, $ret_expr:expr, $ret_error:expr) => {
        if $crate::from_BOOL!($res) {
            $ret_expr
        } else {
            $ret_error
        }
    };
    (return $res:ident, $ret_expr:expr) => {
        if $crate::from_BOOL!(!$res) {
            return $ret_expr;
        }
    };
    ($res:ident if $error_val:tt return else return $def_ret_val:expr) => {
        if $crate::from_BOOL!($res) {
            return Ok($def_ret_val);
        } else {
            let error = $crate::last_error!();
            if error.code.0 == $error_val as i32 {
                return Err(error);
            }
        }
    };
    (bool => $res:ident, $ret_expr:expr) => {
        if $res {
            $ret_expr
        } else {
            let error = $crate::last_error!();
            if error.is_success() {
                $ret_expr
            } else {
                Err(error)
            }
        }
    };
    (Option<Result> => $res:ident, $ret_expr:expr, $error_val:expr) => {
        if $crate::from_BOOL!($res) {
            Ok(Some($ret_expr))
        } else {
            let error = $crate::last_error!();
            if error.code.0 == $error_val as i32 {
                Ok(None)
            } else {
                Err(error)
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
    { $func:ident($($arg:expr), * $(,)?) != $check_val:expr => SetError } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(
                res,
                !=,
                $check_val,
                Ok(res),
                Err($crate::last_error!()),
                $crate::win32::foundation::set_last_error(windows_sys::Win32::Foundation::ERROR_SUCCESS)
            )
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
            $crate::handle_num!(res, $op, $check_val, Some(res), None)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) $op:tt $check_val:expr => Result<Option>} => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(Result<Option> => res, $op, $check_val, res, $crate::last_error!())
        }
    };
    { $func:ident($($arg:expr), * $(,)?) $op:tt $check_val:expr => as $ret_type:ty } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res = unsafe { $func($($arg),*) };
            $crate::handle_num!(res, $op, $check_val, Ok(res as $ret_type), Err($crate::last_error!()))
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
macro_rules! handle_num {
    ($res_val:ident, $op:tt, $check_val:expr, $ret_expr:expr, $ret_error:expr) => {
        if $res_val $op $check_val {
            $ret_expr
        } else {
            $ret_error
        }
    };
    ($res_val:ident, $op:tt, $check_val:expr, $ret_expr:expr, $ret_error:expr, $($pre_error:stmt); *) => {
        if $res_val $op $check_val {
            $ret_expr
        } else {
            $($pre_error)*
            $ret_error
        }
    };
    ($res_val:ident, $op:tt, $check_val:expr, $ret_error_statement:stmt) => {
        if $res_val $op $check_val {
            ret_error_statement
        } else {
            $res_val
        }
    };
    (Result<Option> => $res_val:ident, $op:tt, $check_val:expr, $ret_expr:expr, $get_error:expr) => {
        if $res_val $op $check_val {
            Ok(Some($ret_expr))
        } else {
            let error = $get_error;
            if error.is_success() {
                Ok(None)
            } else {
                Err(error)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_WIN32_ERROR {
    { $func:ident($($arg:expr), * $(,)?) } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(res_code, ())
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident: $ret_type:ty $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let mut $ret_val = <$ret_type>::default();
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(res_code, $ret_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) return Error $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(return => Err($crate::error_from!(res_code)), res_code)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) return Error as Option $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_WIN32_ERROR!(return => None, res_code)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_WIN32_ERROR {
    ($res_code:ident, $ret_expr:expr) => {
        if $res_code == windows_sys::Win32::Foundation::ERROR_SUCCESS {
            Err($crate::error_from!($res_code))
        } else {
            Ok($ret_expr)
        }
    };
    (return => $ret_error:expr, $res_code:ident) => {
        if $res_code != windows_sys::Win32::Foundation::ERROR_SUCCESS {
            return $ret_error;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_HRESULT {
    { $func:ident($($arg:expr), * $(,)?) } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_HRESULT!(res_code, ())
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident: $ret_type:ty $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let mut $ret_val = <$ret_type>::default();
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_HRESULT!(res_code, $ret_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) -> mut $ret_val:ident = $init_val:expr $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let mut $ret_val = $init_val;
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_HRESULT!(res_code, $ret_val)
        }
    };
    { $func:ident($($arg:expr), * $(,)?) return Error $(;)? } => {
        {
            #[allow(clippy::undocumented_unsafe_blocks)]
            let res_code = unsafe { $func($($arg),*) };
            $crate::handle_HRESULT!(return res_code)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_HRESULT {
    ($res_code:ident, $ret_expr:expr) => {
        if $res_code < 0 {
            Err($crate::error_from!($res_code))
        } else {
            Ok($ret_expr)
        }
    };
    (return $res_code:ident) => {
        if $res_code < 0 {
            return Err($crate::error_from!($res_code));
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
