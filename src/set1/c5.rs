
use util::cipher::Cipher;
use util::hex::ToHex;
use util::xor::XorCipher;

// SET 1, CHALLENGE 5: http://cryptopals.com/sets/1/challenges/5/

static INPUT: &'static str = "Burning 'em, if you ain't quick and nimble\n\
    I go crazy when I hear a cymbal";
static EXPECTED: &'static str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a262263\
24272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
static KEY: &'static str = "ICE";

pub fn main() {
    let cipher = XorCipher::new_byte_arr(KEY.as_bytes().to_vec());
    let ciphertext = cipher.encrypt(INPUT.as_bytes()).to_hex();
    println!("Ciphertext: {}", ciphertext);
    println!("Expected:   {}", EXPECTED);
}
