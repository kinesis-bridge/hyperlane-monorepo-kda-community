#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SigDto {
    #[serde(rename = "sig")]
    pub sig: String,
}

impl SigDto {
    pub fn new(sig: String) -> SigDto {
        SigDto { sig }
    }
}
