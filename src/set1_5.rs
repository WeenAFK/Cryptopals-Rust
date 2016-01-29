#![allow(dead_code)]

use set1_1;
use set1_2;


pub fn main() {
    let input = "Burning 'em, if you ain't quick and nimble\n\
        I go crazy when I hear a cymbal".to_owned();
    let key = "ICE".to_owned();

    let bytes = set1_2::xor(
        input.into_bytes().iter(),
        key.into_bytes().iter().cycle()
    );
    let hex_str = set1_1::to_hex(bytes);

    println!("Ciphertext: {}", hex_str);
}
