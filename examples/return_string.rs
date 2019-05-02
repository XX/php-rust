extern crate php_rust as php;

use php::zend::Module;
use php::{c_str, funs, ExecuteData, Zval};

#[no_mangle]
pub extern fn hello_from_rust(_data: &ExecuteData, retval: &mut Zval) {
    retval.set_new_str("Hello from Rust!", false);
}

#[no_mangle]
pub extern fn get_module() -> *mut Module {
    let mut module = Module::new(c_str!("rust_example_ext"), c_str!("0.1.0"));

    module.set_functions(funs![hello_from_rust]);
    module.into_raw()
}