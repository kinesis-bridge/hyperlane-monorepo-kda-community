use crate::{
    error::KadenaClientError,
    models::CommandResultDtoResult,
};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct CommandResultDto {
    #[serde(rename = "reqKey")]
    pub req_key: String,
    #[serde(rename = "txId", deserialize_with = "Option::deserialize")]
    pub tx_id: Option<u64>,
    #[serde(rename = "result")]
    pub result: Box<CommandResultDtoResult>,
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
        result: CommandResultDtoResult,
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
            continuation: continuation.map(Box::new),
            meta_data: meta_data.map(Box::new),
            events: None,
        }
    }

    pub fn result(&self) -> Result<Value, KadenaClientError> {
        match self.result.as_ref() {
            CommandResultDtoResult::Success(res) => Ok(res.data.clone()),
            CommandResultDtoResult::Error(err) => {
                Err(KadenaClientError::TransactionResultError(err.to_string()))
            }
        }
    }

    pub fn continuation(&self) -> Option<&crate::models::PactExecDto> {
        self.continuation.as_ref().map(|c| c.as_ref())
    }
}
