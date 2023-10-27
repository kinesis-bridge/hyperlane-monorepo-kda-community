#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleDto {
    #[serde(rename = "namespace", deserialize_with = "Option::deserialize")]
    pub namespace: Option<serde_json::Value>,
    #[serde(rename = "name")]
    pub name: String,
}

impl ModuleDto {
    pub fn new(namespace: Option<serde_json::Value>, name: String) -> ModuleDto {
        ModuleDto { namespace, name }
    }
}
