use serde_json::Value;

use crate::client::ChainwebConf;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierDto {
    pub name: String,
    pub proof: Value,
    pub capabilities: Vec<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename_all = "camelCase")]
pub struct BuildPactTxDto {
    host: String,
    network: String,
    #[serde(rename = "chain_id")]
    chain_id: u32,
    pact_code: String,
    signer: String,
    sender_account: String,
    gas_limit: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    verifiers: Vec<VerifierDto>,
}

pub struct BuildPactTxDtoBuilder {
    dto: BuildPactTxDto,
}

impl BuildPactTxDtoBuilder {
    pub fn new(
        conf: &ChainwebConf,
        pact_code: String,
        signer: String,
        sender_account: String,
        gas_limit: u64,
    ) -> Self {
        let dto = BuildPactTxDto {
            host: conf.url.to_string(),
            network: conf.network_id.to_string(),
            chain_id: conf.chain_id as u32,
            pact_code,
            signer,
            sender_account,
            gas_limit,
            verifiers: vec![],
        };
        Self { dto }
    }

    pub fn with_verifiers(mut self, verifiers: Vec<VerifierDto>) -> Self {
        self.dto.verifiers = verifiers;
        self
    }

    pub fn build(self) -> BuildPactTxDto {
        self.dto
    }
}
