use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use util::cipher::Cipher;
use util::hex::HexParseable;
use util::xor;

// SET 1, CHALLENGE 4: http://cryptopals.com/sets/1/challenges/4/

pub fn main() {
    let f = File::open("src/set1/4.txt");
    if f.is_err() { return; }
    let f = f.unwrap();
    let reader = BufReader::new(f);

    let res = reader.lines()
        .filter_map(|line| line.ok()) // read each line; filter the Results
        .filter_map(|line| line.parse_hex().ok()) // parse each line as hex if possible
        .map(|line| xor::find_best(&line).map(|val| (line, val))) // find the best for this line
        .filter_map(|val| val) // get rid of any lines with no valid decryptions
        .max(); // get the line with the best decryption

    if res.is_none() { return; }

    let (ciphertext, rated_cipher) = res.unwrap();
    let cipher = rated_cipher.cipher;
    let plaintext = cipher.decrypt_str(&ciphertext).unwrap();
    println!("Best is: \"{}\" with key {:?}.", plaintext.trim(), cipher.key);

    // This program yields: Best is: "Now that the party is jumping" with key [53].
}
