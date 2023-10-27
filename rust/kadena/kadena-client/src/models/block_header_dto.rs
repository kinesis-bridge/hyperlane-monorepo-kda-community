#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockHeaderDto {
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    #[serde(rename = "parent")]
    pub parent: String,
    #[serde(rename = "height")]
    pub height: u64,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "chainId")]
    pub chain_id: u32,
    #[serde(rename = "weight")]
    pub weight: String,
    #[serde(rename = "featureFlags")]
    pub feature_flags: u32,
    #[serde(rename = "epochStart")]
    pub epoch_start: u64,
    #[serde(rename = "adjacents")]
    pub adjacents: ::std::collections::HashMap<String, String>,
    #[serde(rename = "payloadHash")]
    pub payload_hash: String,
    #[serde(rename = "chainwebVersion")]
    pub chainweb_version: String,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
}

impl BlockHeaderDto {
    pub fn new(
        creation_time: u64,
        parent: String,
        height: u64,
        hash: String,
        chain_id: u32,
        weight: String,
        feature_flags: u32,
        epoch_start: u64,
        adjacents: ::std::collections::HashMap<String, String>,
        payload_hash: String,
        chainweb_version: String,
        target: String,
        nonce: String,
    ) -> BlockHeaderDto {
        BlockHeaderDto {
            creation_time,
            parent,
            height,
            hash,
            chain_id,
            weight,
            feature_flags,
            epoch_start,
            adjacents,
            payload_hash,
            chainweb_version,
            target,
            nonce,
        }
    }
}
