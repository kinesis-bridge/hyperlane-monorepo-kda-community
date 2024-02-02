#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BlockPayloadsDto {
    #[serde(rename = "header")]
    pub header: Box<super::BlockHeaderDto>,
    #[serde(rename = "payload")]
    pub payload: Box<super::BlockPayloadDto>,
}

impl BlockPayloadsDto {
    pub fn new(header: super::BlockHeaderDto, payload: super::BlockPayloadDto) -> BlockPayloadsDto {
        BlockPayloadsDto {
            header: Box::new(header),
            payload: Box::new(payload),
        }
    }
}
