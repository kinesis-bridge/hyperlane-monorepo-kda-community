#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsignedCommandDto {
    #[serde(rename = "cmd")]
    pub cmd: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "sigs", skip_deserializing)]
    pub sigs: Vec<crate::models::SignatureJsonDto>,
}

impl UnsignedCommandDto {
    pub fn new(
        cmd: String,
        hash: String,
        sigs: Vec<crate::models::SignatureJsonDto>,
    ) -> UnsignedCommandDto {
        UnsignedCommandDto { cmd, hash, sigs }
    }
}
