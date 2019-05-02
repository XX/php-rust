use libc::*;
use crate::{ExecuteData, Zval};

pub(crate) type StartupFunc = extern fn (type_: c_int, module_number: c_int) -> c_int;
pub(crate) type ShutdownFunc = extern fn (type_: c_int, module_number: c_int) -> c_int;
pub(crate) type InfoFunc = extern fn () ;
pub(crate) type GlobalsCtorFunc = extern fn (global: *const c_void) -> c_void;
pub(crate) type GlobalsDtorFunc = extern fn (global: *const c_void) -> c_void;
pub(crate) type PostDeactivateFunc = extern fn () -> c_int;
pub(crate) type HandlerFunc = extern fn (execute_data: &ExecuteData, retval: &mut Zval);

//pub struct ExecuteData {}

#[repr(C)]
pub struct ArgInfo {
    name: *const c_char,
    class_name: *const c_char,
    type_hint: c_uchar,
    pass_by_reference: c_uchar,
    allow_null: c_uchar,
    is_variadic: c_uchar,
}

impl ArgInfo {
    pub fn new(name: *const c_char, allow_null: c_uchar, is_variadic: c_uchar, pass_by_reference: c_uchar) -> ArgInfo {
        ArgInfo {
            name,
            class_name: std::ptr::null(),
            type_hint: 0,
            pass_by_reference,
            allow_null,
            is_variadic,
        }
    }
}

#[repr(C)]
pub struct Function {
    fname: *const c_char,
    handler: Option<HandlerFunc>,
    arg_info: *const  ArgInfo,
    num_args: u32,
    flags: u32,
}

impl Function {
    pub fn new(name: *const c_char, handler: HandlerFunc) -> Function {
        Function {
            fname: name,
            handler: Some(handler),
            arg_info: std::ptr::null(),
            num_args: 0,
            flags: 0,
        }
    }

    pub fn new_with_args(name: *const c_char, handler: HandlerFunc, args: Box<[ArgInfo]>) -> Function {
        let num_args = args.len() as u32;

        Function {
            fname: name,
            handler: Some(handler),
            arg_info: Box::into_raw(args) as *const ArgInfo,
            num_args: num_args - 1,
            flags: 0,
        }
    }

    pub fn end() -> Function {
        Function {
            fname: std::ptr::null(),
            handler: None,
            arg_info: std::ptr::null(),
            num_args: 0,
            flags: 0,
        }
    }
}
