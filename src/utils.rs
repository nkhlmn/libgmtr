use crate::Cipher;
use crate::IntoNumReduction;

// Logic for calculating ciphers is here
pub fn calc_letter_val(letter: &char, cipher_type: &Cipher) -> u32 {
    if !letter.is_ascii_alphanumeric() {
        // If character is invalid, value is 0
        0
    } else if letter.is_numeric() {
        // If character is a number, value is same as the number
        letter.to_digit(10).unwrap()
    } else {
        // If character is a letter, calculate the value based on the cipher
        match cipher_type {
            Cipher::Ordinal => letter.to_ascii_lowercase() as u32 - 96,
            Cipher::OrdinalReverse => 27 - calc_letter_val(letter, &Cipher::Ordinal),
            Cipher::FullReduction => calc_letter_val(letter, &Cipher::Ordinal).num_reduction(),
            Cipher::FullReductionReverse => {
                calc_letter_val(letter, &Cipher::OrdinalReverse).num_reduction()
            }
            Cipher::FrancisBacon => {
                let mut val = calc_letter_val(letter, &Cipher::Ordinal);
                if letter.is_ascii_uppercase() {
                    val += 26
                }
                val
            }
            Cipher::FrancisBaconis => {
                let mut val: u32 = calc_letter_val(letter, &Cipher::Ordinal) * 2;
                if letter.is_ascii_uppercase() {
                    val -= 1;
                }
                val
            }
            Cipher::Satanic => calc_letter_val(letter, &Cipher::Ordinal) + 35,
            Cipher::Sumerian => calc_letter_val(letter, &Cipher::Ordinal) * 6,
            Cipher::SumerianReverse => calc_letter_val(letter, &Cipher::OrdinalReverse) * 6,
            Cipher::Agrippa => match letter {
                'k' => 10,
                'l' => 20,
                'm' => 30,
                'n' => 40,
                'o' => 50,
                'p' => 60,
                'q' => 70,
                'r' => 80,
                's' => 90,
                't' => 100,
                'u' => 200,
                'x' => 300,
                'y' => 400,
                'z' => 500,
                'j' => 600,
                'v' => 700,
                'w' => 900,
                _ => calc_letter_val(letter, &Cipher::Ordinal),
            },
            Cipher::AgrippaOrdinal => match letter {
                'k' => 10,
                'l' => 11,
                'm' => 12,
                'n' => 13,
                'o' => 14,
                'p' => 15,
                'q' => 16,
                'r' => 17,
                's' => 18,
                't' => 19,
                'u' => 20,
                'x' => 21,
                'y' => 22,
                'z' => 23,
                'j' => 24,
                'v' => 25,
                'w' => 27,
                _ => calc_letter_val(letter, &Cipher::Ordinal),
            },
            Cipher::AgrippaReduction => {
                let ordinal = calc_letter_val(letter, &Cipher::Ordinal);
                if ordinal > 10 && ordinal <= 21 {
                    match letter {
                        's' => 9,
                        _ => calc_letter_val(letter, &Cipher::FullReduction) - 1,
                    }
                } else {
                    match letter {
                        'x' => 3,
                        'y' => 4,
                        'z' => 5,
                        'j' => 6,
                        'v' => 7,
                        'w' => 9,
                        _ => ordinal,
                    }
                }
            }
        }
    }
}
