#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MetaDto {
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    #[serde(rename = "ttl")]
    pub ttl: u64,
    #[serde(rename = "gasLimit")]
    pub gas_limit: u64,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: f64,
    #[serde(rename = "sender")]
    pub sender: String,
}

impl MetaDto {
    pub fn new(
        creation_time: u64,
        ttl: u64,
        gas_limit: u64,
        chain_id: String,
        gas_price: f64,
        sender: String,
    ) -> MetaDto {
        MetaDto {
            creation_time,
            ttl,
            gas_limit,
            chain_id,
            gas_price,
            sender,
        }
    }
}
