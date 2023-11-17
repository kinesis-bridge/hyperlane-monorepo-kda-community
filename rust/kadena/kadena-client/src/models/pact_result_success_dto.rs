#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PactResultSuccessDto {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}

impl PactResultSuccessDto {
    pub fn new(status: String, data: serde_json::Value) -> PactResultSuccessDto {
        PactResultSuccessDto { status, data }
    }
}
