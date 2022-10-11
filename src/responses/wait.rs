use serde::Deserialize;
use crate::responses::ErrorResponse;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerWait
#[derive(Clone, Debug, Deserialize)]
pub struct WaitResponse {

    /// Process exit code of the container.
    ///
    /// On Windows, can be larger than i32.
    #[serde(rename = "StatusCode")]
    exit_code: i64,

    /// "Container waiting error, if any"
    #[serde(rename = "Error")]
    error: Option<ErrorResponse>,

}

impl WaitResponse {

    /// Process exit code of the container
    pub fn exit_code(&self) -> i64 {
        self.exit_code
    }

    /// "Container waiting error, if any"
    pub fn error(&self) -> Option<&str> {
        self.error
            .as_ref()
            .map(|e| e.message.as_ref())
    }
}
