use std::slice;
use crate::zend;

pub struct ZString(*mut zend::String);

impl ZString {
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(
                slice::from_raw_parts((*self.0).val.as_ptr(), (*self.0).len)
            )
        }
    }
}

// todo: unsafe!!
impl From<*mut zend::String> for ZString {
    fn from(from: *mut zend::String) -> Self {
        ZString(from)
    }
}