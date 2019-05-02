use crate::{zend, Uchar, ToSafe};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Refcounted(zend::RefcountedH);

impl Refcounted {
    #[inline]
    pub fn refcount(&self) -> u32 {
        self.0.refcount
    }

    #[inline]
    pub fn type_info(&self) -> u32 {
        unsafe { self.0.u.type_info }
    }

    #[inline]
    pub fn type_(&self) -> Uchar {
        unsafe { self.0.u.v.type_ }
    }

    #[inline]
    pub fn flags(&self) -> Uchar {
        unsafe { self.0.u.v.flags }
    }

    #[inline]
    pub fn gc_info(&self) -> u16 {
        unsafe { self.0.u.v.gc_info }
    }
}

impl From<zend::RefcountedH> for Refcounted {
    fn from(from: zend::RefcountedH) -> Self {
        Self(from)
    }
}

impl ToSafe for zend::RefcountedH {
    type SafeType = Refcounted;
}

impl ToSafe for zend::Refcounted {
    type SafeType = Refcounted;
}