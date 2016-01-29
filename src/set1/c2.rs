use util::hex::HexParseable;
use util::hex::ToHex;
use util::xor;

// SET 1, CHALLENGE 2: http://cryptopals.com/sets/1/challenges/2/

const IN_1: &'static str = "1c0111001f010100061a024b53535009181c";
const IN_2: &'static str = "686974207468652062756c6c277320657965";
const OUT: &'static str  = "746865206b696420646f6e277420706c6179";

pub fn main() {
    let out = xor::xor_bits(&IN_1.parse_hex().unwrap(), &IN_2.parse_hex().unwrap());
    println!("Result:   {}", out.to_hex());
    println!("Expected: {}", OUT);
}
