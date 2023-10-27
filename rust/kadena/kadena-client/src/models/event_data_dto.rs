#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventDataDto {
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    // TODO: implement deserialization for mixed array <number | string>
    #[serde(rename = "params", skip_deserializing)]
    pub params: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "module")]
    pub module: Box<crate::models::ModuleDto>,
    #[serde(rename = "moduleHash")]
    pub module_hash: String,
}

impl EventDataDto {
    pub fn new(
        params: Vec<String>,
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
