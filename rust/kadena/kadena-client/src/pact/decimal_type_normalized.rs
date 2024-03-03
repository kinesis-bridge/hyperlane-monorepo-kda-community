use primitive_types::U256;
use serde::{
    de::Error,
    Deserialize,
    Deserializer,
};

use crate::constants::KDA_DECIMAL_PLACES;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DecimalObjectNormalized {
    #[serde(
        deserialize_with = "normalize_and_deserialize_decimal",
    )]
    pub decimal: U256,
}

fn normalize_and_deserialize_decimal<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let mut parts = s.split('.');
    let integer_part = parts.next().unwrap_or("0");
    let decimal_part = parts.next().unwrap_or_default();
    if decimal_part.len() > KDA_DECIMAL_PLACES as usize {
        return Err(D::Error::custom("Decimal part is too long"))
    }
    let normalized = format!("{}{:0<width$}", integer_part, decimal_part, width = KDA_DECIMAL_PLACES as usize);
    U256::from_dec_str(&normalized).map_err(D::Error::custom)
}
