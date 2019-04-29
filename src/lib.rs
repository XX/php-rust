pub mod zend;
pub mod macros;
pub mod safe_api;

pub use libc;
pub use safe_api::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
