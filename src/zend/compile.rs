use std::mem;
use std::os::raw::{c_void, c_int, c_char};
use crate::zend::{
    zend_uchar, zend_bool, zval, zend_string, zend_array, HashTable, zif_handler,
    ZEND_MAX_RESERVED_RESOURCES
};

pub type ExecuteData = zend_execute_data;
pub type Function = zend_function;
pub type FunctionCommon = zend_function_common;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zend_execute_data {
    /// executed opline
    pub opline: *const zend_op,
    /// current call
    pub call: *mut zend_execute_data,
    pub return_value: *mut zval,
    /// executed funcrion
    pub func: *mut zend_function,
    /// this + call_info + num_args
    pub This: zval,
    pub prev_execute_data: *mut zend_execute_data,
    pub symbol_table: *mut zend_array,
    /// cache op_array.run_time_cache
    pub run_time_cache: *mut *mut c_void,
    pub literals: *mut zval,
}

impl zend_execute_data {
    #[inline]
    pub unsafe fn num_args(&self) -> u32 {
        self.This.u2.num_args
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zend_op {
    pub handler: *const c_void,
    pub op1: znode_op,
    pub op2: znode_op,
    pub result: znode_op,
    pub extended_value: u32,
    pub lineno: u32,
    pub opcode: zend_uchar,
    pub op1_type: zend_uchar,
    pub op2_type: zend_uchar,
    pub result_type: zend_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union znode_op {
    pub constant: u32,
    pub var: u32,
    pub num: u32,
    /// Needs to be signed
    pub opline_num: u32,
    pub jmp_offset: u32,
}

impl Default for znode_op {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zend_internal_function {
    /* Common elements */
    pub type_: zend_uchar,
    /// bitset of arg_info.pass_by_reference
    pub arg_flags: [zend_uchar; 3usize],
    pub fn_flags: u32,
    pub function_name: *mut zend_string,
    pub scope: *mut c_void, // todo: use zend_class_entry instead of c_void
    pub prototype: *mut zend_function,
    pub num_args: u32,
    pub required_num_args: u32,
    pub arg_info: *mut zend_internal_arg_info,
    /* END of common elements */

    pub handler: zif_handler,
    pub module: *mut c_void, // todo; use zend_module_entry instead of c_void
    pub reserved: [*mut c_void; ZEND_MAX_RESERVED_RESOURCES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union zend_function {
    /// MUST be the first element of this struct!
    pub type_: zend_uchar,
    pub common: zend_function_common,
    pub op_array: zend_op_array,
    pub internal_function: zend_internal_function,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zend_function_common {
    /// never used
    pub type_: zend_uchar,
    /// bitset of arg_info.pass_by_reference
    pub arg_flags: [zend_uchar; 3usize],
    pub fn_flags: u32,
    pub function_name: *mut zend_string,
    pub scope: *mut c_void, // todo: use zend_class_entry instead of c_void
    pub prototype: *mut zend_function,
    pub num_args: u32,
    pub required_num_args: u32,
    pub arg_info: *mut zend_arg_info,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zend_arg_info {
    pub name: *mut zend_string,
    pub class_name: *mut zend_string,
    pub type_hint: zend_uchar,
    pub pass_by_reference: zend_uchar,
    pub allow_null: zend_bool,
    pub is_variadic: zend_bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zend_op_array {
    /* Common elements */
    pub type_: zend_uchar,
    /// bitset of arg_info.pass_by_reference
    pub arg_flags: [zend_uchar; 3usize],
    pub fn_flags: u32,
    pub function_name: *mut zend_string,
    pub scope: *mut c_void, // todo: use zend_class_entry instead of c_void
    pub prototype: *mut zend_function,
    pub num_args: u32,
    pub required_num_args: u32,
    pub arg_info: *mut zend_arg_info,
    /* END of common elements */

    pub refcount: *mut u32,
    pub this_var: u32,
    pub last: u32,
    pub opcodes: *mut zend_op,
    pub last_var: c_int,
    pub T: u32,
    pub vars: *mut *mut zend_string,
    pub last_brk_cont: c_int,
    pub last_try_catch: c_int,
    pub brk_cont_array: *mut zend_brk_cont_element,
    pub try_catch_array: *mut zend_try_catch_element,
    /// static variables support
    pub static_variables: *mut HashTable,
    pub filename: *mut zend_string,
    pub line_start: u32,
    pub line_end: u32,
    pub doc_comment: *mut zend_string,
    /// the linked list of delayed declarations
    pub early_binding: u32,
    pub last_literal: c_int,
    pub literals: *mut zval,
    pub cache_size: c_int,
    pub run_time_cache: *mut *mut c_void,
    pub reserved: [*mut c_void; ZEND_MAX_RESERVED_RESOURCES],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct zend_brk_cont_element {
    pub start: c_int,
    pub cont: c_int,
    pub brk: c_int,
    pub parent: c_int,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct zend_try_catch_element {
    pub try_op: u32,
    /// ketchup!
    pub catch_op: u32,
    pub finally_op: u32,
    pub finally_end: u32,
}

/// arg_info for internal functions
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zend_internal_arg_info {
    pub name: *const c_char,
    pub class_name: *const c_char,
    pub type_hint: zend_uchar,
    pub pass_by_reference: zend_uchar,
    pub allow_null: zend_bool,
    pub is_variadic: zend_bool,
}