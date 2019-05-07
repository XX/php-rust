use std::{
    ops::{Deref, DerefMut, Index},
    slice::from_raw_parts_mut, os::raw::c_char,
};
use libc::strlen;
use crate::{zend, ToSafe, Zval, ZString, Ulong};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bucket(zend::Bucket);

impl Bucket {
    #[inline]
    pub fn get_val(&self) -> Option<&Zval> {
        unsafe { self.0.get_val(true) }
            .map(|z| z.as_safe())
    }

    #[inline]
    pub fn get_val_mut(&mut self) -> Option<&mut Zval> {
        unsafe { self.0.get_val_mut(true) }
            .map(|z| z.as_safe_mut())
    }

    #[inline]
    pub fn get_hash(&self) -> Option<Ulong> {
        Some(self.0.h)
    }

    #[inline]
    pub fn get_key(&self) -> Option<ZString> {
        Some(self.0.key.into_safe())
    }
}

impl ToSafe for zend::Bucket {
    type SafeType = Bucket;
}

#[derive(Debug, Copy, Clone)]
pub enum ArrayIndex<'a> {
    /// zend_string pointer, mutable for ability to hash rewrite in php side
    ZString(*mut zend::String),
    /// null-terminated string, like C-string
    NtStr(&'a str),
    /// C-string with its length
    Cstr(*const c_char, usize),
}

impl From<ZString> for ArrayIndex<'_> {
    fn from(from: ZString) -> Self {
        ArrayIndex::ZString(from.into())
    }
}

impl From<*const c_char> for ArrayIndex<'_> {
    fn from(from: *const c_char) -> Self {
        let len = unsafe { strlen(from) };
        ArrayIndex::Cstr(from, len)
    }
}

impl<'a> From<&'a str> for ArrayIndex<'a> {
    fn from(from: &'a str) -> Self {
        assert_eq!(from.as_bytes()[from.len() - 1], b'\0', "ArrayIndex &str must be null terminated.");
        ArrayIndex::NtStr(from)
    }
}

#[derive(Default)]
pub struct ArrayBuilder {
    pub persistent: bool,
    /// wil be increased to 2^x (x >= 3): 8, 16, 32, 64, ...
    pub initial_min_size: u32,
    pub value_destructor: zend::DtorFuncT,
}

impl ArrayBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            value_destructor: Some(zend::ZVAL_PTR_DTOR),
            ..Default::default()
        }
    }

    #[inline]
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }

    #[inline]
    pub fn with_initial_min_size(mut self, size: u32) -> Self {
        self.initial_min_size = size;
        self
    }

    #[inline]
    pub fn with_value_destructor(mut self, destructor: zend::DtorFuncT) -> Self {
        self.value_destructor = destructor;
        self
    }

    #[inline]
    pub fn build(self) -> Array {
        let ArrayBuilder { initial_min_size, value_destructor, persistent } = self;
        Array::init(initial_min_size, value_destructor, persistent)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Array(*mut zend::Array);

pub trait ArrayApi {
    fn get<'a, I>(&self, index: I) -> Option<&Zval>
    where
        I: Into<ArrayIndex<'a>>;

    fn insert<'a, I, V>(&mut self, index: I, value: V) -> Option<&Zval>
    where
        I: Into<ArrayIndex<'a>>,
        V: Into<Zval>;

    fn exists<'a, I>(&self, index: I) -> bool
    where
        I: Into<ArrayIndex<'a>>;

    fn buckets_iter(&self) -> BucketsIter;

    fn buckets_iter_mut(&mut self) -> BucketsIterMut;
}

impl Array {
    #[inline]
    pub fn new() -> Self {
        let ArrayBuilder { initial_min_size, value_destructor, persistent } = ArrayBuilder::new();
        Self::init(initial_min_size, value_destructor, persistent)
    }

    #[inline]
    pub fn init(size: u32, destructor: zend::DtorFuncT, persistent: bool) -> Self {
        let ht = zend::Array::alloc();
        zend::Array::init(ht, size, (), destructor, persistent);
        Array(ht)
    }

    #[inline]
    pub fn raw(&self) -> *mut zend::Array {
        self.0
    }
}

impl ArrayApi for Array {
    #[inline]
    fn get<'a, I>(&self, index: I) -> Option<&Zval>
    where
        I: Into<ArrayIndex<'a>>
    {
        unsafe {
            let zv = match index.into() {
                ArrayIndex::ZString(zs) =>
                    zend::HashTable::find_ind(self.0, zs),
                ArrayIndex::NtStr(nts) =>
                    zend::HashTable::str_find_ind(self.0, nts.as_ptr() as *const _, nts.len() - 1),
                ArrayIndex::Cstr(cs, len) =>
                    zend::HashTable::str_find_ind(self.0, cs, len),
            };
            if zv.is_null() {
                None
            } else {
                Some(&*(zv as *const Zval))
            }
        }
    }

    #[inline]
    fn insert<'a, I, V>(&mut self, index: I, val: V) -> Option<&Zval>
    where
        I: Into<ArrayIndex<'a>>,
        V: Into<Zval>,
    {
        unsafe {
            let zv = match index.into() {
                ArrayIndex::ZString(zs) =>
                    zend::HashTable::update_ind(self.0, zs, val.into().as_raw_mut()),
                ArrayIndex::NtStr(nts) =>
                    zend::HashTable::str_update_ind(self.0, nts.as_ptr() as *const _, nts.len() - 1, val.into().as_raw_mut()),
                ArrayIndex::Cstr(cs, len) =>
                    zend::HashTable::str_update_ind(self.0, cs, len, val.into().as_raw_mut()),
            };
            if zv.is_null() {
                None
            } else {
                Some(&*(zv as *const Zval))
            }
        }
    }

    #[inline]
    fn exists<'a, I>(&self, index: I) -> bool
    where
        I: Into<ArrayIndex<'a>>
    {
        unsafe {
            match index.into() {
                ArrayIndex::ZString(zs) =>
                    zend::HashTable::exists_ind(self.0, zs),
                ArrayIndex::NtStr(nts) =>
                    zend::HashTable::str_exists_ind(self.0, nts.as_ptr() as *const _, nts.len() - 1),
                ArrayIndex::Cstr(cs, len) =>
                    zend::HashTable::str_exists_ind(self.0, cs, len),
            }
        }
    }

    #[inline]
    fn buckets_iter(&self) -> BucketsIter {
        unsafe { BucketsIter::from_raw(self.0) }
    }

    #[inline]
    fn buckets_iter_mut(&mut self) -> BucketsIterMut {
        unsafe { BucketsIterMut::from_raw(self.0) }
    }
}

// todo: maybe unsafe!
impl Deref for Array {
    type Target = zend::Array;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl Index<ArrayIndex<'_>> for Array {
    type Output = Zval;

    #[inline]
    fn index(&self, index: ArrayIndex<'_>) -> &Self::Output {
        self.get(index)
            .expect("Value is not exists in array for given index")
    }
}

impl ToSafe for *mut zend::Array {
    type SafeType = Array;

    #[inline]
    fn into_safe(self) -> Self::SafeType {
        Array(self)
    }
}

pub struct BucketsIter<'a> {
    buckets: &'a [zend::Bucket],
    idx: usize,
}

impl BucketsIter<'_> {
    #[inline]
    pub unsafe fn from_raw(raw: *const zend::Array) -> Self {
        BucketsIter {
            buckets: (*raw).buckets(),
            idx: 0,
        }
    }
}

impl<'a> Iterator for BucketsIter<'a> {
    type Item = &'a Bucket;

    #[inline]
    fn next(&mut self) -> Option<&'a Bucket> {
        unsafe {
            while self.idx < self.buckets.len()
                && self.buckets[self.idx].get_val(true).is_none()
            {
                self.idx += 1;
            }
        }
        if self.idx < self.buckets.len() {
            self.idx += 1;
            Some(self.buckets[self.idx - 1].as_safe())
        } else {
            None
        }
    }
}

pub struct BucketsIterMut<'a> {
    rest_buckets: &'a mut [zend::Bucket],
}

impl BucketsIterMut<'_> {
    #[inline]
    pub unsafe fn from_raw(raw: *mut zend::Array) -> Self {
        BucketsIterMut {
            rest_buckets: (*raw).buckets_mut(),
        }
    }
}

impl<'a> Iterator for BucketsIterMut<'a> {
    type Item = &'a mut Bucket;

    #[inline]
    fn next(&mut self) -> Option<&'a mut Bucket> {
        let mut idx = 0;
        let len = self.rest_buckets.len();
        unsafe {
            while idx < len
                && self.rest_buckets[idx].get_val(true).is_none()
            {
                idx += 1;
            }
            if idx < len {
                let item = &mut self.rest_buckets[idx] as *mut zend::Bucket;
                let rest = from_raw_parts_mut(
                    self.rest_buckets.as_mut_ptr().add(idx + 1),
                    len - idx - 1
                );
                self.rest_buckets = rest;
                Some((*item).as_safe_mut())
            } else {
                None
            }
        }
    }
}