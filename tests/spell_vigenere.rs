#[cfg(test)]
mod vigenere_cipher {
    use project_init::{Cipher, Vigenere};

    #[test]
    fn vigenere_cipher_methode() {
        let plain_text = "hello, world";

        let vigenere = Vigenere::new("test");
        let cipher_text = vigenere.cipher(plain_text).unwrap();
        assert_eq!(cipher_text, "aideh, agkeh".to_owned());
    }

    #[test]
    fn vigenere_decipher_methode() {
        let cipher_text = "aideh, agkeh";

        let vigenere = Vigenere::new("test");
        let plain_text = vigenere.decipher(cipher_text).unwrap();
        assert_eq!(plain_text, "hello, world".to_owned());
    }
}
