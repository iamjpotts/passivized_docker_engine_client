use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
#[derive(Clone, Debug, Deserialize)]
pub struct CreateContainerResponse {

    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "Warnings")]
    pub warnings: Vec<String>,
}
