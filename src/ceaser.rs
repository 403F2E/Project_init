use crate::black_box::Cipher;

#[derive(Debug)]
pub struct Ceaser {
    pub key: u8,
    pub buffer: String,
}

impl Ceaser {
    pub fn new(key: u8) -> Self {
        Self {
            key,
            buffer: "".to_owned(),
        }
    }
}

impl Cipher for Ceaser {
    fn cipher(&mut self, plain_text: &str) -> &str {
        self.buffer.clear();
        self.buffer = plain_text
            .chars()
            .map(|c| -> char {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((c as u8 - base + self.key) % 26) + base) as char
                } else {
                    c
                }
            })
            .collect();
        &self.buffer
    }

    fn decipher(&mut self, cipher_text: &str) -> &str {
        self.buffer.clear();
        self.buffer = cipher_text
            .chars()
            .map(|c| -> char {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    (((c as u8 - base - self.key) % 26) + base) as char
                } else {
                    c
                }
            })
            .collect();
        &self.buffer
    }
}
