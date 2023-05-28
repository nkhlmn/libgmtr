pub mod cipher;
pub mod utils;

use cipher::Cipher;
use utils::*;

pub trait CipherValue {
    /// Calculate the value of a given cipher
    fn cipher_val(&self, cipher_type: &Cipher) -> u32;
}

impl CipherValue for String {
    /// # Examples
    /// ```
    /// use libgmtr::{*, cipher::*};
    /// let val = "Hello World".to_string().cipher_val(&Cipher::Ordinal);
    /// assert_eq!(val, 124)
    /// ```
    fn cipher_val(&self, cipher_type: &Cipher) -> u32 {
        self.chars()
            .fold(0, |acc, letter| acc + letter.cipher_val(cipher_type))
    }
}

impl CipherValue for char {
    /// # Examples
    /// ```
    /// use libgmtr::{*, cipher::*};
    /// let val = 'w'.cipher_val(&Cipher::Ordinal);
    /// assert_eq!(val, 23)
    /// ```
    fn cipher_val(&self, cipher_type: &Cipher) -> u32 {
        get_letter_value(self, cipher_type)
    }
}

#[cfg(test)]
mod tests {
    use super::Cipher;
    use super::CipherValue;

    #[test]
    fn calc_cipher_val_0() {
        let word = "Hello".to_string();
        let cipher_type = Cipher::Ordinal;
        let val = word.cipher_val(&cipher_type);
        assert_eq!(val, 52)
    }

    #[test]
    fn calc_cipher_val_1() {
        let word = "Hello".to_string();
        let cipher_type = Cipher::FullReduction;
        let val = word.cipher_val(&cipher_type);
        assert_eq!(val, 25)
    }

    #[test]
    fn calc_cipher_val_2() {
        let word = "Hello".to_string();
        let cipher_type = Cipher::Agrippa;
        let val = word.cipher_val(&cipher_type);
        assert_eq!(val, 103)
    }
}
