use crate::models::{ PactResultSuccessDto, PactResultErrorDto };

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandResultDtoResult {
    Success(PactResultSuccessDto),
    Error(PactResultErrorDto),
}
