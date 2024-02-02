use crate::models::EventParam;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PactEventDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "module")]
    pub module: serde_json::Value,
    #[serde(rename = "params")]
    pub params: Vec<EventParam>,
    #[serde(rename = "moduleHash")]
    pub module_hash: String,
}

impl PactEventDto {
    pub fn new(
        name: String,
        module: serde_json::Value,
        params: Vec<EventParam>,
        module_hash: String,
    ) -> PactEventDto {
        PactEventDto {
            name,
            module,
            params,
            module_hash,
        }
    }
}
