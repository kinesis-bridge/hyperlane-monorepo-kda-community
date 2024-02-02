#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct CmdPayloadDto {
    #[serde(rename = "exec")]
    pub exec: Box<crate::models::ExecDto>,
}

impl CmdPayloadDto {
    pub fn new(exec: crate::models::ExecDto) -> CmdPayloadDto {
        CmdPayloadDto {
            exec: Box::new(exec),
        }
    }
}
