#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct SignatureJsonDto {
    #[serde(rename = "sig", deserialize_with = "Option::deserialize")]
    pub sig: Option<String>,
}

impl SignatureJsonDto {
    pub fn new(sig: Option<String>) -> SignatureJsonDto {
        SignatureJsonDto { sig }
    }
}
