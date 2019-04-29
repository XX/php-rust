use std::ptr;
use std::os::raw::c_int;
use crate::{c_str, zend};

pub struct ExecuteData(zend::ExecuteData);

impl ExecuteData {
    #[inline]
    pub fn num_args(&self) -> u32 {
        unsafe { self.0.num_args() }
    }

    #[inline]
    pub fn parse_double_parameter(&self) -> Option<f64> {
        let mut d: f64 = 0.0;
        if unsafe { zend::zend_parse_parameters(
                self.num_args() as c_int, c_str!("d"), &mut d as *mut _
        ) }.is_success() {
            Some(d)
        } else {
            None
        }
    }

    #[inline]
    pub fn parse_array_parameter(&self) -> Option<&zend::Array> {
        let val: *mut zend::Zval = ptr::null_mut();
        unsafe {
            if zend::zend_parse_parameters(
                self.num_args() as c_int, c_str!("a"), &val
            ).is_success() {
                Some(&*(*val).value.arr)
            } else {
                None
            }
        }
    }
}