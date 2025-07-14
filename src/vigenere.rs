use core::panic;

use crate::black_box::Cipher;

#[derive(Debug)]
pub struct Vigenere {
    pub key: &'static str,
    pub shifts: Vec<u8>,
    pub buffer: String,
}

impl Vigenere {
    pub fn new(key: &'static str) -> Self {
        if key.is_empty() {
            panic!("Vigenere key must not be empty.");
        }

        let shifts = key
            .chars()
            .map(|c| {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                c as u8 - base
            })
            .collect();

        Self {
            key,
            shifts,
            buffer: "".to_owned(),
        }
    }
}

impl Cipher for Vigenere {
    fn cipher(&mut self, plain_text: &str) -> &str {
        let mut shift_iter = self.shifts.iter().cycle();
        self.buffer.clear();

        self.buffer = plain_text
            .chars()
            .map(|c| -> char {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shift = shift_iter.next().unwrap();
                    (((c as u8 - base + shift) % 26) + base) as char
                } else {
                    c
                }
            })
            .collect();
        &self.buffer
    }

    fn decipher(&mut self, cipher_text: &str) -> &str {
        let mut shift_iter = self.shifts.iter().cycle();
        self.buffer.clear();
        self.buffer = cipher_text
            .chars()
            .map(|c| -> char {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shift = shift_iter.next().unwrap();
                    (((c as u8 - base + 26 - shift) % 26) + base) as char
                } else {
                    c
                }
            })
            .collect();
        &self.buffer
    }
}
