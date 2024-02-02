use crate::models::{
    PactResultErrorDto,
    PactResultSuccessDto,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandResultDtoResult {
    Success(PactResultSuccessDto),
    Error(PactResultErrorDto),
}

#[cfg(test)]
impl Default for CommandResultDtoResult {
    fn default() -> Self {
        CommandResultDtoResult::Success(PactResultSuccessDto::default())
    }
}
