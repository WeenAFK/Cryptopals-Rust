use std::ops::BitXor;

use super::cipher;


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
