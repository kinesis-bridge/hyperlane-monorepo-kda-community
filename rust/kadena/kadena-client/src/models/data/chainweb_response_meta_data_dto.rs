#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ChainwebResponseMetaDataDto {
    #[serde(rename = "blockHash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    #[serde(rename = "blockTime")]
    pub block_time: f32,
    #[serde(rename = "blockHeight")]
    pub block_height: f32,
    #[serde(rename = "prevBlockHash")]
    pub prev_block_hash: String,
    #[serde(rename = "publicMeta", skip_serializing_if = "Option::is_none")]
    pub public_meta: Option<Box<crate::models::MetaDataDto>>,
}

impl ChainwebResponseMetaDataDto {
    pub fn new(
        block_time: f32,
        block_height: f32,
        prev_block_hash: String,
    ) -> ChainwebResponseMetaDataDto {
        ChainwebResponseMetaDataDto {
            block_hash: None,
            block_time,
            block_height,
            prev_block_hash,
            public_meta: None,
        }
    }
}
