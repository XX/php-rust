use std::ptr;
use std::os::raw::c_void;
use crate::zend;

pub use array::*;
pub use execute_data::*;
pub use refcounted::*;
pub use string::*;
pub use zval::*;

pub use zend::Long;
pub use zend::Ulong;
pub use zend::OffT;
pub use zend::Bool;
pub use zend::Uchar;
pub use zend::Type;

pub mod array;
pub mod execute_data;
pub mod refcounted;
pub mod string;
pub mod zval;

pub type VoidPtr = *const c_void;
pub type VoidPtrMut = *mut c_void;

pub trait ToSafe {
    type SafeType;

    fn into_safe(self) -> Self::SafeType where Self: Sized {
        unsafe { ptr::read(&self as *const Self as *const Self::SafeType) }
    }

    fn as_safe(&self) -> &Self::SafeType {
        unsafe { &*(self as *const Self as *const Self::SafeType) }
    }

    fn as_safe_mut(&mut self) -> &mut Self::SafeType {
        unsafe { &mut *(self as *mut Self as *mut Self::SafeType) }
    }
}