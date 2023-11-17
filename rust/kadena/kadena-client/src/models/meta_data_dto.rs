#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaDataDto {
    #[serde(rename = "creationTime")]
    pub creation_time: f32,
    #[serde(rename = "ttl")]
    pub ttl: f32,
    #[serde(rename = "gasLimit")]
    pub gas_limit: f32,
    #[serde(rename = "gasPrice")]
    pub gas_price: f32,
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "chainId")]
    pub chain_id: serde_json::Value,
}

impl MetaDataDto {
    pub fn new(
        creation_time: f32,
        ttl: f32,
        gas_limit: f32,
        gas_price: f32,
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
