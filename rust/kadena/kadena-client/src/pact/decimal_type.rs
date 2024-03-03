use primitive_types::U256;
use serde::{
    de::Error,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecimalValue {
    U256Value(U256),
    F64Value(f64),
}

impl ToString for DecimalValue {
    fn to_string(&self) -> String {
        match self {
            DecimalValue::U256Value(u) => u.to_string(),
            DecimalValue::F64Value(f) => f.to_string(),
        }
    }
}

impl Into<u64> for DecimalValue {
    fn into(self) -> u64 {
        match self {
            DecimalValue::U256Value(u) => u.as_u64(),
            DecimalValue::F64Value(f) => f as u64,
        }
    }
}

impl Into<U256> for DecimalValue {
    fn into(self) -> U256 {
        match self {
            DecimalValue::U256Value(u) => u,
            DecimalValue::F64Value(f) => U256::from(f as u128),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecimalObject {
    #[serde(
        deserialize_with = "deserialize_decimal",
        serialize_with = "serialize_decimal"
    )]
    pub decimal: DecimalValue,
}

fn deserialize_decimal<'de, D>(deserializer: D) -> Result<DecimalValue, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match U256::from_dec_str(&s) {
        Ok(u) => Ok(DecimalValue::U256Value(u)),
        Err(_) => match s.parse::<f64>() {
            Ok(f) => Ok(DecimalValue::F64Value(f)),
            Err(e) => Err(D::Error::custom(e)),
        },
    }
}

fn serialize_decimal<S>(value: &DecimalValue, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        DecimalValue::U256Value(u) => serializer.serialize_str(&u.to_string()),
        DecimalValue::F64Value(f) => serializer.serialize_str(&f.to_string()),
    }
}
