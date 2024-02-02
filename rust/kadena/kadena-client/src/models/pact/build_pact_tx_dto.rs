#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BuildPactTxDto {
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "network")]
    pub network: String,
    #[serde(rename = "chain_id")]
    pub chain_id: u32,
    #[serde(rename = "pactCode")]
    pub pact_code: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "senderAccount")]
    pub sender_account: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: u64,
}

impl BuildPactTxDto {
    pub fn new(
        host: String,
        network: String,
        chain_id: u32,
        pact_code: String,
        signer: String,
        sender_account: String,
        gas_limit: u64,
    ) -> BuildPactTxDto {
        BuildPactTxDto {
            host,
            network,
            chain_id,
            pact_code,
            signer,
            sender_account,
            gas_limit,
        }
    }
}
