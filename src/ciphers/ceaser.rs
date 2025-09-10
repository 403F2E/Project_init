use crate::black_box::{Cipher, CipherResult};

#[derive(Debug)]
pub struct Ceaser {
    pub key: u8,
}

impl Ceaser {
    pub fn new(key: u8) -> Self {
        Self { key }
    }
}

impl Cipher for Ceaser {
    fn cipher(&self, plain_text: &str) -> CipherResult {
        let encrypted = plain_text
            .bytes()
            .map(|c| -> char {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((c + self.key - base) % 26) + base) as char
                } else {
                    c as char
                }
            })
            .collect();
        Ok(encrypted)
    }

    fn decipher(&self, cipher_text: &str) -> CipherResult {
        let decrypted = cipher_text
            .bytes()
            .map(|c| -> char {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((c - base - self.key) % 26) + base) as char
                } else {
                    c as char
                }
            })
            .collect();
        Ok(decrypted)
    }
}
