use crate::client::ChainwebConf;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct ContinueTransferRemoteDto {
    host: String,
    network: String,
    #[serde(rename = "chain_id")]
    chain_id: u16,
    pact_id: String,
    destination_chain_id: u16,
    step: u8,
    rollback: bool,
}

impl ContinueTransferRemoteDto {
    pub fn new(
        conf: &ChainwebConf,
        pact_id: String,
        destination_chain_id: u16,
        step: u8,
        rollback: bool,
    ) -> Self {
        Self {
            host: conf.url.to_string(),
            network: conf.network_id.to_string(),
            chain_id: conf.chain_id as u16,
            pact_id,
            destination_chain_id,
            step,
            rollback,
        }
    }
}
