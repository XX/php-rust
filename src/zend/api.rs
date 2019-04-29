use std::mem;
use std::os::raw::{c_char, c_int};
use crate::zend::{
    zend_type, zval, zend_execute_data, zend_internal_arg_info, ZEND_RESULT_CODE,
};

pub type FunctionEntry = zend_function_entry;

pub type zif_handler = Option<unsafe extern fn(execute_data: *mut zend_execute_data, return_value: *mut zval)>;

/* Parameter parsing API */

pub const ZEND_PARSE_PARAMS_QUIET: zend_type = 1 << 1;
pub const ZEND_PARSE_PARAMS_THROW: zend_type = 1 << 2;

extern {
    pub fn zend_parse_parameters(num_args: c_int, type_spec: *const c_char, ...) -> ZEND_RESULT_CODE;
    pub fn zend_parse_parameters_ex(flags: c_int, num_args: c_int, type_spec: *const c_char, ...) -> ZEND_RESULT_CODE;
    pub fn zend_parse_parameters_throw(num_args: c_int, type_spec: *const c_char, ...) -> ZEND_RESULT_CODE;
    pub fn zend_zval_type_name(arg: *const zval) -> *mut c_char;

    pub fn zend_parse_method_parameters(num_args: c_int, this_ptr: *mut zval, type_spec: *const c_char, ...) -> ZEND_RESULT_CODE;
    pub fn zend_parse_method_parameters_ex(flags: c_int, num_args: c_int, this_ptr: *mut zval, type_spec: *const c_char, ...) -> ZEND_RESULT_CODE;

    pub fn zend_parse_parameter(flags: c_int, arg_num: c_int, arg: *mut zval, spec: *const c_char, ...) -> ZEND_RESULT_CODE;
}

/* End of parameter parsing API */


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zend_function_entry {
    pub fname: *const c_char,
    pub handler: zif_handler,
    pub arg_info: *const zend_internal_arg_info,
    pub num_args: u32,
    pub flags: u32,
}

impl Default for zend_function_entry {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}