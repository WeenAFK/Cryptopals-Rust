/// Attempts to parse a hexadecimal string.
/// If the input string has an odd number of characters, the remaining bits in the last byte will
/// be filled with zeroes.
pub fn parse_hex_str(string: &str) -> Result<Vec<u8>, &'static str> {
    string.chars()
        .map(|ch| { ch.to_digit(16).map(|a_u32| { a_u32 as u8 }) })
        .map(|o| { o.ok_or("Input is not a valid hex string!") })
        .collect::<Result<Vec<u8>, &str>>() // collect is awesome
        .map(|vec:Vec<u8>| {
            // merge hex digit pairs into single bytes
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

pub trait HexParseable {
    fn parse_hex(&self) -> Result<Vec<u8>, &'static str>;
}

impl HexParseable for str {
    fn parse_hex(&self) -> Result<Vec<u8>, &'static str> {
        parse_hex_str(self)
    }
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

static TAB_HEX:&'static [u8] = b"0123456789abcdef";

impl ToHex for Vec<u8> {

    fn to_hex(&self) -> String {
        // Source: https://github.com/rust-lang/rust/blob/master/src/libserialize/hex.rs
        let mut v = Vec::with_capacity(self.len() * 2);
        for byte in self {
            v.push(TAB_HEX[(byte >> 4) as usize]);
            v.push(TAB_HEX[(byte & 0xF) as usize]);
        }

        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
}
