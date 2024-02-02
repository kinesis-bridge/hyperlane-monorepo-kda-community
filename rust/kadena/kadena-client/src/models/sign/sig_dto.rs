#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct SigDto {
    #[serde(rename = "sig")]
    pub sig: String,
}

impl SigDto {
    pub fn new(sig: String) -> SigDto {
        SigDto { sig }
    }
}
