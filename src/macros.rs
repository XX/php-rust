
#[macro_export]
macro_rules! c_str {
    ($s:expr) => {{
        concat!($s, "\0").as_ptr() as *const ::std::os::raw::c_char
    }}
}

#[macro_export]
macro_rules! funs {
    ($($fname:ident),*) => {
        Box::new(
            [$($crate::zend::Function::new(c_str!(stringify!($fname)), $fname),)* $crate::zend::Function::end()]
        )
    }
}

/*
#ifndef ZEND_MM_ALIGNMENT
# define ZEND_MM_ALIGNMENT Z_L(8)
# define ZEND_MM_ALIGNMENT_LOG2 Z_L(3)
#elif ZEND_MM_ALIGNMENT < 4
# undef ZEND_MM_ALIGNMENT
# undef ZEND_MM_ALIGNMENT_LOG2
# define ZEND_MM_ALIGNMENT Z_L(4)
# define ZEND_MM_ALIGNMENT_LOG2 Z_L(2)
#endif
#define ZEND_MM_ALIGNMENT_MASK ~(ZEND_MM_ALIGNMENT - Z_L(1))
#define ZEND_MM_ALIGNED_SIZE(size) (((size) + ZEND_MM_ALIGNMENT - Z_L(1)) & ZEND_MM_ALIGNMENT_MASK)

#define _ZSTR_HEADER_SIZE XtOffsetOf(zend_string, val)
#define _ZSTR_STRUCT_SIZE(len) (_ZSTR_HEADER_SIZE + len + 1)

# define ZEND_FILE_LINE_EMPTY_C	NULL, 0
# define ZEND_FILE_LINE_C	__FILE__, __LINE__

*/