#[cfg(test)]
mod ceaser_cipher {
    use project_initials::{black_box::Cipher, ceaser::Ceaser};

    #[test]
    fn ceaser_cipher_methode() {
        let plain_text = "hello, world";

        let mut binding = Ceaser::new(3);
        let cipher_text = Ceaser::cipher(&mut binding, plain_text);
        assert_eq!(cipher_text, "khoor, zruog");
    }
    #[test]
    fn ceaser_decipher_methode() {
        let cipher_text = "khoor, zruog";

        let mut binding = Ceaser::new(3);
        let plain_text = Ceaser::decipher(&mut binding, cipher_text);

        assert_eq!(plain_text, "hello, world");
    }
}

#[cfg(test)]
mod vigenere_cipher {
    use project_initials::{black_box::Cipher, vigenere::Vigenere};

    #[test]
    fn vigenere_cipher_methode() {
        let plain_text = "hello, world";

        let mut binding = Vigenere::new("test");
        let cipher_text = Vigenere::cipher(&mut binding, plain_text);
        assert_eq!(cipher_text, "aideh, agkeh");
    }

    #[test]
    fn vigenere_decipher_methode() {
        let cipher_text = "aideh, agkeh";

        let mut binding = Vigenere::new("test");
        let plain_text = Vigenere::decipher(&mut binding, cipher_text);
        assert_eq!(plain_text, "hello, world");
    }
}
