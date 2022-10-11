use serde::Deserialize;
use std::collections::HashMap;
use crate::responses::{Mount, NetworkSettings};

// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize)]
pub struct ListedContainer {

    #[serde(rename = "Id")]
    pub id: String,

    /// Names will start with a forward slash
    #[serde(rename = "Names")]
    pub names: Vec<String>,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "ImageID")]
    pub image_id: String,

    #[serde(rename = "Command")]
    pub command: String,

    #[serde(rename = "Created")]
    pub created: u64,

    #[serde(rename = "Ports")]
    pub ports: Vec<PortMapping>,

    #[serde(rename = "SizeRW")]
    pub size_rw: Option<i64>,

    #[serde(rename = "SizeRootFS")]
    pub size_root_fs: Option<i64>,

    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "HostConfig")]
    pub host_config: ListedContainerHostConfig,

    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,

    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount>,
}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize)]
pub struct ListedContainerHostConfig {

    #[serde(rename = "NetworkMode")]
    pub network_mode: String

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct PortMapping {

    #[serde(rename = "IP")]
    pub ip: Option<String>,

    #[serde(rename = "PrivatePort")]
    pub private_port: u16,

    #[serde(rename = "PublicPort")]
    pub public_port: Option<u16>,

    #[serde(rename = "Type")]
    pub port_type: String

}
