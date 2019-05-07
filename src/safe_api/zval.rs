use std::{
    ops::{Deref, DerefMut, Index},
    cmp::PartialEq,
};
use crate::{
    zend, Long, Double, ZString, Array, ArrayApi, ArrayIndex, BucketsIter, BucketsIterMut,
    VoidPtrMut, Refcounted, ToSafe
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Undefined,
    Null,
    Bool(bool),
    Long(Long),
    Double(f64),
    Counted(*mut Refcounted),
    Reference(*mut zend::Reference),
    String(ZString),
    Array(Array),
    Zval(*mut Zval),
    Ptr(VoidPtrMut),
    Function(*mut zend::Function),
    Unknown,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Zval(zend::Zval);

impl Zval {
    #[inline]
    pub fn value(&self) -> Value {
        unsafe {
            match self.0.get_type() as zend::Type {
                zend::IS_UNDEF => Value::Undefined,
                zend::IS_NULL => Value::Null,
                zend::IS_FALSE => Value::Bool(false),
                zend::IS_TRUE => Value::Bool(true),
                zend::IS_LONG => Value::Long(self.0.value.lval),
                zend::IS_DOUBLE => Value::Double(self.0.value.dval),
                zend::IS_STRING => Value::String(self.0.value.str.into_safe()),
                zend::IS_ARRAY => Value::Array(self.0.value.arr.into_safe()),
//            zend::IS_OBJECT => Value::Object(self.0.value.obj),
//            zend::IS_RESOURCE,
                zend::IS_REFERENCE => Value::Reference(self.0.value.ref_),
                _ => Value::Unknown,
            }
        }
    }

    #[inline]
    pub fn as_array(&self) -> Option<Array> {
        unsafe {
            if self.0.get_type() as zend::Type == zend::IS_ARRAY {
                Some(self.0.value.arr.into_safe())
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn raw(self) -> zend::Zval {
        self.0
    }

    #[inline]
    pub fn as_raw(&self) -> *const zend::Zval {
        &self.0 as *const _
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> *mut zend::Zval {
        &mut self.0 as *mut _
    }
}

impl ArrayApi for Zval {
    #[inline]
    fn get<'a, I>(&self, index: I) -> Option<&Zval>
    where
        I: Into<ArrayIndex<'a>>
    {
        self.as_array()
            .expect("Can't convert Zval to Array because it is not array")
            .get(index)
            .map(|zv| unsafe { &*(zv as *const _) })
    }

    #[inline]
    fn insert<'a, I, V>(&mut self, index: I, val: V) -> Option<&Zval>
    where
        I: Into<ArrayIndex<'a>>,
        V: Into<Zval>,
    {
        self.as_array()
            .expect("Can't convert Zval to Array because it is not array")
            .insert(index, val)
            .map(|zv| unsafe { &*(zv as *const _) })
    }

    #[inline]
    fn exists<'a, I>(&self, index: I) -> bool
    where
        I: Into<ArrayIndex<'a>>
    {
        self.as_array()
            .expect("Can't convert Zval to Array because it is not array")
            .exists(index)
    }

    #[inline]
    fn buckets_iter(&self) -> BucketsIter {
        let raw_array = self.as_array()
            .expect("Can't convert Zval to Array because it is not array")
            .raw();
        unsafe { BucketsIter::from_raw(raw_array) }
    }

    #[inline]
    fn buckets_iter_mut(&mut self) -> BucketsIterMut {
        let raw_array = self.as_array()
            .expect("Can't convert Zval to Array because it is not array")
            .raw();
        unsafe { BucketsIterMut::from_raw(raw_array) }
    }
}

impl Index<ArrayIndex<'_>> for Zval {
    type Output = Zval;

    #[inline]
    fn index(&self, index: ArrayIndex<'_>) -> &Self::Output {
        <Self as ArrayApi>::get(self, index)
            .expect("Value is not exists in array for given index")
    }
}

impl Deref for Zval {
    type Target = zend::Zval;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Zval {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Zval {
    fn eq(&self, other: &Zval) -> bool {
        self.value() == other.value()
    }
}

impl From<Long> for Zval {
    fn from(from: Long) -> Self {
        let mut zv = zend::Zval::default();
        zv.set_long(from);
        Zval(zv)
    }
}

impl From<Double> for Zval {
    fn from(from: Double) -> Self {
        let mut zv = zend::Zval::default();
        zv.set_double(from);
        Zval(zv)
    }
}

impl From<&'_ str> for Zval {
    fn from(from: &'_ str) -> Self {
        let mut zv = zend::Zval::default();
        zv.set_new_str(from, false);
        Zval(zv)
    }
}

impl ToSafe for zend::Zval {
    type SafeType = Zval;
}