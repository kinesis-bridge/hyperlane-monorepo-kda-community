#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
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

impl ToString for PactResultErrorDto {
    fn to_string(&self) -> String {
        format!("Status: {}, Error: {}", self.status, self.error)
    }
}
