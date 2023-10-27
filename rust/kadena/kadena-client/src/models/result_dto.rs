#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultDto {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "data")]
    pub data: String,
}

impl ResultDto {
    pub fn new(status: String, data: String) -> ResultDto {
        ResultDto { status, data }
    }
}
