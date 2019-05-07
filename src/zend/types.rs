use std::mem;
use std::os::raw::{c_uchar, c_char, c_void, c_int};
use crate::zend::zend_function;

/// Export renamed zend types
pub type Long = zend_long;
pub type Ulong = zend_ulong;
pub type Double = f64;
pub type OffT = zend_off_t;
pub type Bool = zend_bool;
pub type Uchar = zend_uchar;
pub type DtorFuncT = dtor_func_t;
pub type Type = zend_type;
pub type Value = zend_value;
pub type Zval = zval;
pub type ValueWw = zend_value_ww;
pub type RefcountedH = zend_refcounted_h;
pub type Refcounted = zend_refcounted;
pub type String = zend_string;
pub type Array = zend_array;
pub type Reference = zend_reference;

pub type Flag = u32;
pub type VaList = *mut c_char;

/// regular data types
pub const IS_UNDEF: zend_type = 0;
pub const IS_NULL: zend_type = 1;
pub const IS_FALSE: zend_type = 2;
pub const IS_TRUE: zend_type = 3;
pub const IS_LONG: zend_type = 4;
pub const IS_DOUBLE: zend_type = 5;
pub const IS_STRING: zend_type = 6;
pub const IS_ARRAY: zend_type = 7;
pub const IS_OBJECT: zend_type = 8;
pub const IS_RESOURCE: zend_type = 9;
pub const IS_REFERENCE: zend_type = 10;

/// constant expressions
pub const IS_CONSTANT: zend_type = 11;
pub const IS_CONSTANT_AST: zend_type = 12;

/// fake types
pub const _IS_BOOL: zend_type = 13;
pub const IS_CALLABLE: zend_type = 14;

/// internal types
pub const IS_INDIRECT: zend_type = 15;
pub const IS_PTR: zend_type = 17;

pub const Z_TYPE_MASK: Flag = 0xff;

pub const Z_TYPE_FLAGS_SHIFT: Flag = 8;
pub const Z_CONST_FLAGS_SHIFT: Flag = 16;

pub const GC_TYPE_MASK: Flag = 0x0000000f;
pub const GC_FLAGS_MASK: Flag = 0x000003f0;
pub const GC_INFO_MASK: Flag = 0xfffffc00;
pub const GC_FLAGS_SHIFT: Flag = 0;
pub const GC_INFO_SHIFT: Flag = 10;

/** zval_gc_flags(zval.value.gc.u.type_info) (common flags) */
pub const GC_COLLECTABLE: Flag = 1 << 4;
/// used for recursion detection
pub const GC_PROTECTED: Flag = 1 << 5;
/// can't be canged in place
pub const GC_IMMUTABLE: Flag = 1 << 6;
/// allocated using malloc
pub const GC_PERSISTENT: Flag = 1 << 7;
/// persistent, but thread-local
pub const GC_PERSISTENT_LOCAL: Flag = 1 << 8;

pub const GC_ARRAY: Flag = IS_ARRAY as Flag | (GC_COLLECTABLE << GC_FLAGS_SHIFT);
pub const GC_OBJECT: Flag = IS_OBJECT as Flag | (GC_COLLECTABLE << GC_FLAGS_SHIFT);

/** zval.u1.v.type_flags */
pub const IS_TYPE_CONSTANT: Flag = 1 << 0;
pub const IS_TYPE_IMMUTABLE: Flag = 1 << 1;
pub const IS_TYPE_REFCOUNTED: Flag = 1 << 2;
pub const IS_TYPE_COLLECTABLE: Flag = 1 << 3;
pub const IS_TYPE_COPYABLE: Flag = 1 << 4;
pub const IS_TYPE_SYMBOLTABLE: Flag = 1 << 5;

/** extended types */
pub const IS_INTERNED_STRING_EX: Flag = IS_STRING as Flag;

pub const IS_STRING_EX: Flag = IS_STRING as Flag | (( IS_TYPE_REFCOUNTED | IS_TYPE_COPYABLE) << Z_TYPE_FLAGS_SHIFT);
pub const IS_ARRAY_EX: Flag = 	IS_ARRAY as Flag | (( IS_TYPE_REFCOUNTED | IS_TYPE_COLLECTABLE | IS_TYPE_COPYABLE) << Z_TYPE_FLAGS_SHIFT);
pub const IS_OBJECT_EX: Flag = IS_OBJECT as Flag | (( IS_TYPE_REFCOUNTED | IS_TYPE_COLLECTABLE) << Z_TYPE_FLAGS_SHIFT);
pub const IS_RESOURCE_EX: Flag = IS_RESOURCE as Flag | (( IS_TYPE_REFCOUNTED ) << Z_TYPE_FLAGS_SHIFT);
pub const IS_REFERENCE_EX: Flag = IS_REFERENCE as Flag | (( IS_TYPE_REFCOUNTED ) << Z_TYPE_FLAGS_SHIFT);

pub const IS_CONSTANT_EX: Flag = IS_CONSTANT as Flag | ((IS_TYPE_CONSTANT | IS_TYPE_REFCOUNTED | IS_TYPE_COPYABLE) << Z_TYPE_FLAGS_SHIFT);
pub const IS_CONSTANT_AST_EX: Flag = IS_CONSTANT_AST as Flag | ((IS_TYPE_CONSTANT | IS_TYPE_REFCOUNTED | IS_TYPE_COPYABLE) << Z_TYPE_FLAGS_SHIFT);


pub type zend_long = isize;
pub type zend_ulong = usize;
pub type zend_off_t = isize;
pub type zend_bool = c_uchar;
pub type zend_uchar = c_uchar;
//pub type zend_object_handlers = _zend_object_handlers;
//pub type zend_class_entry = _zend_class_entry;
//pub type zend_function = _zend_function;
//pub type zend_execute_data = _zend_execute_data;
//pub type zend_object = _zend_object;
//pub type zend_resource = _zend_resource;
//pub type zend_ast_ref = _zend_ast_ref;
//pub type zend_ast = _zend_ast;
pub type dtor_func_t = Option<unsafe extern fn(pDest: *mut zval)>;
pub type zend_type = usize;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ZEND_RESULT_CODE(c_int);

impl ZEND_RESULT_CODE {
    /// this MUST stay a negative number, or it may affect functions!
    pub const FAILURE: ZEND_RESULT_CODE = ZEND_RESULT_CODE(-1);
    pub const SUCCESS: ZEND_RESULT_CODE = ZEND_RESULT_CODE(0);

    pub fn is_success(&self) -> bool {
        *self == Self::SUCCESS
    }

    pub fn is_failure(&self) -> bool {
        *self == Self::FAILURE
    }

    pub fn as_int(&self) -> c_int {
        self.0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union zend_value {
    /// long value
    pub lval: zend_long,
    /// double value
    pub dval: f64,
    pub counted: *mut zend_refcounted,
    pub str: *mut zend_string,
    pub arr: *mut zend_array,
//    pub obj: *mut zend_object,
//    pub res: *mut zend_resource,
    pub ref_: *mut zend_reference,
//    pub ast: *mut zend_ast_ref,
    pub zv: *mut zval,
    pub ptr: *mut c_void,
//    pub ce: *mut zend_class_entry,
    pub func: *mut zend_function,
    pub ww: zend_value_ww,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct zend_value_ww {
    pub w1: u32,
    pub w2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zval {
    pub value: zend_value,
    pub u1: zval_u1,
    pub u2: zval_u2,
}

impl Default for zval {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union zval_u1 {
    pub v: zval_v,
    pub type_info: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct zval_v {
    /// active type
    pub type_: zend_uchar,
    pub type_flags: zend_uchar,
    pub const_flags: zend_uchar,
    /// call info for EX(This)
    pub reserved: zend_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union zval_u2 {
    pub var_flags: u32,
    /// hash collision chain
    pub next: u32,
    /// literal cache slot
    pub cache_slot: u32,
    /// line number (for ast nodes)
    pub lineno: u32,
    /// arguments number for EX(This)
    pub num_args: u32,
    /// foreach position
    pub fe_pos: u32,
    /// foreach iterator index
    pub fe_iter_idx: u32,
}

impl zval {
    #[inline]
    pub fn set_undef(&mut self) {
        self.set_type_info(IS_UNDEF as u32);
    }

    #[inline]
    pub fn set_null(&mut self) {
        self.set_type_info(IS_NULL as u32);
    }

    #[inline]
    pub fn set_false(&mut self) {
        self.set_type_info(IS_FALSE as u32);
    }

    #[inline]
    pub fn set_true(&mut self) {
        self.set_type_info(IS_TRUE as u32);
    }

    #[inline]
    pub fn set_bool(&mut self, value: bool) {
        self.set_type_info(if value {IS_TRUE} else {IS_FALSE} as u32);
    }

    #[inline]
    pub fn set_long(&mut self, value: zend_long) {
        self.value.lval = value;
        self.set_type_info(IS_LONG as u32);
    }

    #[inline]
    pub fn set_double(&mut self, value: f64) {
        self.value.dval = value;
        self.set_type_info(IS_DOUBLE as u32);
    }

//    #[inline]
//    pub fn set_str(&mut self, value: &mut zend_string) {
//        self.value.str = value as *mut _;
//        self.set_type_info();
//    }

    #[inline]
    pub fn set_arr(&mut self, value: &mut zend_array) {
        self.value.arr = value as *mut _;
        self.set_type_info(IS_ARRAY_EX);
    }

    #[inline]
    pub fn set_new_str(&mut self, src: &str, persistent: bool) {
        self.value.str = zend_string::init(src, persistent);
        self.set_type_info(IS_STRING_EX);
    }

    #[inline]
    pub fn is_undef(&self) -> bool {
        self.get_type() == IS_UNDEF as zend_uchar
    }

    #[inline]
    pub fn is_indirect(&self) -> bool {
        self.get_type() == IS_INDIRECT as zend_uchar
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        let t = self.get_type() as zend_type;
        t == IS_TRUE || t == IS_FALSE
    }

    #[inline]
    pub fn is_long(&self) -> bool {
        self.get_type() == IS_LONG as zend_uchar
    }

    #[inline]
    pub fn is_double(&self) -> bool {
        self.get_type() == IS_DOUBLE as zend_uchar
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        self.get_type() == IS_STRING as zend_uchar
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        self.get_type() == IS_ARRAY as zend_uchar
    }

    #[inline]
    pub fn indirect(&self) -> *mut Self {
        unsafe { self.value.zv }
    }

    #[inline]
    pub fn get_type(&self) -> zend_uchar {
        unsafe { self.u1.v.type_ }
    }

    #[inline]
    pub fn get_type_flags(&self) -> zend_uchar {
        unsafe { self.u1.v.type_flags }
    }

    #[inline]
    pub fn get_const_flags(&self) -> zend_uchar {
        unsafe { self.u1.v.const_flags }
    }

    #[inline]
    pub fn get_type_info(&self) -> u32 {
        unsafe { self.u1.type_info }
    }

    #[inline]
    pub fn set_type_info(&mut self, type_info: u32) {
        self.u1.type_info = type_info;
    }

    #[inline]
    pub fn get_var_flags(&self) -> u32 {
        unsafe { self.u2.var_flags }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zend_refcounted_h {
    /// reference counter 32-bit
    pub refcount: u32,
    pub u: zend_refcounted_h_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union zend_refcounted_h_u {
    pub v: zend_refcounted_h_u_v,
    pub type_info: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct zend_refcounted_h_u_v {
    pub type_: zend_uchar,
    /// used for strings & objects
    pub flags: zend_uchar,
    /// keeps GC root number (or 0) and color
    pub gc_info: u16,
}

impl Default for zend_refcounted_h {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct zend_refcounted {
    pub gc: zend_refcounted_h,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zend_string {
    pub gc: zend_refcounted_h,
    pub h: zend_ulong,
    pub len: usize,
    pub val: [c_uchar; 1],
}

impl Default for zend_string {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

/** string flags (zval.value->gc.u.flags) */
/// allocated using malloc
pub const IS_STR_PERSISTENT: Flag = 1 << 0;
/// interned string
pub const IS_STR_INTERNED: Flag = 1 << 1;
/// relives request boundary
pub const IS_STR_PERMANENT: Flag = 1 << 2;
/// constant index
pub const IS_STR_CONSTANT: Flag = 1 << 3;
/// the same as IS_CONSTANT_UNQUALIFIED
pub const IS_STR_CONSTANT_UNQUALIFIED: Flag = 1 << 4;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bucket {
    pub val: zval,
    /// hash value (or numeric index)
    pub h: zend_ulong,
    /// string key or NULL for numerics
    pub key: *mut zend_string,
}

impl Default for Bucket {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Bucket {
    #[inline]
    pub unsafe fn get_val(&self, indirect: bool) -> Option<&zval> {
        let z = if indirect && self.val.get_type() == (IS_INDIRECT as zend_uchar) {
            &*self.val.indirect()
        } else {
            &self.val
        };
        if z.get_type() == (IS_UNDEF as zend_uchar) {
            return None;
        }
        Some(z)
    }

    #[inline]
    pub unsafe fn get_val_mut(&mut self, indirect: bool) -> Option<&mut zval> {
        let z = if indirect && self.val.get_type() == (IS_INDIRECT as zend_uchar) {
            &mut *self.val.indirect()
        } else {
            &mut self.val
        };
        if z.get_type() == (IS_UNDEF as zend_uchar) {
            return None;
        }
        Some(z)
    }
}

pub type HashTable = zend_array;
pub type HashPosition = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zend_array {
    pub gc: zend_refcounted_h,
    pub u: zend_array_u,
    pub nTableMask: u32,
    pub arData: *mut Bucket,
    pub nNumUsed: u32,
    pub nNumOfElements: u32,
    pub nTableSize: u32,
    pub nInternalPointer: u32,
    pub nNextFreeElement: zend_long,
    pub pDestructor: dtor_func_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union zend_array_u {
    pub v: zend_array_u_v,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct zend_array_u_v {
    pub flags: zend_uchar,
    pub nApplyCount: zend_uchar,
    pub nIteratorsCount: zend_uchar,
    pub consistency: zend_uchar,
}

impl Default for zend_array_u {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for zend_array {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct zend_reference {
    pub gc: zend_refcounted_h,
    pub val: zval,
}

impl Default for zend_reference {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zend_string_layout() {
        assert_eq!(
            mem::size_of::<zend_string>(), 32, concat!("Size of: ", stringify!(zend_string))
        );
        assert_eq!(
            mem::align_of::<zend_string>(), 8, concat!("Alignment of ", stringify!(zend_string))
        );
    }
}