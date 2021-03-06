use std::{slice, ptr};
use std::os::raw::{c_char, c_int, c_void};
use crate::zend::{
    HashTable, HashPosition, Bucket, dtor_func_t,zend_bool, zend_ulong, zend_string, zval, zend_type,
    ZEND_RESULT_CODE, VaList,
};

pub const HASH_KEY_IS_STRING: c_int = 1;
pub const HASH_KEY_IS_LONG: c_int = 2;
pub const HASH_KEY_NON_EXISTENT: c_int = 3;

extern {
    /** startup/shutdown */
    fn _zend_hash_init(ht: *mut HashTable, nSize: u32, pDestructor: dtor_func_t, persistent: zend_bool);
    fn _zend_hash_init_ex(ht: *mut HashTable, nSize: u32, pDestructor: dtor_func_t, persistent: zend_bool, bApplyProtection: zend_bool);
    fn zend_hash_destroy(ht: *mut HashTable);
    fn zend_hash_clean(ht: *mut HashTable);

    /** additions/updates/changes */
    fn _zend_hash_add_or_update(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval, flag: u32) -> *mut zval;
    fn _zend_hash_update(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval) -> *mut zval;
    fn _zend_hash_update_ind(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval) -> *mut zval;
    fn _zend_hash_add(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval) -> *mut zval;
    fn _zend_hash_add_new(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval) -> *mut zval;

//#define zend_hash_update(ht, key, pData) \
//_zend_hash_update(ht, key, pData ZEND_FILE_LINE_CC)
//#define zend_hash_update_ind(ht, key, pData) \
//_zend_hash_update_ind(ht, key, pData ZEND_FILE_LINE_CC)
//#define zend_hash_add(ht, key, pData) \
//_zend_hash_add(ht, key, pData ZEND_FILE_LINE_CC)
//#define zend_hash_add_new(ht, key, pData) \
//_zend_hash_add_new(ht, key, pData ZEND_FILE_LINE_CC)

    fn _zend_hash_str_add_or_update(ht: *mut HashTable, key: *const c_char, len: usize, pData: *mut zval, flag: u32) -> *mut zval;
    fn _zend_hash_str_update(ht: *mut HashTable, key: *const c_char, len: usize, pData: *mut zval) -> *mut zval;
    fn _zend_hash_str_update_ind(ht: *mut HashTable, key: *const c_char, len: usize, pData: *mut zval) -> *mut zval;
    fn _zend_hash_str_add(ht: *mut HashTable, key: *const c_char, len: usize, pData: *mut zval) -> *mut zval;
    fn _zend_hash_str_add_new(ht: *mut HashTable, key: *const c_char, len: usize, pData: *mut zval) -> *mut zval;

//#define zend_hash_str_update(ht, key, len, pData) \
//_zend_hash_str_update(ht, key, len, pData ZEND_FILE_LINE_CC)
//#define zend_hash_str_update_ind(ht, key, len, pData) \
//_zend_hash_str_update_ind(ht, key, len, pData ZEND_FILE_LINE_CC)
//#define zend_hash_str_add(ht, key, len, pData) \
//_zend_hash_str_add(ht, key, len, pData ZEND_FILE_LINE_CC)
//#define zend_hash_str_add_new(ht, key, len, pData) \
//_zend_hash_str_add_new(ht, key, len, pData ZEND_FILE_LINE_CC)

    fn _zend_hash_index_add_or_update(ht: *mut HashTable, h: zend_ulong, pData: *mut zval, flag: u32) -> *mut zval;
    fn _zend_hash_index_add(ht: *mut HashTable, h: zend_ulong, pData: *mut zval) -> *mut zval;
    fn _zend_hash_index_add_new(ht: *mut HashTable, h: zend_ulong, pData: *mut zval) -> *mut zval;
    fn _zend_hash_index_update(ht: *mut HashTable, h: zend_ulong, pData: *mut zval) -> *mut zval;
    fn _zend_hash_next_index_insert(ht: *mut HashTable, pData: *mut zval) -> *mut zval;
    fn _zend_hash_next_index_insert_new(ht: *mut HashTable, pData: *mut zval) -> *mut zval;

//#define zend_hash_index_add(ht, h, pData) \
//_zend_hash_index_add(ht, h, pData ZEND_FILE_LINE_CC)
//#define zend_hash_index_add_new(ht, h, pData) \
//_zend_hash_index_add_new(ht, h, pData ZEND_FILE_LINE_CC)
//#define zend_hash_index_update(ht, h, pData) \
//_zend_hash_index_update(ht, h, pData ZEND_FILE_LINE_CC)
//#define zend_hash_next_index_insert(ht, pData) \
//_zend_hash_next_index_insert(ht, pData ZEND_FILE_LINE_CC)
//#define zend_hash_next_index_insert_new(ht, pData) \
//_zend_hash_next_index_insert_new(ht, pData ZEND_FILE_LINE_CC)

    fn zend_hash_index_add_empty_element(ht: *mut HashTable, h: zend_ulong) -> *mut zval;
    fn zend_hash_add_empty_element(ht: *mut HashTable, key: *mut zend_string) -> *mut zval;
    fn zend_hash_str_add_empty_element(ht: *mut HashTable, key: *const c_char, len: usize) -> *mut zval;

    fn zend_hash_graceful_destroy(ht: *mut HashTable);
    fn zend_hash_graceful_reverse_destroy(ht: *mut HashTable);
    fn zend_hash_apply(ht: *mut HashTable, apply_func: apply_func_t);
    fn zend_hash_apply_with_argument(ht: *mut HashTable, apply_func: apply_func_arg_t , _: *mut c_void);
    fn zend_hash_apply_with_arguments(ht: *mut HashTable, apply_func: apply_func_args_t, _: c_int, ...);

    /** Deletes */
    fn zend_hash_del(ht: *mut HashTable, key: *mut zend_string) -> ZEND_RESULT_CODE;
    fn zend_hash_del_ind(ht: *mut HashTable, key: *mut zend_string) -> ZEND_RESULT_CODE;
    fn zend_hash_str_del(ht: *mut HashTable, key: *const c_char, len: usize) -> ZEND_RESULT_CODE;
    fn zend_hash_str_del_ind(ht: *mut HashTable, key: *const c_char, len: usize) -> ZEND_RESULT_CODE;
    fn zend_hash_index_del(ht: *mut HashTable, h: zend_ulong) -> ZEND_RESULT_CODE;
    fn zend_hash_del_bucket(ht: *mut HashTable, p: *mut Bucket);

    /** Data retreival */
    fn zend_hash_find(ht: *const HashTable, key: *mut zend_string) -> *mut zval;
    fn zend_hash_str_find(ht: *const HashTable, key: *const c_char, len: usize) -> *mut zval;
    fn zend_hash_index_find(ht: *const HashTable, h: zend_ulong) -> *mut zval;

    /** Misc */
    fn zend_hash_exists(ht: *const HashTable, key: *mut zend_string) -> zend_bool;
    fn zend_hash_str_exists(ht: *const HashTable, str: *const c_char, len: usize) -> zend_bool;
    fn zend_hash_index_exists(ht: *const HashTable, h: zend_ulong) -> zend_bool;

    /** traversing */
    fn zend_hash_move_forward_ex(ht: *mut HashTable, pos: *mut HashPosition) -> ZEND_RESULT_CODE;
    fn zend_hash_move_backwards_ex(ht: *mut HashTable, pos: *mut HashPosition) -> ZEND_RESULT_CODE;
    fn zend_hash_get_current_key_ex(ht: *const HashTable, str_index: *mut *mut zend_string, num_index: *mut zend_ulong, pos: *mut HashPosition) -> ZEND_RESULT_CODE;
    fn zend_hash_get_current_key_zval_ex(ht: *const HashTable, key: *mut zval, pos: *mut HashPosition);
    fn zend_hash_get_current_key_type_ex(ht: *mut HashTable, pos: *mut HashPosition) -> c_int;
    fn zend_hash_get_current_data_ex(ht: *mut HashTable, pos: *mut HashPosition) -> *mut zval;
    fn zend_hash_internal_pointer_reset_ex(ht: *mut HashTable, pos: *mut HashPosition);
    fn zend_hash_internal_pointer_end_ex(ht: *mut HashTable, pos: *mut HashPosition);
}

pub const ZEND_HASH_APPLY_KEEP: zend_type = 0;
pub const ZEND_HASH_APPLY_REMOVE: zend_type = 1 << 0;
pub const ZEND_HASH_APPLY_STOP: zend_type = 1 << 1;

pub type apply_func_t = extern fn(pDest: *mut zval) -> c_int;
pub type apply_func_arg_t = extern fn(pDest: *mut zval, argument: *mut c_void) -> c_int;
pub type apply_func_args_t = extern fn(pDest: *mut zval, num_args: c_int, args: VaList, hash_key: *mut zend_hash_key) -> c_int;

#[repr(C)]
pub struct zend_hash_key {
    h: zend_ulong,
    key: *mut zend_string,
}

impl HashTable {
    #[inline]
    pub fn init(ht: *mut HashTable, n_size: u32, _hash_function: (), destructor: dtor_func_t, persistent: bool) {
        unsafe { _zend_hash_init(ht, n_size, destructor, if persistent { 1 } else { 0 }) };
    }

    #[inline]
    pub fn init_ex(ht: *mut HashTable, n_size: u32, _hash_function: (), destructor: dtor_func_t, persistent: bool, apply_protection: bool) {
        unsafe { _zend_hash_init_ex(ht, n_size, destructor, if persistent { 1 } else { 0 }, if apply_protection { 1 } else { 0 }) };
    }

    #[inline]
    pub fn destroy(ht: *mut HashTable) {
        unsafe { zend_hash_destroy(ht) };
    }

    #[inline]
    pub fn clean(ht: *mut HashTable) {
        unsafe { zend_hash_clean(ht) };
    }

    #[inline]
    pub unsafe fn has_more_elements_ex(ht: *mut HashTable, pos: *mut HashPosition) -> ZEND_RESULT_CODE {
        if zend_hash_get_current_key_type_ex(ht, pos) == HASH_KEY_NON_EXISTENT {
            ZEND_RESULT_CODE::FAILURE
        } else {
            ZEND_RESULT_CODE::SUCCESS
        }
    }

    #[inline]
    pub unsafe fn find_ind(ht: *const HashTable, key: *mut zend_string) -> *mut zval {
        let zv = zend_hash_find(ht, key);

        if !zv.is_null() && (*zv).is_indirect() {
            let indir = (*zv).indirect();
            if !(*indir).is_undef() {
                indir
            } else {
                ptr::null_mut()
            }
        } else {
            zv
        }
    }

    #[inline]
    pub unsafe fn exists_ind(ht: *const HashTable, key: *mut zend_string) -> bool {
        let zv = zend_hash_find(ht, key);

        !zv.is_null() && (!(*zv).is_indirect() || !(*(*zv).indirect()).is_undef())
    }

    #[inline]
    pub unsafe fn str_find_ind(ht: *const HashTable, key: *const c_char, len: usize) -> *mut zval {
        let zv = zend_hash_str_find(ht, key, len);

        if !zv.is_null() && (*zv).is_indirect() {
            let indir = (*zv).indirect();
            if !(*indir).is_undef() {
                indir
            } else {
                ptr::null_mut()
            }
        } else {
            zv
        }
    }

    #[inline]
    pub unsafe fn str_exists_ind(ht: *const HashTable, key: *const c_char, len: usize) -> bool {
        let zv = zend_hash_str_find(ht, key, len);

        !zv.is_null() && (!(*zv).is_indirect() || !(*(*zv).indirect()).is_undef())
    }

    #[inline]
    pub unsafe fn update_ind(ht: *mut HashTable, key: *mut zend_string, val: *mut zval) -> *mut zval {
        _zend_hash_update_ind(ht, key, val)
    }

    #[inline]
    pub unsafe fn str_update_ind(ht: *mut HashTable, key: *const c_char, len: usize, val: *mut zval) -> *mut zval {
        _zend_hash_str_update_ind(ht, key, len, val)
    }
/*

static zend_always_inline zval *zend_symbtable_add_new(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_add_new(ht, idx, pData);
	} else {
		return zend_hash_add_new(ht, key, pData);
	}
}

static zend_always_inline zval *zend_symtable_update(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_update(ht, idx, pData);
	} else {
		return zend_hash_update(ht, key, pData);
	}
}


static zend_always_inline zval *zend_symtable_update_ind(ht: *mut HashTable, key: *mut zend_string, pData: *mut zval)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_update(ht, idx, pData);
	} else {
		return zend_hash_update_ind(ht, key, pData);
	}
}


static zend_always_inline int zend_symtable_del(ht: *mut HashTable, key: *mut zend_string)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_del(ht, idx);
	} else {
		return zend_hash_del(ht, key);
	}
}


static zend_always_inline int zend_symtable_del_ind(ht: *mut HashTable, key: *mut zend_string)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_del(ht, idx);
	} else {
		return zend_hash_del_ind(ht, key);
	}
}


static zend_always_inline zval *zend_symtable_find(ht: *const HashTable, key: *mut zend_string)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_find(ht, idx);
	} else {
		return zend_hash_find(ht, key);
	}
}


static zend_always_inline zval *zend_symtable_find_ind(ht: *const HashTable, key: *mut zend_string)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_find(ht, idx);
	} else {
		return zend_hash_find_ind(ht, key);
	}
}


static zend_always_inline int zend_symtable_exists(ht: *mut HashTable, key: *mut zend_string)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_exists(ht, idx);
	} else {
		return zend_hash_exists(ht, key);
	}
}


static zend_always_inline int zend_symtable_exists_ind(ht: *mut HashTable, key: *mut zend_string)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC(key, idx)) {
		return zend_hash_index_exists(ht, idx);
	} else {
		return zend_hash_exists_ind(ht, key);
	}
}


static zend_always_inline zval *zend_symtable_str_update(ht: *mut HashTable, str: *const c_char, len: usize, pData: *mut zval)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC_STR(str, len, idx)) {
		return zend_hash_index_update(ht, idx, pData);
	} else {
		return zend_hash_str_update(ht, str, len, pData);
	}
}


static zend_always_inline zval *zend_symtable_str_update_ind(ht: *mut HashTable, str: *const c_char, len: usize, pData: *mut zval)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC_STR(str, len, idx)) {
		return zend_hash_index_update(ht, idx, pData);
	} else {
		return zend_hash_str_update_ind(ht, str, len, pData);
	}
}


static zend_always_inline int zend_symtable_str_del(ht: *mut HashTable, str: *const c_char, len: usize)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC_STR(str, len, idx)) {
		return zend_hash_index_del(ht, idx);
	} else {
		return zend_hash_str_del(ht, str, len);
	}
}


static zend_always_inline int zend_symtable_str_del_ind(ht: *mut HashTable, str: *const c_char, len: usize)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC_STR(str, len, idx)) {
		return zend_hash_index_del(ht, idx);
	} else {
		return zend_hash_str_del_ind(ht, str, len);
	}
}


static zend_always_inline zval *zend_symtable_str_find(ht: *mut HashTable, str: *const c_char, len: usize)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC_STR(str, len, idx)) {
		return zend_hash_index_find(ht, idx);
	} else {
		return zend_hash_str_find(ht, str, len);
	}
}


static zend_always_inline int zend_symtable_str_exists(ht: *mut HashTable, str: *const c_char, len: usize)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC_STR(str, len, idx)) {
		return zend_hash_index_exists(ht, idx);
	} else {
		return zend_hash_str_exists(ht, str, len);
	}
}

static zend_always_inline void *zend_hash_add_ptr(ht: *mut HashTable, key: *mut zend_string, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_add(ht, key, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_add_new_ptr(ht: *mut HashTable, key: *mut zend_string, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_add_new(ht, key, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_str_add_ptr(ht: *mut HashTable, str: *const c_char, len: usize, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_str_add(ht, str, len, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_str_add_new_ptr(ht: *mut HashTable, str: *const c_char, len: usize, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_str_add_new(ht, str, len, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_update_ptr(ht: *mut HashTable, key: *mut zend_string, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_update(ht, key, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_str_update_ptr(ht: *mut HashTable, str: *const c_char, len: usize, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_str_update(ht, str, len, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_add_mem(ht: *mut HashTable, key: *mut zend_string, pData: *mut c_void, size_t size)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, NULL);
	if ((zv = zend_hash_add(ht, key, &tmp))) {
		Z_PTR_P(zv) = pemalloc(size, ht->u.flags & HASH_FLAG_PERSISTENT);
		memcpy(Z_PTR_P(zv), pData, size);
		return Z_PTR_P(zv);
	}
	return NULL;
}

static zend_always_inline void *zend_hash_str_add_mem(ht: *mut HashTable, str: *const c_char, len: usize, pData: *mut c_void, size_t size)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, NULL);
	if ((zv = zend_hash_str_add(ht, str, len, &tmp))) {
		Z_PTR_P(zv) = pemalloc(size, ht->u.flags & HASH_FLAG_PERSISTENT);
		memcpy(Z_PTR_P(zv), pData, size);
		return Z_PTR_P(zv);
	}
	return NULL;
}

static zend_always_inline void *zend_hash_update_mem(ht: *mut HashTable, key: *mut zend_string, pData: *mut c_void, size_t size)
{
	void *p;

	p = pemalloc(size, ht->u.flags & HASH_FLAG_PERSISTENT);
	memcpy(p, pData, size);
	return zend_hash_update_ptr(ht, key, p);
}

static zend_always_inline void *zend_hash_str_update_mem(ht: *mut HashTable, str: *const c_char, len: usize, pData: *mut c_void, size_t size)
{
	void *p;

	p = pemalloc(size, ht->u.flags & HASH_FLAG_PERSISTENT);
	memcpy(p, pData, size);
	return zend_hash_str_update_ptr(ht, str, len, p);
}

static zend_always_inline void *zend_hash_index_add_ptr(ht: *mut HashTable, h: zend_ulong, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_index_add(ht, h, &tmp);
	return zv ? Z_PTR_P(zv) : NULL;
}

static zend_always_inline void *zend_hash_index_add_new_ptr(ht: *mut HashTable, h: zend_ulong, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_index_add_new(ht, h, &tmp);
	return zv ? Z_PTR_P(zv) : NULL;
}

static zend_always_inline void *zend_hash_index_update_ptr(ht: *mut HashTable, h: zend_ulong, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_index_update(ht, h, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_index_add_mem(ht: *mut HashTable, h: zend_ulong, pData: *mut c_void, size_t size)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, NULL);
	if ((zv = zend_hash_index_add(ht, h, &tmp))) {
		Z_PTR_P(zv) = pemalloc(size, ht->u.flags & HASH_FLAG_PERSISTENT);
		memcpy(Z_PTR_P(zv), pData, size);
		return Z_PTR_P(zv);
	}
	return NULL;
}

static zend_always_inline void *zend_hash_next_index_insert_ptr(ht: *mut HashTable, pData: *mut c_void)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, pData);
	zv = zend_hash_next_index_insert(ht, &tmp);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_index_update_mem(ht: *mut HashTable, h: zend_ulong, pData: *mut c_void, size_t size)
{
	void *p;

	p = pemalloc(size, ht->u.flags & HASH_FLAG_PERSISTENT);
	memcpy(p, pData, size);
	return zend_hash_index_update_ptr(ht, h, p);
}

static zend_always_inline void *zend_hash_next_index_insert_mem(ht: *mut HashTable, pData: *mut c_void, size_t size)
{
	zval tmp, *zv;

	ZVAL_PTR(&tmp, NULL);
	if ((zv = zend_hash_next_index_insert(ht, &tmp))) {
		Z_PTR_P(zv) = pemalloc(size, ht->u.flags & HASH_FLAG_PERSISTENT);
		memcpy(Z_PTR_P(zv), pData, size);
		return Z_PTR_P(zv);
	}
	return NULL;
}

static zend_always_inline void *zend_hash_find_ptr(ht: *const HashTable, key: *mut zend_string)
{
	zval *zv;

	zv = zend_hash_find(ht, key);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_str_find_ptr(ht: *const HashTable, str: *const c_char, len: usize)
{
	zval *zv;

	zv = zend_hash_str_find(ht, str, len);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_hash_index_find_ptr(ht: *const HashTable, h: zend_ulong)
{
	zval *zv;

	zv = zend_hash_index_find(ht, h);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}

static zend_always_inline void *zend_symtable_str_find_ptr(ht: *mut HashTable, str: *const c_char, len: usize)
{
	zend_ulong idx;

	if (ZEND_HANDLE_NUMERIC_STR(str, len, idx)) {
		return zend_hash_index_find_ptr(ht, idx);
	} else {
		return zend_hash_str_find_ptr(ht, str, len);
	}
}

static zend_always_inline void *zend_hash_get_current_data_ptr_ex(ht: *mut HashTable, HashPosition *pos)
{
	zval *zv;

	zv = zend_hash_get_current_data_ex(ht, pos);
	if (zv) {
		ZEND_ASSUME(Z_PTR_P(zv));
		return Z_PTR_P(zv);
	} else {
		return NULL;
	}
}
*/


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