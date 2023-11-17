#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PactResultErrorDto {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: serde_json::Value,
}

impl PactResultErrorDto {
    pub fn new(status: String, error: serde_json::Value) -> PactResultErrorDto {
        PactResultErrorDto { status, error }
    }
}
