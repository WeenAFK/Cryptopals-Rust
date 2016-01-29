#![allow(dead_code)]

use std::ops::BitXor;
use set1_1;

pub fn xor_bits(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    xor(buf1.iter(), buf2.iter())
}

/*
pub fn xor<'a, T, I1, I2>(itr1: I1, itr2: I2) -> Vec<T>
    where T: 'a,
          &'a T: BitXor<&'a T, Output=T>,
          I1: Iterator<Item=&'a T>,
          I2: Iterator<Item=&'a T>
{
    itr1.zip(itr2)
        .map(|(a, b)| { a ^ b })
        .collect()
}
*/

pub fn xor<'a, 'b, T, U, I, J>(itr1:I, itr2: J) -> Vec<<T as BitXor<U>>::Output>
    where T: 'a + BitXor<U>,
          U: 'b,
          I: Iterator<Item=T>,
          J: Iterator<Item=U>
{
    itr1.zip(itr2).map(|(a,b)| a ^ b).collect()
}

pub fn main() {
    let data1 = set1_1::parse_hex_str("1c0111001f010100061a024b53535009181c").unwrap();
    let data2 = set1_1::parse_hex_str("686974207468652062756c6c277320657965").unwrap();
    println!("Result: {:?}", set1_1::to_hex(xor_bits(&data1[..], &data2[..])));
}
