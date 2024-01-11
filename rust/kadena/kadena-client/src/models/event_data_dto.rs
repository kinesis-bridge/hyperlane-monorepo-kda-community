#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventDataParams {
    String(String),
    Integer(u64),
    Float(f64),
}

impl ToString for EventDataParams {
    fn to_string(&self) -> String {
        match self {
            EventDataParams::String(s) => s.clone(),
            EventDataParams::Integer(i) => i.to_string(),
            EventDataParams::Float(f) => f.to_string(),
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
