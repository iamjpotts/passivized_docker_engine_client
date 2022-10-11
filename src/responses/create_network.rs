
use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Network/operation/NetworkCreate
#[derive(Clone, Debug, Deserialize)]
pub struct CreateNetworkResponse {

    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "Warning")]
    pub warning: String,
}
