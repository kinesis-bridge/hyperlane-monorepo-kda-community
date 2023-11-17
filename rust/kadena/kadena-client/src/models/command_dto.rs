#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandDto {
    #[serde(rename = "cmd")]
    pub cmd: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "sigs", skip_deserializing)]
    pub sigs: Vec<crate::models::SignatureJsonDto>,
}

impl CommandDto {
    pub fn new(
        cmd: String,
        hash: String,
        sigs: Vec<crate::models::SignatureJsonDto>,
    ) -> CommandDto {
        CommandDto { cmd, hash, sigs }
    }
}
