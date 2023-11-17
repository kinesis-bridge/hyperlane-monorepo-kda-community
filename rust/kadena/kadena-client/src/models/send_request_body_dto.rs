#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendRequestBodyDto {
    #[serde(rename = "cmds")]
    pub cmds: Vec<crate::models::CommandDto>,
    #[serde(rename = "hostapi")]
    pub hostapi: String,
}

impl SendRequestBodyDto {
    pub fn new(cmds: Vec<crate::models::CommandDto>, hostapi: String) -> SendRequestBodyDto {
        SendRequestBodyDto { cmds, hostapi }
    }
}
