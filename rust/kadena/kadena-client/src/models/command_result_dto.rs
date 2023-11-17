#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandResultDto {
    #[serde(rename = "reqKey")]
    pub req_key: String,
    #[serde(rename = "txId", deserialize_with = "Option::deserialize")]
    pub tx_id: Option<u64>,
    #[serde(rename = "result")]
    pub result: Box<crate::models::CommandResultDtoResult>,
    #[serde(rename = "gas")]
    pub gas: u64,
    #[serde(rename = "logs", deserialize_with = "Option::deserialize")]
    pub logs: Option<String>,
    #[serde(rename = "continuation", deserialize_with = "Option::deserialize")]
    pub continuation: Option<Box<crate::models::PactExecDto>>,
    #[serde(rename = "metaData", deserialize_with = "Option::deserialize")]
    pub meta_data: Option<Box<crate::models::ChainwebResponseMetaDataDto>>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::PactEventDto>>,
}

impl CommandResultDto {
    pub fn new(
        req_key: String,
        tx_id: Option<u64>,
        result: crate::models::CommandResultDtoResult,
        gas: u64,
        logs: Option<String>,
        continuation: Option<crate::models::PactExecDto>,
        meta_data: Option<crate::models::ChainwebResponseMetaDataDto>,
    ) -> CommandResultDto {
        CommandResultDto {
            req_key,
            tx_id,
            result: Box::new(result),
            gas,
            logs,
            continuation: if let Some(x) = continuation {
                Some(Box::new(x))
            } else {
                None
            },
            meta_data: if let Some(x) = meta_data {
                Some(Box::new(x))
            } else {
                None
            },
            events: None,
        }
    }
}
