#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ExecDto {
    #[serde(rename = "data")]
    pub data: Box<crate::models::DataDto>,
    #[serde(rename = "code")]
    pub code: String,
}

impl ExecDto {
    pub fn new(data: crate::models::DataDto, code: String) -> ExecDto {
        ExecDto {
            data: Box::new(data),
            code,
        }
    }
}
