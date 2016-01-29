
pub trait Cipher {
    fn encrypt(&self, &[u8]) -> Vec<u8>;
    fn decrypt(&self, &[u8]) -> Vec<u8>;
}
