use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use util::cipher::Cipher;
use util::hex::HexParseable;
use util::xor;

// SET 1, CHALLENGE 4: http://cryptopals.com/sets/1/challenges/4/

// Return a Result so we can abuse the try! macro
pub fn main() -> io::Result<()> {
    let f = try!(File::open("src/set1/4.txt"));
    let reader = BufReader::new(f);

    let res = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse_hex().ok())
        .map(|line| xor::find_best(&line).map(|val| (line, val)))
        .filter_map(|val| val)
        .max();

    if let None = res { return Ok(()); }

    let (ciphertext, rated_cipher) = res.unwrap();
    let cipher = rated_cipher.cipher;
    let plaintext = cipher.decrypt_str(&ciphertext).unwrap();
    println!("Best is: \"{}\" with key {:?}", plaintext.trim(), cipher.key);

    Ok(())
}
