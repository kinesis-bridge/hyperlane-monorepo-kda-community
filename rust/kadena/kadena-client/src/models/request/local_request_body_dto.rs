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
}

impl LocalRequestBodyDto {
    pub fn new(
        cmd: crate::models::CommandDto,
        hostapi: String,
        preflight: bool,
        signature_verification: bool,
    ) -> LocalRequestBodyDto {
        LocalRequestBodyDto {
            cmd: Box::new(cmd),
            hostapi,
            preflight,
            signature_verification,
        }
    }
}
