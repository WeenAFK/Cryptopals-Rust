#![allow(dead_code)]

/// Since all characters used in Base64 happen to be only 1 byte in
/// UTF-8, to get a Base64 character, just do an index lookup on this
/// table.
static TAB_BASE64:&'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static TAB_HEX:&'static [u8] = b"0123456789abcdef";

/// Parses a hexadecimal string
pub fn parse_hex_str(string: &str) -> Result<Vec<u8>, &'static str> {
    string.chars()
        .map(|ch| { ch.to_digit(16).map(|a_u32| { a_u32 as u8 }) })
        .map(|o| { o.ok_or("Input is not a hex string!") })
        .collect::<Result<Vec<u8>, &str>>()
        .map(|vec:Vec<u8>| {
            // 2 hex digits per byte
            (&vec[..]).chunks(2)
                .map(|c:&[u8]| {
                    let b:u8 = c[0] << 4;
                    if c.len() == 2 {
                        b + c[1]
                    } else {
                        b
                    }
                })
                .collect::<Vec<u8>>()
        })
}

// Source: https://github.com/rust-lang/rust/blob/master/src/libserialize/hex.rs
pub fn to_hex(data: Vec<u8>) -> String {
    let mut v = Vec::with_capacity(data.len() * 2);
    for byte in data {
        v.push(TAB_HEX[(byte >> 4) as usize]);
        v.push(TAB_HEX[(byte & 0xF) as usize]);
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

pub fn to_base64(data: Vec<u8>) -> String {
    let itr = (&data[..]).chunks(3)
        .map(|c:&[u8]| {
            let mut chunk:u32 = 0;
            for n in 0..c.len() {
                chunk |= (c[n] as u32) << (24 - 8*n); // read left-to-right
            }
            chunk += c.len() as u32;
            chunk
        });
    let mut base64: Vec<u8> = Vec::new();
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

pub fn main() {
    let string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    match parse_hex_str(string) {
        Ok(bytes) => println!("Result: {:?}", to_base64(bytes)),
        Err(msg)  => println!("{}", msg)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        super::main();
    }
}
