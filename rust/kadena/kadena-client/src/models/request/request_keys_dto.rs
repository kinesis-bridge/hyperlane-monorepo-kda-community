#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct RequestKeysDto {
    #[serde(rename = "requestKeys")]
    pub request_keys: Vec<String>,
}

impl RequestKeysDto {
    pub fn new(request_keys: Vec<String>) -> RequestKeysDto {
        RequestKeysDto { request_keys }
    }
}
