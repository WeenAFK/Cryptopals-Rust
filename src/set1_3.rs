#![allow(dead_code)]

use std::iter;
use set1_1;
use set1_2;

pub struct XorDecRes {
    pub key: u8,
    pub pts: u64,
    pub text: Option<String>,
}

pub fn count_freqs(text: &String) -> [f64; 26] {
    let mut freqs = [0_f64; 26];
    for ch in text.chars() {
        match ch {
            'a'...'z' => freqs[ch as usize - 'a' as usize] += 1_f64,
            'A'...'Z' => freqs[ch as usize - 'A' as usize] += 1_f64,
            _ => (),
        }
    }
    freqs
}

pub fn dot_eng(freqs: &[f64; 26]) -> f64 {
    // https://en.wikipedia.org/wiki/Letter_frequency
    let eng: [f64; 26] = [
        8.167,  // a
        1.492,  // b
        2.787,  // c
        4.253,  // d
        12.702, // e
        2.228,  // f
        2.015,  // g
        6.094,  // h
        6.966,  // i
        0.153,  // j
        0.772,  // k
        4.025,  // l
        2.406,  // m
        6.749,  // n
        7.507,  // o
        1.929,  // p
        0.095,  // q
        5.987,  // r
        6.327,  // s
        9.056,  // t
        2.758,  // u
        0.978,  // v
        2.361,  // w
        0.150,  // x
        1.974,  // y
        0.074,  // z
    ];

    eng.iter().zip(freqs.iter())
        .map(|(a,b)| a * b )
        .fold(0_f64, |acc, val| acc + val )
}

pub fn xor_with(text: &[u8], key: u8) -> Vec<u8> {
    set1_2::xor(text.iter(), iter::repeat(&key))
}

/// Assumes the cyphertext is encrypted by a 1-byte xor key.
pub fn find_best(hex_str: &str) -> Result<XorDecRes, &'static str> {
    let str_bytes = try!(set1_1::parse_hex_str(hex_str));
    (0_u8..).take(128)
        .map(|key| {
            let text = String::from_utf8(xor_with(&str_bytes, key)).ok();
            let pts: f64 = dot_eng(&count_freqs(&text.as_ref().unwrap_or(&"".to_owned())));
            XorDecRes {
                key: key,
                pts: (pts * 1000.0) as u64,
                text: text,
            }
        })
        .max_by_key(|val| val.pts)
        .ok_or("")
}



pub fn main() {
    let encoded_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let str_bytes = set1_1::parse_hex_str(encoded_str).unwrap();
    let best: Option<(u8, f64)> = (0_u8..).take(120)//.take(255)
        //.inspect(|b| { println!("Key: {}", b) })
        .map(|key| {
            (key, String::from_utf8(xor_with(&str_bytes, key)).unwrap_or("".to_owned()))
        })
        .inspect(|&(key,ref val)| { println!("Key: {}, String: {}", key as char, val) })
        .map(|(key,bytes)| {
            (key, dot_eng(&count_freqs(&bytes)))
        })
        //.inspect(|&(key,val)| { if val != 0.0 { println!("Key: {}, Fitness: {}", key, val) }})
        .max_by_key(|&(_,val)| (val * 1000_f64) as u64);
    let (key,fitness) = best.unwrap();
    println!("Key was: {}", key as char);
    println!("Fitness was: {}", fitness);
    let plaintext = String::from_utf8(xor_with(&str_bytes, key)).unwrap();

    println!("Decrypted: {}", plaintext);
}
