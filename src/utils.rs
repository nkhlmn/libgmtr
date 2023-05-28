use crate::Cipher;

// Logic for calculating ciphers is here
pub fn get_letter_value(letter: &char, cipher_type: &Cipher) -> u32 {
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
            Cipher::OrdinalReverse => 27 - get_letter_value(letter, &Cipher::Ordinal),
            Cipher::FullReduction => {
                let letter_value = get_letter_value(letter, &Cipher::Ordinal);
                match letter_value % 9 {
                    0 => 9,
                    _ => letter_value % 9,
                }
            }
            Cipher::FullReductionReverse => {
                let letter_value = get_letter_value(letter, &Cipher::OrdinalReverse);
                match letter_value % 9 {
                    0 => 9,
                    _ => letter_value % 9,
                }
            }
            Cipher::FrancisBacon => {
                let value: u32 = get_letter_value(letter, &Cipher::Ordinal);
                if letter.is_ascii_uppercase() {
                    value + 26
                } else {
                    value
                }
            }
            Cipher::FrancisBaconis => {
                let value: u32 = get_letter_value(letter, &Cipher::Ordinal);
                if letter.is_ascii_uppercase() {
                    (value * 2) - 1
                } else {
                    value * 2
                }
            }
            Cipher::Satanic => get_letter_value(letter, &Cipher::Ordinal) + 35,
            Cipher::Sumerian => get_letter_value(letter, &Cipher::Ordinal) * 6,
            Cipher::SumerianReverse => get_letter_value(letter, &Cipher::OrdinalReverse) * 6,
            Cipher::Agrippa => {
                let letter_str = letter.to_string();
                match letter_str.as_str() {
                    "k" => 10,
                    "l" => 20,
                    "m" => 30,
                    "n" => 40,
                    "o" => 50,
                    "p" => 60,
                    "q" => 70,
                    "r" => 80,
                    "s" => 90,
                    "t" => 100,
                    "u" => 200,
                    "x" => 300,
                    "y" => 400,
                    "z" => 500,
                    "j" => 600,
                    "v" => 700,
                    "w" => 900,
                    _ => get_letter_value(letter, &Cipher::Ordinal),
                }
            }
            Cipher::AgrippaOrdinal => {
                let letter_str = letter.to_string();
                match letter_str.as_str() {
                    "k" => 10,
                    "l" => 11,
                    "m" => 12,
                    "n" => 13,
                    "o" => 14,
                    "p" => 15,
                    "q" => 16,
                    "r" => 17,
                    "s" => 18,
                    "t" => 19,
                    "u" => 20,
                    "x" => 21,
                    "y" => 22,
                    "z" => 23,
                    "j" => 24,
                    "v" => 25,
                    "w" => 27,
                    _ => get_letter_value(letter, &Cipher::Ordinal),
                }
            }
            Cipher::AgrippaReduction => {
                let letter_str = letter.to_string();
                let ordinal = get_letter_value(letter, &Cipher::Ordinal);

                if ordinal > 10 && ordinal <= 21 {
                    match letter_str.as_str() {
                        "s" => 9,
                        _ => {
                            let reduction = get_letter_value(letter, &Cipher::FullReduction);
                            let out = reduction - 1;
                            out
                        }
                    }
                } else {
                    match letter_str.as_str() {
                        "x" => 3,
                        "y" => 4,
                        "z" => 5,
                        "j" => 6,
                        "v" => 7,
                        "w" => 9,
                        _ => ordinal,
                    }
                }
            }
        }
    }
}
