use primitive_types::U256;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventDataParams {
    String(String),
    Integer(u64),
    Float(f64),
    IntObject(IntObject),
    DecimalObject(DecimalObject),
}

impl ToString for EventDataParams {
    fn to_string(&self) -> String {
        match self {
            EventDataParams::String(s) => s.clone(),
            EventDataParams::Integer(i) => i.to_string(),
            EventDataParams::Float(f) => f.to_string(),
            EventDataParams::IntObject(i) => i.int.to_string(),
            EventDataParams::DecimalObject(d) => d.decimal.to_string(),
        }
    }
}

impl TryInto<u64> for EventDataParams {
    type Error = std::num::ParseIntError;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            EventDataParams::String(s) => s.parse::<u64>(),
            EventDataParams::Integer(i) => Ok(i),
            EventDataParams::Float(f) => Ok(f as u64),
            EventDataParams::IntObject(i) => Ok(i.int),
            EventDataParams::DecimalObject(d) => Ok(d.decimal.as_u64()),
        }
    }
}

impl TryInto<u32> for EventDataParams {
    type Error = std::num::ParseIntError;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            EventDataParams::String(s) => s.parse::<u32>(),
            EventDataParams::Integer(i) => Ok(i as u32),
            EventDataParams::Float(f) => Ok(f as u32),
            EventDataParams::IntObject(i) => Ok(i.int as u32),
            EventDataParams::DecimalObject(d) => Ok(d.decimal.as_u32()),
        }
    }
}

impl TryInto<f64> for EventDataParams {
    type Error = std::num::ParseFloatError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            EventDataParams::String(s) => s.parse::<f64>(),
            EventDataParams::Integer(i) => Ok(i as f64),
            EventDataParams::Float(f) => Ok(f),
            EventDataParams::IntObject(i) => Ok(i.int as f64),
            EventDataParams::DecimalObject(d) => Ok(d.decimal.as_u64() as f64),
        }
    }
}

impl TryInto<U256> for EventDataParams {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<U256, Self::Error> {
        match self {
            EventDataParams::String(s) => Ok(U256::from_dec_str(&s)?),
            EventDataParams::Integer(i) => Ok(U256::from(i)),
            EventDataParams::Float(f) => Ok(U256::from(f as u64)),
            EventDataParams::IntObject(i) => Ok(U256::from(i.int)),
            EventDataParams::DecimalObject(d) => Ok(d.decimal),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventDataDto {
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(rename = "params")]
    pub params: Vec<EventDataParams>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "module")]
    pub module: Box<crate::models::ModuleDto>,
    #[serde(rename = "moduleHash")]
    pub module_hash: String,
}

impl EventDataDto {
    pub fn new(
        params: Vec<EventDataParams>,
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
