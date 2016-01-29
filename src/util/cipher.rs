
pub trait Cipher {
    fn encrypt(&self, &[u8]) -> Vec<u8>;
    fn decrypt(&self, &[u8]) -> Vec<u8>;

    fn decrypt_str(&self, text: &[u8]) -> Option<String> {
        String::from_utf8(self.decrypt(text)).ok()
    }
}
