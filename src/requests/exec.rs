use serde::Serialize;

// See https://docs.docker.com/engine/api/v1.41/#tag/Exec/operation/ExecStart
#[derive(Clone, Debug, Default, Serialize)]
pub struct ExecStartRequest {

    #[serde(rename = "Detach")]
    pub detach: bool,

    #[serde(rename = "Tty")]
    pub tty: bool
}
