use util::cipher::Cipher;
use util::freq;
use util::hex::HexParseable;
use util::xor::XorCipher;

// SET 1, CHALLENGE 3: http://cryptopals.com/sets/1/challenges/3/

static IN: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn main() {
    let ciphertext = IN.parse_hex().unwrap();
    let ciphers = (0u8..).take(128).map(XorCipher::new_byte);
    let best = freq::find_best(&ciphertext, ciphers).unwrap();
    let plaintext = String::from_utf8(best.decrypt(&ciphertext)).unwrap();
    println!("Best is \"{}\" with key {:?}.", plaintext, best.key)
}
