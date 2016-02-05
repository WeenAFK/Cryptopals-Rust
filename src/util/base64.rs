use std::collections::HashMap;
use std::io;

use util::ioutil;

pub trait Base64Parseable {
    fn parse_base64(&self) -> io::Result<Vec<u8>>;
}

impl Base64Parseable for str {
    fn parse_base64(&self) -> io::Result<Vec<u8>> {
        if self.len() % 4 != 0 {
            return ioutil::err("Length is not a multiple of 4!");
        }

        let map = inverse_map();

        // Note: it's strictly incorrect to use as_bytes() instead of chars() due to UTF-8,
        // but I'm lazy and for our use cases it doesn't matter.
        self.as_bytes().chunks(4)
            .map(|c:&[u8]| {
                // 3 bytes per 4 characters; each character represents 6 bits.
                // For each padding character we lose a byte.
                // We forward through a u32 containing the 24-bit word in the higher (leftmost)
                // bits and size data in the least significant (rightmost) bits.
                let mut chars = 4u32;
                if c[3] == '=' as u8 { chars -= 1; } // pad 1
                if c[2] == '=' as u8 { chars -= 1; } // pad 2
                let mut word = chars - 1;

                for i in 0..chars {
                    let val = map.get(&c[i as usize]);
                    if val.is_none() { return ioutil::err("Not a valid Base64 string!"); }
                    word |= (*val.unwrap() as u32) << (26 - 6*i);
                }

                Ok(word)
            }).collect::<io::Result<Vec<u32>>>()
            .map(|words| {
                let mut vec = Vec::with_capacity(3 * words.len());
                for word in words {
                    let len = word & 0b11;
                    for i in 0..len {
                        let byte = word >> (24 - 8*i);
                        vec.push(byte as u8);
                    }
                }
                vec
            })
    }
}

/// Since all characters used in Base64 happen to be only 1 byte in
/// UTF-8, to get a Base64 character, just do an index lookup on this
/// table.
static TAB_BASE64: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn inverse_map() -> HashMap<u8, u8> {
    let mut map = HashMap::<u8, u8>::new();
    for (i,ch) in TAB_BASE64.iter().enumerate() {
        map.insert(*ch, i as u8);
    }
    map
}

pub trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for Vec<u8> {
    fn to_base64(&self) -> String {
        // A fairly simple base64 conversion
        let itr = (&self[..]).chunks(3)
            .map(|c:&[u8]| {
                let mut chunk:u32 = 0;
                for n in 0..c.len() {
                    chunk |= (c[n] as u32) << (24 - 8*n); // read left-to-right
                }
                chunk += c.len() as u32;
                chunk
            });
        let len = if self.len() % 3 == 0 { 4 * (self.len() / 3) } else { 4 * (self.len() / 3) + 4 };
        let mut base64: Vec<u8> = Vec::with_capacity(len);
        for chunk in itr {
            let len:u32 = chunk & 0b11;
            let chars:u32 = len+1;
            for n in 0..chars {
                let idx:u32 = (chunk >> (26 - 6*n)) & 0b111111;
                let val:u8 = TAB_BASE64[idx as usize];
                base64.push(val);
            }

            if len < 3 { base64.push('=' as u8) }
            if len < 2 { base64.push('=' as u8) }
        }
        String::from_utf8(base64).unwrap()
    }
}
