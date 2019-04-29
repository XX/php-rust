use std::mem;
use std::os::raw::c_void;
use crate::zend::{zend_type, HashTable};

extern "C" {
    fn _emalloc(size: usize) -> *mut c_void;
    fn _safe_emalloc(nmemb: usize, size: usize, offset: usize) -> *mut c_void;
    fn _safe_malloc(nmemb: usize, size: usize, offset: usize) -> *mut c_void;
    fn _efree(ptr: *mut c_void);
    fn __zend_malloc(len: usize) -> *mut c_void;
}

pub const ZEND_MM_ALIGNMENT: zend_type = 8;
pub const ZEND_MM_ALIGNMENT_LOG2: zend_type = 3;
pub const ZEND_MM_ALIGNMENT_MASK: zend_type = !(ZEND_MM_ALIGNMENT - 1);
pub const fn ZEND_MM_ALIGNED_SIZE(size: usize) -> zend_type {
    (size + ZEND_MM_ALIGNMENT - 1) & ZEND_MM_ALIGNMENT_MASK
}

#[inline]
pub fn emalloc<T>(size: usize) -> *mut T {
    unsafe { _emalloc(size) as *mut _ }
}

#[inline]
pub fn safe_emalloc<T>(nmemb: usize, size: usize, offset: usize) -> *mut T {
    unsafe { _safe_emalloc(nmemb, size, offset) as *mut _ }
}

#[inline]
pub fn efree<T>(ptr: *mut T) {
    unsafe { _efree(ptr as *mut _) }
}


/** Selective persistent/non persistent allocation */

#[inline]
pub fn pemalloc<T>(size: usize, persistent: bool) -> *mut T {
    if persistent {
        unsafe { __zend_malloc(size) as *mut _ }
    } else {
        emalloc(size)
    }
}

#[inline]
pub fn safe_pemalloc<T>(nmemb: usize, size: usize, offset: usize, persistent: bool) -> *mut T {
    if persistent {
        unsafe { _safe_malloc(nmemb, size, offset) as *mut _ }
    } else {
        safe_emalloc(nmemb, size, offset)
    }
}

#[inline]
pub fn pefree<T>(ptr: *mut T, persistent: bool) {
    if persistent {
        unsafe { libc::free(ptr as *mut _) }
    } else {
        efree(ptr)
    }
}

#[inline]
pub fn pefree_size<T>(ptr: *mut T, size: usize, persistent: bool) {
    if persistent {
        unsafe { libc::free(ptr as *mut _) }
    } else {
        efree_size(ptr, size)
    }
}

#[inline]
pub fn efree_size<T>(ptr: *mut T, _size: usize) {
    efree(ptr as *mut _)
}

/// fast cache for HashTables
impl HashTable {
    /// ALLOC_HASHTABLE
    #[inline]
    pub fn alloc() -> *mut Self {
        emalloc(mem::size_of::<Self>())
    }

    /// FREE_HASHTABLE
    #[inline]
    pub fn free(ht: *mut Self) {
        efree_size(ht, mem::size_of::<Self>())
    }
/*
#define ALLOC_HASHTABLE_REL(ht)	\
(ht) = (HashTable *) emalloc_rel(sizeof(HashTable))

#define FREE_HASHTABLE_REL(ht)	\
efree_size_rel(ht, sizeof(HashTable))
*/
}