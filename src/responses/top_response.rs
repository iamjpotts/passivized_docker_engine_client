use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerTop
#[derive(Clone, Debug, Deserialize)]
pub struct TopResponse {

    #[serde(rename = "Titles")]
    pub titles: Vec<String>,

    #[serde(rename = "Processes")]
    pub processes: Vec<Vec<String>>

}
