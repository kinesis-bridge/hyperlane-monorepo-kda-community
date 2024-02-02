#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DataDto {
    #[serde(rename = "keyset")]
    pub keyset: Box<crate::models::KeysetDto>,
}

impl DataDto {
    pub fn new(keyset: crate::models::KeysetDto) -> DataDto {
        DataDto {
            keyset: Box::new(keyset),
        }
    }
}
