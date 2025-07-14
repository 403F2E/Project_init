pub trait Cipher {
    fn cipher(&mut self, plain_text: &str) -> &str;
    fn decipher(&mut self, cipher_text: &str) -> &str;
}
