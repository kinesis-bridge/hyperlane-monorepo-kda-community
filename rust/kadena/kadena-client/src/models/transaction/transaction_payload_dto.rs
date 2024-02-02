#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TransactionPayloadDto {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "sigs")]
    pub sigs: Vec<crate::models::SigDto>,
    #[serde(rename = "cmd")]
    pub cmd: Box<crate::models::CmdDto>,
}

impl TransactionPayloadDto {
    pub fn new(
        hash: String,
        sigs: Vec<crate::models::SigDto>,
        cmd: crate::models::CmdDto,
    ) -> TransactionPayloadDto {
        TransactionPayloadDto {
            hash,
            sigs,
            cmd: Box::new(cmd),
        }
    }
}
