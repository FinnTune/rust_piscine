#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let atbash_cipher = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = c as u8 - base;
                let cipher: char = (b'z' - offset).into();
                if c.is_ascii_uppercase() {
                    cipher.to_ascii_uppercase()
                } else {
                    cipher
                }
            } else {
                c
            }
        })
        .collect::<String>();

    println!("{} {} {}", ciphered, original, ciphered == "".to_string());
    if ciphered == "" || original == "" {
        return None
    }
    if atbash_cipher == ciphered {
        Some(Ok(true))
    } else {
        let error = CipherError::new(false, atbash_cipher);
        Some(Err(error))
    }
}