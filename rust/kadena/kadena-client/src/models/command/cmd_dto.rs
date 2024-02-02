#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct CmdDto {
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(rename = "payload")]
    pub payload: Box<crate::models::CmdPayloadDto>,
    #[serde(rename = "signers")]
    pub signers: Vec<crate::models::SignerDto>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::MetaDto>,
    #[serde(rename = "nonce")]
    pub nonce: String,
}

impl CmdDto {
    pub fn new(
        network_id: String,
        payload: crate::models::CmdPayloadDto,
        signers: Vec<crate::models::SignerDto>,
        meta: crate::models::MetaDto,
        nonce: String,
    ) -> CmdDto {
        CmdDto {
            network_id,
            payload: Box::new(payload),
            signers,
            meta: Box::new(meta),
            nonce,
        }
    }
}
