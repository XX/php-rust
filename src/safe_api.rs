pub mod execute_data;
pub mod string;

use std::os::raw::c_void;
use crate::zend;

pub use execute_data::*;
pub use string::*;

pub use zend::Long;
pub use zend::Ulong;
pub use zend::OffT;
pub use zend::Bool;
pub use zend::Uchar;
pub use zend::Type;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Value {
    /// long value
    Lval(Long),
    /// double value
    Dval(f64),
    Counted(*mut zend::Refcounted),
    Str(*mut zend::String),
    Zv(*mut zend::Zval),
    Ptr(*mut c_void),
    Ww(zend::zend_value_ww),
}