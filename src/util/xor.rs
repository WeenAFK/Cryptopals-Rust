use std::ops::BitXor;
use std::cmp::Ordering;
use std::collections::HashSet;

use util::cipher;
use util::cipher::Cipher;
use util::freq;
use util::freq::RatedCipher;
use util::math;


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

/// Returns a None if the inputs are of unequal length.
pub fn hamming_distance(a: &[u8], b: &[u8]) -> Option<u32> {
    if a.len() != b.len() {
        None
    } else {
        Some(a.iter().zip(b.iter())
            .map(|(a,b)| (a ^ b).count_ones())
            .fold(0u32, |acc,val| acc + val))
    }
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

impl Clone for XorCipher {
    fn clone(&self) -> Self {
        XorCipher { key: self.key.clone() }
    }
}

/// Uses frequency analysis to guess which single-character XorCipher is most suited to decrypting
/// the given ciphertext.
pub fn find_best(ciphertext: &[u8]) -> Option<RatedCipher<XorCipher>> {
    freq::find_best(ciphertext, (0u8..128).map(XorCipher::new_byte))
}

/// Returns the best-guess key size for the given ciphertext encrypted using a vigenere xor cipher.
pub fn find_key_size(ciphertext: &[u8]) -> usize {
    let mut list: Vec<(usize, f64)> = (1usize..)
        .map(|size| (size, key_size_rating(ciphertext, size)))
        .take_while(|&(_,rating)| rating.is_some())
        .map(|(size,rating)| (size, rating.unwrap()))
        .collect();
    list.sort_by(|&(_,a), &(_,b)| a.partial_cmp(&b).unwrap_or(Ordering::Equal)); // best to worst

    let mut iter = list.iter()
        //.inspect(|&&(size,score)| println!("Size: {}, score: {}", size, score))
        .map(|&(size,_)| size);

    // Some of the best keys may have lengths which are multiples of the true key size (so we may
    // end up with a key with cycling characters). To avoid this, we try to find the gcd of the
    // best keys and stop when we've narrowed ourselves down sufficiently.
    let mut factors = math::factors(iter.next().unwrap());
    for num in iter {
        let factors2 = math::factors(num);
        let intersection = factors.intersection(&factors2).cloned().collect::<HashSet<usize>>();
        if intersection.len() == 2 {
            // We've narrowed down our factors to {1, some_gcd}
            return *intersection.iter().max().unwrap();
        } else if intersection.len() == 1 {
            // We've narrowed down too far; go back
            return *factors.iter().max().unwrap();
        } else {
            factors = intersection;
        }
    }
    *factors.iter().max().unwrap()
}

/// Finds the rating for a key of size key_size for the given ciphertext. The result is given as
/// an Option because currently the used algorithm doesn't support key lengths greater than
/// ciphertext.len() / 2. The smaller the returned f64, the better.
fn key_size_rating(ciphertext: &[u8], key_size: usize) -> Option<f64> {
    let (count, hamming) = ciphertext.chunks(2 * key_size)//.take(1)
        .filter_map(|s| {
            if s.len() < 2 * key_size {
                None
            } else {
                hamming_distance(&s[0..key_size], &s[key_size..2*key_size])
            }
        })
        .fold((0usize, 0u32), |(n,acc), ham| (n + key_size, acc + ham));

    if count == 0 {
        None
    } else {
        Some(hamming as f64 / count as f64)
    }
    //let dist = hamming_distance(&ciphertext[0..key_size], &ciphertext[key_size..2*key_size]);
    //Some(dist.unwrap() as f64 / key_size as f64)
}

/// Returns the best-guess cipher for the specified key length.
fn find_vigenere_cipher(ciphertext: &[u8], key_size: usize) -> XorCipher {
    let key = freq::find_best_gen(ciphertext, (0u8..128).map(XorCipher::new_byte), key_size)
        .iter()
        .map(|rc| rc.cipher.key[0])
        .collect::<Vec<u8>>();
    XorCipher::new_byte_arr(key)
}

/// Attempts a vigenere xor decryption on the given ciphertext. Returns a tuple containing
/// (to our best guess) the key and decrypted pliantext.
pub fn decrypt_vigenere(ciphertext: &[u8]) -> (String, String) {
    let key_size = find_key_size(ciphertext);
    let cipher = find_vigenere_cipher(ciphertext, key_size);
    let key = String::from_utf8(cipher.key.clone()).unwrap();
    let plaintext = cipher.decrypt_str(ciphertext).unwrap();
    (key, plaintext)
}
