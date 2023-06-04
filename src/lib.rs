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

pub trait IntoNumReduction {
    fn num_reduction(&self) -> u32;
}

impl IntoNumReduction for u32 {
    /// # Examples
    /// ```
    /// use libgmtr::IntoNumReduction;
    ///
    /// let n: u32 = 351;
    /// assert_eq!(n.num_reduction(), 9)
    /// ```
    fn num_reduction(&self) -> u32 {
        match self {
            0..=9 => *self,
            _ => self
                .to_string()
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .fold(0, |acc, x: u32| acc + x)
                .num_reduction(),
        }
    }
}
