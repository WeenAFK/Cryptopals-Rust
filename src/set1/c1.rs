use util::hex::HexParseable;
use util::hex::ToHex;
use util::base64::ToBase64;
use util::base64::Base64Parseable;

// SET 1, CHALLENGE 1: http://cryptopals.com/sets/1/challenges/1/

const INPUT: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const EXPECTED_OUT: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

pub fn main() {
    match INPUT.parse_hex() {
        Ok(bytes) => {
            println!("Result:   {}", bytes.to_base64());
            println!("Expected: {}", EXPECTED_OUT);

            println!("Original:      {}", INPUT);
            println!("Reconstructed: {}", bytes.to_base64().parse_base64().unwrap().to_hex());
        },
        Err(msg)  => println!("{}", msg)
    }
}

#[test]
fn test() {
    assert_eq!(INPUT.parse_hex().unwrap().to_base64(), EXPECTED_OUT);
}
