use primitive_types::U256;
use serde::{
    Deserialize,
    Serialize,
};
use strum_macros::EnumDiscriminants;

use crate::error::KadenaClientError;
use super::{IntObject, DecimalObject};

/// PactValueType is an associated enum that can be used to determine the type of the PactValue
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumDiscriminants)]
#[serde(untagged)]
#[strum_discriminants(name(PactValueType))]
pub enum PactValue {
    String(String),
    Integer(u64),
    Float(f64),
    IntObject(IntObject),
    DecimalObject(DecimalObject),
    Array(Vec<PactValue>),
}

impl ToString for PactValue {
    fn to_string(&self) -> String {
        match self {
            PactValue::String(s) => s.clone(),
            PactValue::Integer(i) => i.to_string(),
            PactValue::Float(f) => f.to_string(),
            PactValue::IntObject(i) => i.int.to_string(),
            PactValue::DecimalObject(d) => d.decimal.to_string(),
            PactValue::Array(a) => {
                let mut s = String::from("[");
                for param in a {
                    s.push_str(&param.to_string());
                    s.push_str(", ");
                }
                s.pop();
                s.push(']');
                s
            }
        }
    }
}

impl TryInto<u64> for &PactValue {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            PactValue::String(s) => s.parse::<u64>().map_err(KadenaClientError::from),
            PactValue::Integer(i) => Ok(*i),
            PactValue::Float(f) => Ok(*f as u64),
            PactValue::IntObject(i) => Ok(i.int),
            PactValue::DecimalObject(d) => Ok(d.decimal.into()),
            PactValue::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to u64".into(),
            )),
        }
    }
}

impl TryInto<u32> for &PactValue {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            PactValue::String(s) => s.parse::<u32>().map_err(KadenaClientError::from),
            PactValue::Integer(i) => Ok(*i as u32),
            PactValue::Float(f) => Ok(*f as u32),
            PactValue::IntObject(i) => Ok(i.int as u32),
            PactValue::DecimalObject(d) => Ok(Into::<u64>::into(d.decimal) as u32),
            PactValue::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to u32".into(),
            )),
        }
    }
}

impl TryInto<u8> for &PactValue {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            PactValue::String(s) => s.parse::<u8>().map_err(KadenaClientError::from),
            PactValue::Integer(i) => Ok(*i as u8),
            PactValue::Float(f) => Ok(*f as u8),
            PactValue::IntObject(i) => Ok(i.int as u8),
            PactValue::DecimalObject(d) => Ok(Into::<u64>::into(d.decimal) as u8),
            PactValue::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to u8".into(),
            )),
        }
    }
}

impl TryInto<f64> for &PactValue {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            PactValue::String(s) => s.parse::<f64>().map_err(KadenaClientError::from),
            PactValue::Integer(i) => Ok(*i as f64),
            PactValue::Float(f) => Ok(*f),
            PactValue::IntObject(i) => Ok(i.int as f64),
            PactValue::DecimalObject(d) => Ok(Into::<u64>::into(d.decimal) as f64),
            PactValue::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to f64".into(),
            )),
        }
    }
}

impl TryInto<U256> for &PactValue {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<U256, Self::Error> {
        match self {
            PactValue::String(s) => U256::from_dec_str(&s).map_err(|_| {
                KadenaClientError::TypeConversionError("Cannot convert string to U256".into())
            }),
            PactValue::Integer(i) => Ok(U256::from(*i)),
            PactValue::Float(f) => Ok(U256::from(*f as u64)),
            PactValue::IntObject(i) => Ok(U256::from(i.int)),
            PactValue::DecimalObject(d) => Ok(d.decimal.into()),
            PactValue::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to U256".into(),
            )),
        }
    }
}
