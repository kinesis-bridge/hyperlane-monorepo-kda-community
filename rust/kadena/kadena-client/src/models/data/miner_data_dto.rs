#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MinerDataDto {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "predicate")]
    pub predicate: String,
    #[serde(rename = "public-keys")]
    pub public_keys: Vec<String>,
}

impl MinerDataDto {
    pub fn new(account: String, predicate: String, public_keys: Vec<String>) -> MinerDataDto {
        MinerDataDto {
            account,
            predicate,
            public_keys,
        }
    }
}
