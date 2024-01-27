use crate::models::{PactResultErrorDto, PactResultSuccessDto};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandResultDtoResult {
    Success(PactResultSuccessDto),
    Error(PactResultErrorDto),
}
