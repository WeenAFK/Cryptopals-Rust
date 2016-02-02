use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use util::base64::Base64Parseable;
use util::xor;

// SET 1, CHALLENGE 6: http://cryptopals.com/sets/1/challenges/6/

/// An alternative to the try! macro which doesn't require the enclosing function to return a
/// Result.
macro_rules! try_unwrap {
     ($e:expr) => (match $e { Ok(e) => e, Err(_) => return })
}

pub fn main() {
    let f = try_unwrap!(File::open("res/1-6.txt"));
    let reader = BufReader::new(f);

    let input = try_unwrap!(reader.lines().collect::<Result<String, _>>());
    let bytes = try_unwrap!(input.parse_base64());
    let candidates = xor::find_key_size(&bytes);

    //println!("Candidate sizes: {:?}", candidates);

    for i in 28usize..29 {
        match String::from_utf8(xor::decrypt_vigenere(&bytes[0..], i)) {
            Ok(text) => println!("Key size: {}, decrypted: {}", i, text),
            Err(_)   => println!("Key size {} has no valid decryption.", i)
        }
    }
}

#[test]
fn test1() {
    assert_eq!(xor::hamming_distance(b"this is a test", b"wokka wokka!!!"), Some(37));
}
