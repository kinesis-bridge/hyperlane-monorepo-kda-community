#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MetaDataDto {
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    #[serde(rename = "ttl")]
    pub ttl: u64,
    #[serde(rename = "gasLimit")]
    pub gas_limit: u64,
    #[serde(rename = "gasPrice")]
    pub gas_price: f64,
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "chainId")]
    pub chain_id: serde_json::Value,
}

impl MetaDataDto {
    pub fn new(
        creation_time: u64,
        ttl: u64,
        gas_limit: u64,
        gas_price: f64,
        sender: String,
        chain_id: serde_json::Value,
    ) -> MetaDataDto {
        MetaDataDto {
            creation_time,
            ttl,
            gas_limit,
            gas_price,
            sender,
            chain_id,
        }
    }
}
