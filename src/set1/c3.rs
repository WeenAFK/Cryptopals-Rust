use util::cipher::Cipher;
use util::hex::HexParseable;
use util::xor;

// SET 1, CHALLENGE 3: http://cryptopals.com/sets/1/challenges/3/

static IN: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn main() {
    let ciphertext = IN.parse_hex().unwrap();
    let best = xor::find_best(&ciphertext).unwrap();
    let plaintext = best.cipher.decrypt_str(&ciphertext).unwrap();
    println!("Best is: \"{}\" with key {:?}.", plaintext, best.cipher.key)

    // This program yields: Best is: "Cooking MC's like a pound of bacon" with key [88].
}
