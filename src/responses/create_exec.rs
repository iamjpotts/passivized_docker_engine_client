use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Exec/operation/ContainerExec
#[derive(Clone, Debug, Deserialize)]
pub struct CreateExecResponse {

    #[serde(rename = "Id")]
    pub id: String

}
