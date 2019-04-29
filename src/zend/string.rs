use std::{mem, ptr};
use crate::zend::{zend_string, Flag, pemalloc, ZEND_MM_ALIGNED_SIZE, IS_STRING, IS_STR_PERSISTENT};

impl zend_string {
    #[inline]
    pub fn alloc(len: usize, persistent: bool) -> *mut Self {
        let ret = pemalloc::<Self>(
            ZEND_MM_ALIGNED_SIZE(mem::size_of::<Self>() + len),
            persistent
        );

        unsafe {
            (*ret).gc.refcount = 1;
            (*ret).gc.u.type_info = IS_STRING as Flag
                | (if persistent {IS_STR_PERSISTENT} else {0} << 8);
            (*ret).h = 0;
            (*ret).len = len;
        };
        ret
    }

    #[inline]
    pub fn init(src: &str, persistent: bool) -> *mut Self {
        let len = src.len();
        let ret = Self::alloc(len, persistent);

        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), (*ret).val.as_mut_ptr(), len);
            *(*ret).val.get_unchecked_mut(len) = b'\0';
        }
        ret
    }
}