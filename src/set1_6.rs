#![allow(dead_code)]

pub fn main() {

}

#[test]
fn test2() {
    let str1 = "this is a test".to_owned().into_bytes();
    let str2 = "wokka wokka!!!".to_owned().into_bytes();
    assert_eq!(hamming_distance(&str1, &str2), 37);
}

pub fn hamming_distance(str1: &[u8], str2: &[u8]) -> u32 {
    str1.iter().zip(str2.iter())
        .map(|(a,b)| (a ^ b).count_ones())
        .fold(0u32, |acc,val| acc + val)
}
