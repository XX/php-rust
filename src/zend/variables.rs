use crate::zend::zval;

extern {
    fn zend_print_variable(var: *const zval) -> usize;
    fn _zval_ptr_dtor(zval_ptr: *mut zval);
    fn _zval_internal_dtor_for_ptr(zvalue: *mut zval);
    fn _zval_internal_dtor(zvalue: *mut zval);
    fn _zval_internal_ptr_dtor(zvalue: *mut zval);
    fn _zval_dtor_wrapper(zvalue: *mut zval);
}

#[inline]
pub fn print_variable(var: *const zval) -> usize {
    unsafe { zend_print_variable(var) }
}

pub const ZVAL_PTR_DTOR: unsafe extern fn(*mut zval) = _zval_ptr_dtor;

impl zval {
    #[inline]
    pub fn ptr_dtor(zval_ptr: *mut Self) {
        unsafe { _zval_ptr_dtor(zval_ptr) };
    }

    #[inline]
    pub fn internal_dtor(zvalue: *mut Self) {
        unsafe { _zval_internal_dtor(zvalue) };
    }

    #[inline]
    pub fn internal_ptr_dtor(zvalue: *mut Self) {
        unsafe { _zval_internal_ptr_dtor(zvalue) };
    }

    #[inline]
    pub fn dtor_wrapper(zvalue: *mut Self) {
        unsafe { _zval_dtor_wrapper(zvalue) };
    }
}