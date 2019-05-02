use std::{slice, cmp::PartialEq};
use crate::{zend, ToSafe};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

impl PartialEq for ZString {
    fn eq(&self, other: &ZString) -> bool {
        self.0 == other.0 || self.as_str() == other.as_str()
    }
}

// todo: unsafe!!
impl From<*mut zend::String> for ZString {
    fn from(from: *mut zend::String) -> Self {
        ZString(from)
    }
}

impl From<ZString> for *mut zend::String {
    fn from(from: ZString) -> Self {
        from.0
    }
}

impl From<ZString> for *const zend::String {
    fn from(from: ZString) -> Self {
        from.0
    }
}

impl ToSafe for *mut zend::String {
    type SafeType = ZString;

    fn into_safe(self) -> Self::SafeType {
        ZString(self)
    }
}