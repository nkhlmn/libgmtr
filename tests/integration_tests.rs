
use libgmtr::cipher::Cipher;
use libgmtr::CipherValue;

#[test]
fn ordinal_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::Ordinal), 351)
}

#[test]
fn ordinal_2() {
    let word = "Abc".to_string();
    assert_eq!(word.cipher_val(&Cipher::Ordinal), 6)
}

#[test]
fn ordinal_3() {
    let word = "Black Magic".to_string();
    assert_eq!(word.cipher_val(&Cipher::Ordinal), 62)
}

#[test]
fn ordinal_reverse_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::OrdinalReverse), 351)
}

#[test]
fn ordinal_reverse_2() {
    let word = "chaos".to_string();
    assert_eq!(word.cipher_val(&Cipher::OrdinalReverse), 89)
}

#[test]
fn full_reduction_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::FullReduction), 126)
}

#[test]
fn full_reduction_2() {
    let word = "room237".to_string();
    assert_eq!(word.cipher_val(&Cipher::FullReduction), 37)
}

#[test]
fn full_reduction_reverse_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(
        word.cipher_val(&Cipher::FullReductionReverse),
        126
        )
}

#[test]
fn francis_bacon_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::FrancisBacon), 351)
}

#[test]
fn francis_baconis_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::FrancisBaconis), 702)
}

#[test]
fn satanic_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::Satanic), 1261)
}

#[test]
fn satanic_2() {
    let word = "Satanic Gematria".to_string();
    assert_eq!(word.cipher_val(&Cipher::Satanic), 666)
}

#[test]
fn sumerian_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::Sumerian), 2106)
}

#[test]
fn sumerian_2() {
    let word = "Sumerian Gematria".to_string();
    assert_eq!(word.cipher_val(&Cipher::Sumerian), 1044)
}

#[test]
fn sumerian_reverse_1() {
    let word = "Sumerian Gematria".to_string();
    assert_eq!(word.cipher_val(&Cipher::SumerianReverse), 1548)
}

#[test]
fn sumerian_reverse_2() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::SumerianReverse), 2106)
}

#[test]
fn agrippa_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::Agrippa), 4195)
}

#[test]
fn agrippa_ordinal_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::AgrippaOrdinal), 352)
}

#[test]
fn agrippa_reduction_1() {
    let word = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(word.cipher_val(&Cipher::AgrippaReduction), 127);
}
