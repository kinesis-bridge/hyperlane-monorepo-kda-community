#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoinbaseDto {
    #[serde(rename = "gas")]
    pub gas: f32,
    #[serde(rename = "result")]
    pub result: Box<crate::models::ResultDto>,
    #[serde(rename = "reqKey")]
    pub req_key: String,
    #[serde(rename = "logs")]
    pub logs: String,
    #[serde(rename = "events")]
    pub events: Vec<crate::models::EventDataDto>,
    #[serde(rename = "metaData", deserialize_with = "Option::deserialize")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "continuation", deserialize_with = "Option::deserialize")]
    pub continuation: Option<serde_json::Value>,
    #[serde(rename = "txId")]
    pub tx_id: u64,
}

impl CoinbaseDto {
    pub fn new(
        gas: f32,
        result: crate::models::ResultDto,
        req_key: String,
        logs: String,
        events: Vec<crate::models::EventDataDto>,
        meta_data: Option<serde_json::Value>,
        continuation: Option<serde_json::Value>,
        tx_id: u64,
    ) -> CoinbaseDto {
        CoinbaseDto {
            gas,
            result: Box::new(result),
            req_key,
            logs,
            events,
            meta_data,
            continuation,
            tx_id,
        }
    }
}
