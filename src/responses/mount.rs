use std::collections::HashMap;
use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize)]
pub struct BindOptions {

    #[serde(rename = "Propagation")]
    pub propagation: String,

    #[serde(rename = "NonRecursive")]
    pub non_recursive: bool

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize)]
pub struct DriverConfig {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Options")]
    pub options: HashMap<String, String>

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize)]
pub struct Mount {

    #[serde(rename = "Target")]
    pub target: Option<String>,

    #[serde(rename = "Source")]
    pub source: String,

    #[serde(rename = "Type")]
    pub mount_type: String,

    #[serde(rename = "ReadOnly", default)]
    pub read_only: bool,

    #[serde(rename = "Consistency", default)]
    pub consistency: String,

    #[serde(rename = "BindOptions")]
    pub bind_options: Option<BindOptions>,

    #[serde(rename = "VolumeOptions")]
    pub volume_options: Option<VolumeOptions>,

    #[serde(rename = "TmpfsOptions")]
    pub tmpfs_options: Option<TmpfsOptions>

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize)]
pub struct TmpfsOptions {

    #[serde(rename = "SizeBytes")]
    pub size_bytes: i64,

    #[serde(rename = "Mode")]
    pub mode: i32

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList
#[derive(Clone, Debug, Deserialize)]
pub struct VolumeOptions {

    #[serde(rename = "NoCopy")]
    pub no_copy: bool,

    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "DriverConfig")]
    pub driver_config: DriverConfig

}
