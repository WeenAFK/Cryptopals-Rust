use util::ioutil;
use util::xor;

// SET 1, CHALLENGE 6: http://cryptopals.com/sets/1/challenges/6/

pub fn main() {
    let bytes = ioutil::read_file_base64("res/1-6.txt").unwrap();
    let (key, plaintext) = xor::decrypt_vigenere(&bytes[0..512]);
    println!("Key: \"{}\"; plaintext: {}", key, plaintext);
}

#[test]
fn test1() {
    assert_eq!(xor::hamming_distance(b"this is a test", b"wokka wokka!!!"), Some(37));
}
