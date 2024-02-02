#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct SignerDto {
    #[serde(rename = "pubKey")]
    pub pub_key: String,
}

impl SignerDto {
    pub fn new(pub_key: String) -> SignerDto {
        SignerDto { pub_key }
    }
}
