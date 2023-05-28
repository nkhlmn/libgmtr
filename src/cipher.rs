use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Cipher {
    Ordinal,
    OrdinalReverse,
    FullReduction,
    FullReductionReverse,
    FrancisBacon,
    FrancisBaconis,
    Satanic,
    Sumerian,
    SumerianReverse,
    Agrippa,
    AgrippaOrdinal,
    AgrippaReduction,
}

impl Default for Cipher {
    fn default() -> Self {
        Self::Ordinal
    }
}

impl fmt::Display for Cipher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cipher::OrdinalReverse => "Ordinal (R)".to_string(),
                Cipher::FullReduction => "Full Reduction".to_string(),
                Cipher::FullReductionReverse => "Full Reduction (R)".to_string(),
                Cipher::FrancisBacon => "Francis Bacon".to_string(),
                Cipher::FrancisBaconis => "Francis Baconis".to_string(),
                Cipher::SumerianReverse => "Sumerian (R)".to_string(),
                Cipher::AgrippaOrdinal => "Agrippa Oridinal".to_string(),
                Cipher::AgrippaReduction => "Agrippa Reduction".to_string(),
                _ => format!("{:?}", self),
            }
        )
    }
}

