use std::ops::BitXor;

use super::cipher;
use super::freq;
use super::freq::RatedCipher;


pub fn xor_bits(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    xor(buf1.iter(), buf2.iter())
}

/// A generic method which XORs everything from the given iterators and collects the result into a
/// Vec.
// Yes, this function is unnecessarily generic as we only work with u8s, but it was an interesting
// learning experience. Credit to nayru25
pub fn xor<'a, 'b, T, U, I, J>(itr1: I, itr2: J) -> Vec<<T as BitXor<U>>::Output>
    where T: 'a + BitXor<U>,
          U: 'b,
          I: Iterator<Item=T>,
          J: Iterator<Item=U>
{
    itr1.zip(itr2).map(|(a,b)| a ^ b).collect()
}

pub fn hamming_distance(str1: &[u8], str2: &[u8]) -> u32 {
    str1.iter().zip(str2.iter())
        .map(|(a,b)| (a ^ b).count_ones())
        .fold(0u32, |acc,val| acc + val)
}



pub struct XorCipher {
    pub key: Vec<u8>
}

impl XorCipher {
    pub fn new_byte(key: u8) -> XorCipher {
        XorCipher { key: vec!(key) }
    }

    pub fn new_byte_arr(key: Vec<u8>) -> XorCipher {
        XorCipher { key: key }
    }
}

impl cipher::Cipher for XorCipher {
    fn encrypt(&self, text: &[u8]) -> Vec<u8> {
        xor(text.iter(), self.key.iter().cycle())
    }

    fn decrypt(&self, text: &[u8]) -> Vec<u8> {
        self.encrypt(text) // xor undoes itself
    }
}

/// Uses frequency analysis to guess which single-character XorCipher is most suited to decrypting
/// the given ciphertext.
pub fn find_best(ciphertext: &[u8]) -> Option<RatedCipher<XorCipher>> {
    freq::find_best(ciphertext, (0u8..128).map(XorCipher::new_byte))
}
