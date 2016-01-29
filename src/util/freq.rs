//! Utilities for frequency analysis

use std::cmp::*;

use super::cipher;


pub struct FreqTable {
    /// 26 letters + space + punctuation
    table: [f64; 28]
}

/// 26 letters, + space and punctuation. Tuple contains approximate relative frequency,
/// and the character as a u8. We don't use this type since the u8 is unimportant.
type FreqTableVerbose = [(f64, u8); 28];

static FREQS_ENG_VERBOSE: FreqTableVerbose =
    [   // https://en.wikipedia.org/wiki/Letter_frequency
        (8.167, 'a' as u8),    (1.492, 'b' as u8),    (2.787, 'c' as u8),
        (4.253, 'd' as u8),    (12.702,'e' as u8),    (2.228, 'f' as u8),
        (2.015, 'g' as u8),    (6.094, 'h' as u8),    (6.966, 'i' as u8),
        (0.153, 'j' as u8),    (0.772, 'k' as u8),    (4.025, 'l' as u8),
        (2.406, 'm' as u8),    (6.749, 'n' as u8),    (7.507, 'o' as u8),
        (1.929, 'p' as u8),    (0.095, 'q' as u8),    (5.987, 'r' as u8),
        (6.327, 's' as u8),    (9.056, 't' as u8),    (2.758, 'u' as u8),
        (0.978, 'v' as u8),    (2.361, 'w' as u8),    (0.150, 'x' as u8),
        (1.974, 'y' as u8),    (0.074, 'z' as u8),    (17.20, ' ' as u8),
        (1.000, '.' as u8) // '.' placeholds for all punctuation
    ];

fn convert_table(tab: &FreqTableVerbose) -> FreqTable {
    let mut new: FreqTable = FreqTable { table: [0f64; 28] };
    for i in 0..tab.len() {
        new.table[i] = tab[i].0;
    }
    new
}

// FIXME: Unnecessary runtime computation
fn english_table() -> FreqTable {
    convert_table(&FREQS_ENG_VERBOSE)
}

impl FreqTable {

    pub fn new(text: &String) -> FreqTable {
        let mut tab: FreqTable = FreqTable { table: [0f64; 28] };
        for c in text.chars() {
            match c {
                'a'...'z' => tab.table[c as usize - 'a' as usize] += 1f64,
                'A'...'Z' => tab.table[c as usize - 'A' as usize] += 1f64,
                ' '       => tab.table[26] += 1f64,
                '.' | ',' | '!' | '?' => tab.table[27] += 1f64,
                _ => (),
            }
        }
        tab
    }

    pub fn score(&self) -> f64 {
        english_table().table.iter().zip(self.table.iter())
            .map(|(a,b)| a * b)
            .fold(0f64, |acc,val| acc + val)
    }

}

pub struct RatedCipher<T: cipher::Cipher> {
    pub cipher: T,
    pub score: f64,
}

impl<T: cipher::Cipher> PartialEq for RatedCipher<T> {
    fn eq(&self, _: &RatedCipher<T>) -> bool {
        false
    }
}

impl<T: cipher::Cipher> Eq for RatedCipher<T> {}

impl<T: cipher::Cipher> PartialOrd for RatedCipher<T> {
    fn partial_cmp(&self, other: &RatedCipher<T>) -> Option<Ordering> {
        if self.score > other.score {
            Some(Ordering::Greater)
        } else if self.score < other.score {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl<T: cipher::Cipher> Ord for RatedCipher<T> {
    fn cmp(&self, other: &RatedCipher<T>) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Uses frequency analysis to guess which cipher is most suited to decrypting the given
/// ciphertext.
pub fn find_best<T, I>(ciphertext: &[u8], candidates: I) -> Option<RatedCipher<T>>
        where T: cipher::Cipher,
              I: Iterator<Item=T> + Sized
{
    candidates
        .filter_map(|cipher| {
            String::from_utf8(cipher.decrypt(ciphertext)).ok()
                .map(|t| RatedCipher{ cipher:cipher, score: FreqTable::new(&t).score() })
        })
        .max()
        //.map(|rc| rc.cipher)
}
