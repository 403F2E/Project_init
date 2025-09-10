#[cfg(test)]
mod ceaser_cipher {
    use project_init::{Ceaser, Cipher};

    #[test]
    fn ceaser_cipher_methode() {
        let plain_text = "hello, world";

        let ceaser = Ceaser::new(3);
        let cipher_text = ceaser.cipher(plain_text).unwrap();
        assert_eq!(cipher_text, "khoor, zruog".to_owned());
    }
    #[test]
    fn ceaser_decipher_methode() {
        let cipher_text = "khoor, zruog";

        let ceaser = Ceaser::new(3);
        let plain_text = ceaser.decipher(cipher_text).unwrap();

        assert_eq!(plain_text, "hello, world".to_owned());
    }
}
