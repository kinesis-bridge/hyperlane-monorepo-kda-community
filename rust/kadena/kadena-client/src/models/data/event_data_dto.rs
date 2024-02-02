use primitive_types::U256;
use serde::{
    de::Error,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use strum_macros::EnumDiscriminants;

use crate::error::KadenaClientError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecimalObject {
    #[serde(
        deserialize_with = "deserialize_decimal",
        serialize_with = "serialize_decimal"
    )]
    decimal: U256,
}

fn deserialize_decimal<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    U256::from_dec_str(&s).map_err(D::Error::custom)
}

fn serialize_decimal<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = value.to_string();
    serializer.serialize_str(&s)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntObject {
    int: u64,
}

/// EventParam is a type that can be used to represent any of the possible types of event
/// parameter EventParamType is an associated enum that can be used to determine the type of the
/// EventParam
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumDiscriminants)]
#[serde(untagged)]
#[strum_discriminants(name(EventParamType))]
pub enum EventParam {
    String(String),
    Integer(u64),
    Float(f64),
    IntObject(IntObject),
    DecimalObject(DecimalObject),
    Array(Vec<EventParam>),
}

impl ToString for EventParam {
    fn to_string(&self) -> String {
        match self {
            EventParam::String(s) => s.clone(),
            EventParam::Integer(i) => i.to_string(),
            EventParam::Float(f) => f.to_string(),
            EventParam::IntObject(i) => i.int.to_string(),
            EventParam::DecimalObject(d) => d.decimal.to_string(),
            EventParam::Array(a) => {
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

impl TryInto<u64> for &EventParam {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            EventParam::String(s) => s.parse::<u64>().map_err(KadenaClientError::from),
            EventParam::Integer(i) => Ok(*i),
            EventParam::Float(f) => Ok(*f as u64),
            EventParam::IntObject(i) => Ok(i.int),
            EventParam::DecimalObject(d) => Ok(d.decimal.as_u64()),
            EventParam::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to u64".into(),
            )),
        }
    }
}

impl TryInto<u32> for &EventParam {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            EventParam::String(s) => s.parse::<u32>().map_err(KadenaClientError::from),
            EventParam::Integer(i) => Ok(*i as u32),
            EventParam::Float(f) => Ok(*f as u32),
            EventParam::IntObject(i) => Ok(i.int as u32),
            EventParam::DecimalObject(d) => Ok(d.decimal.as_u32()),
            EventParam::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to u32".into(),
            )),
        }
    }
}

impl TryInto<u8> for &EventParam {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            EventParam::String(s) => s.parse::<u8>().map_err(KadenaClientError::from),
            EventParam::Integer(i) => Ok(*i as u8),
            EventParam::Float(f) => Ok(*f as u8),
            EventParam::IntObject(i) => Ok(i.int as u8),
            EventParam::DecimalObject(d) => Ok(d.decimal.as_u32() as u8),
            EventParam::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to u8".into(),
            )),
        }
    }
}

impl TryInto<f64> for &EventParam {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            EventParam::String(s) => s.parse::<f64>().map_err(KadenaClientError::from),
            EventParam::Integer(i) => Ok(*i as f64),
            EventParam::Float(f) => Ok(*f),
            EventParam::IntObject(i) => Ok(i.int as f64),
            EventParam::DecimalObject(d) => Ok(d.decimal.as_u64() as f64),
            EventParam::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to f64".into(),
            )),
        }
    }
}

impl TryInto<U256> for &EventParam {
    type Error = KadenaClientError;

    fn try_into(self) -> Result<U256, Self::Error> {
        match self {
            EventParam::String(s) => U256::from_dec_str(&s).map_err(|_| {
                KadenaClientError::TypeConversionError("Cannot convert string to U256".into())
            }),
            EventParam::Integer(i) => Ok(U256::from(*i)),
            EventParam::Float(f) => Ok(U256::from(*f as u64)),
            EventParam::IntObject(i) => Ok(U256::from(i.int)),
            EventParam::DecimalObject(d) => Ok(d.decimal),
            EventParam::Array(_) => Err(KadenaClientError::TypeConversionError(
                "Cannot convert array to U256".into(),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct EventDataDto {
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(rename = "params")]
    pub params: Vec<EventParam>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "module")]
    pub module: Box<crate::models::ModuleDto>,
    #[serde(rename = "moduleHash")]
    pub module_hash: String,
}

impl EventDataDto {
    pub fn new(
        params: Vec<EventParam>,
        name: String,
        module: crate::models::ModuleDto,
        module_hash: String,
    ) -> EventDataDto {
        EventDataDto {
            height: None,
            params,
            name,
            module: Box::new(module),
            module_hash,
        }
    }
}
