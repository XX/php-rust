extern crate php_rust as php;

use php::zend::Module;
use php::{c_str, str_index, funs, ExecuteData, Zval, Array, ArrayApi};

#[no_mangle]
pub extern fn array_from_rust(_data: &ExecuteData, retval: &mut Zval) {
    let mut a = Array::new();
    a.insert(str_index!("foo"), 1);
    a.insert(str_index!("bar"), 2.0);
    a.insert(str_index!("baz"), "some message");

    retval.set_arr(&mut a);
}

#[no_mangle]
pub extern fn get_module() -> *mut Module {
    let mut module = Module::new(c_str!("rust_example_ext"), c_str!("0.1.0"));

    module.set_functions(funs![array_from_rust]);
    module.into_raw()
}