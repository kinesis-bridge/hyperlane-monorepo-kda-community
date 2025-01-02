#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct LocalRequestBodyDto {
    #[serde(rename = "cmd")]
    pub cmd: Box<crate::models::CommandDto>,
    #[serde(rename = "hostapi")]
    pub hostapi: String,
    #[serde(rename = "preflight")]
    pub preflight: bool,
    #[serde(rename = "signatureVerification")]
    pub signature_verification: bool,
    #[serde(rename = "rewindDepth", skip_serializing_if = "Option::is_none")]
    pub rewind_depth: Option<u64>,
}

impl LocalRequestBodyDto {
    pub fn new(
        cmd: crate::models::CommandDto,
        hostapi: String,
        preflight: bool,
        signature_verification: bool,
        rewind_depth: Option<u64>,
    ) -> LocalRequestBodyDto {
        LocalRequestBodyDto {
            cmd: Box::new(cmd),
            hostapi,
            preflight,
            signature_verification,
            rewind_depth,
        }
    }
}
