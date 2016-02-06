
const PAD_CHAR: u8 = '\x04' as u8;

pub fn pad(bytes: &mut Vec<u8>, block_size: usize) {
    let rem = bytes.len() % block_size;
    if rem != 0 {
        for _ in 0..(block_size - rem) {
            bytes.push(PAD_CHAR);
        }
    }
}
