use std::{ptr, slice};
use std::os::raw::c_int;
use crate::{c_str, zend, Zval};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExecuteData(zend::ExecuteData);

impl ExecuteData {
    #[inline]
    pub fn num_args(&self) -> u32 {
        unsafe { self.0.num_args() }
    }

    #[inline]
    pub fn parse_parameters(&self) -> Option<&mut [Zval]> {
        let mut args: *mut Zval = ptr::null_mut();
        let mut argc: c_int = 0;

        unsafe {
            if zend::zend_parse_parameters(
                self.num_args() as c_int, c_str!("*"), &mut args, &mut argc
            ).is_success() {
                Some(slice::from_raw_parts_mut(args, argc as usize))
            } else {
                None
            }
        }
    }
}