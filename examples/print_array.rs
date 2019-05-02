extern crate php_rust as php;

use php::zend::Module;
use php::{c_str, funs, ExecuteData, Zval, Value, ArrayApi};

#[no_mangle]
pub extern fn print_array(data: &ExecuteData, _retval: &mut Zval) {
    let params = data.parse_parameters().unwrap();
    let a = params[0].as_array().unwrap();

    for b in a.buckets_iter() {
        match b.get_val().unwrap().value() {
            Value::Null => println!("null"),
            Value::Bool(v) => println!("{:?}", v),
            Value::Long(v) => println!("{:?}", v),
            Value::Double(v) => println!("{:?}", v),
            Value::String(v) => println!("{:?}", v.as_str()),
            v => println!("{:?}", v),
        }
    }
}

#[no_mangle]
pub extern fn get_module() -> *mut Module {
    let mut module = Module::new(c_str!("rust_example_ext"), c_str!("0.1.0"));

    module.set_functions(funs![print_array]);
    module.into_raw()
}