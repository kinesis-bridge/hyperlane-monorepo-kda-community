use primitive_types::U256;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

use super::{DecimalObjectNormalized, IntObject};
use crate::constants::KDA_SCALING_FACTOR;

#[derive(Clone, Debug, PartialEq)]
pub struct PicoKda(U256);

impl<'de> Deserialize<'de> for PicoKda {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let u256_value = match value {
            Value::Number(n) => {
                if n.is_u64() {
                    U256::from(n.as_u64().unwrap()).checked_mul(U256::from(KDA_SCALING_FACTOR))
                } else if n.is_f64() {
                    let scaled = n.as_f64().unwrap() * (KDA_SCALING_FACTOR as f64);
                    Some(U256::from(scaled.round() as u128))
                } else {
                    return Err(D::Error::custom("Invalid number type for PicoKda"));
                }
            }
            Value::Object(_) => {
                if let Ok(int_object) = IntObject::deserialize(&value) {
                    U256::from(int_object.int).checked_mul(U256::from(KDA_SCALING_FACTOR))
                } else if let Ok(decimal_object) = DecimalObjectNormalized::deserialize(&value) {
                    Some(decimal_object.decimal)
                } else {
                    return Err(D::Error::custom("Invalid object type for PicoKda"));
                }
            }
            _ => return Err(D::Error::custom("Invalid type for PicoKda")),
        };

        match u256_value {
            Some(value) => Ok(PicoKda(value)),
            None => Err(D::Error::custom("Overflow occurred while scaling PicoKda")),
        }
    }
}

impl Into<U256> for PicoKda {
    fn into(self) -> U256 {
        self.0
    }
}

impl Into<[u64; 4]> for PicoKda {
    fn into(self) -> [u64; 4] {
        self.0 .0
    }
}
