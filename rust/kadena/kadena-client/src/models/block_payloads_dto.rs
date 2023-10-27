#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockPayloadsDto {
    #[serde(rename = "header")]
    pub header: Box<crate::models::BlockHeaderDto>,
    #[serde(rename = "payload")]
    pub payload: Box<crate::models::BlockPayloadDto>,
}

impl BlockPayloadsDto {
    pub fn new(
        header: crate::models::BlockHeaderDto,
        payload: crate::models::BlockPayloadDto,
    ) -> BlockPayloadsDto {
        BlockPayloadsDto {
            header: Box::new(header),
            payload: Box::new(payload),
        }
    }
}
