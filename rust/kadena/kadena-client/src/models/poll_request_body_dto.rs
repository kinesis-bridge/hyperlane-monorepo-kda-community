#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PollRequestBodyDto {
    #[serde(rename = "requestKeys")]
    pub request_keys: Vec<String>,
    #[serde(rename = "hostapi")]
    pub hostapi: String,
}

impl PollRequestBodyDto {
    pub fn new(request_keys: Vec<String>, hostapi: String) -> PollRequestBodyDto {
        PollRequestBodyDto {
            request_keys,
            hostapi,
        }
    }
}
