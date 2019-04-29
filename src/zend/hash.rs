use std::slice;
use crate::zend::{HashTable, Bucket, dtor_func_t, zend_bool};

extern {
    /** startup/shutdown */

    fn _zend_hash_init(ht: *mut HashTable, nSize: u32, pDestructor: dtor_func_t, persistent: zend_bool);
    fn _zend_hash_init_ex(ht: *mut HashTable, nSize: u32, pDestructor: dtor_func_t, persistent: zend_bool, bApplyProtection: zend_bool);
    fn zend_hash_destroy(ht: *mut HashTable);
    fn zend_hash_clean(ht: *mut HashTable);
}

impl HashTable {
    #[inline]
    pub fn init(ht: *mut Self, n_size: u32, _hash_function: (), destructor: dtor_func_t, persistent: bool) {
        unsafe { _zend_hash_init(ht, n_size, destructor, if persistent { 1 } else { 0 }) };
    }

    #[inline]
    pub fn init_ex(ht: *mut Self, n_size: u32, _hash_function: (), destructor: dtor_func_t, persistent: bool, apply_protection: bool) {
        unsafe { _zend_hash_init_ex(ht, n_size, destructor, if persistent { 1 } else { 0 }, if apply_protection { 1 } else { 0 }) };
    }

    #[inline]
    pub fn destroy(ht: *mut Self) {
        unsafe { zend_hash_destroy(ht) };
    }

    #[inline]
    pub fn clean(ht: *mut Self) {
        unsafe { zend_hash_clean(ht) };
    }

    #[inline]
    pub unsafe fn buckets(&self) -> &[Bucket] {
        slice::from_raw_parts(self.arData, self.nNumUsed as usize)
    }

    #[inline]
    pub unsafe fn buckets_mut(&mut self) -> &mut [Bucket] {
        slice::from_raw_parts_mut(self.arData, self.nNumUsed as usize)
    }

//    #[inline]
//    pub unsafe fn foreach(&mut self, indirect: bool) {
//        for b in self.buckets_mut() {
//            let z = b.get_val_mut(indirect);
////            f(b, z)
//        }
//    }
}