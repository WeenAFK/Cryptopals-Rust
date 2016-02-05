pub mod hex;
pub mod base64;
pub mod xor;
pub mod cipher;
pub mod freq;
pub mod math;
pub mod ioutil;

/*
/// An alternative to the try! macro which doesn't require the enclosing function to return a
/// Result.
#[macro_export]
macro_rules! try_unwrap {
     ($e:expr) => (match $e { Ok(e) => e, Err(_) => return })
}
*/
