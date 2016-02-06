pub mod base64;
pub mod cipher;
pub mod freq;
pub mod hex;
pub mod ioutil;
pub mod math;
pub mod pad;
pub mod xor;


/*
/// An alternative to the try! macro which doesn't require the enclosing function to return a
/// Result.
#[macro_export]
macro_rules! try_unwrap {
     ($e:expr) => (match $e { Ok(e) => e, Err(_) => return })
}
*/
