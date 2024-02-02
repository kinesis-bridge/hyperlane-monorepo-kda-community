#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct KeysetDto {
    #[serde(rename = "pred")]
    pub pred: String,
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
}

impl KeysetDto {
    pub fn new(pred: String, keys: Vec<String>) -> KeysetDto {
        KeysetDto { pred, keys }
    }
}
