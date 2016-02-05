extern crate crypto;

use self::crypto::{ aes, blockmodes, buffer };
use self::crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use self::crypto::symmetriccipher::Decryptor;

use util::ioutil;

// SET 1, CHALLENGE 7: http://cryptopals.com/sets/1/challenges/7/

pub fn main() {
    // Using the crypto library because I'm far too lazy to roll out my own AES implementation.
    // Code templated from: https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs

    let encrypted_data = ioutil::read_base64("res/1-7.txt").unwrap();

    let mut decryptor = aes::ecb_decryptor(
        aes::KeySize::KeySize128,
        b"YELLOW SUBMARINE",
        blockmodes::PkcsPadding
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    println!("SET 1-7 DECRYPTED: \n{}", String::from_utf8(final_result).unwrap());
}
