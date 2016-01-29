/// Since all characters used in Base64 happen to be only 1 byte in
/// UTF-8, to get a Base64 character, just do an index lookup on this
/// table.
static TAB_BASE64:&'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for Vec<u8> {
    fn to_base64(&self) -> String {
        // A fairly rudimentary base64 conversion
        let itr = (&self[..]).chunks(3)
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
}
