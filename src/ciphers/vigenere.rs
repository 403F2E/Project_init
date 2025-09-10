use crate::{Cipher, CipherResult, TABULA_RECTA_LOWER, TABULA_RECTA_UPPER};

#[derive(Debug)]
pub struct Vigenere {
    key: String,
}

impl Vigenere {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}

impl Cipher for Vigenere {
    fn cipher(&self, plain_text: &str) -> CipherResult {
        let shift_slice: &[u8] = self.key.as_bytes();
        let mut adds: usize = 0usize;

        let encrypted = plain_text
            .bytes()
            .enumerate()
            .map(move |(i, c)| -> char {
                if c.is_ascii_alphabetic() {
                    match c.is_ascii_lowercase() {
                        true => {
                            let shift =
                                (shift_slice[(i + adds) % shift_slice.len()] - b'a') as usize;
                            TABULA_RECTA_LOWER[shift][(c - b'a') as usize] as char
                        }
                        false => {
                            let shift =
                                (shift_slice[(i + adds) % shift_slice.len()] - b'A') as usize;
                            TABULA_RECTA_UPPER[shift][(c - b'A') as usize] as char
                        }
                    }
                } else {
                    adds += 1;
                    c as char
                }
            })
            .collect();
        Ok(encrypted)
    }

    fn decipher(&self, cipher_text: &str) -> CipherResult {
        let shift_slice: &[u8] = self.key.as_bytes();
        let mut adds: usize = 0usize;

        let decrypted: String = cipher_text
            .bytes()
            .enumerate()
            .map(|(i, c)| -> char {
                if c.is_ascii_alphabetic() {
                    match c.is_ascii_lowercase() {
                        true => {
                            let shift = shift_slice[(i + adds) % shift_slice.len()] as usize - 97;
                            //TABULA_RECTA_LOWER[shift][(c + 97) as usize] as char;
                            (TABULA_RECTA_LOWER[shift]
                                .iter()
                                .position(|&j| j == c)
                                .unwrap() as u8
                                + 97) as char
                        }
                        false => {
                            let shift = shift_slice[(i + adds) % shift_slice.len()] as usize - 65;
                            //TABULA_RECTA_LOWER[shift][(c + 97) as usize] as char;
                            (TABULA_RECTA_LOWER[shift]
                                .iter()
                                .position(|&j| j == c)
                                .unwrap() as u8
                                + 65) as char
                        }
                    }
                } else {
                    adds += 1;
                    c as char
                }
            })
            .collect();
        Ok(decrypted)
    }
}
