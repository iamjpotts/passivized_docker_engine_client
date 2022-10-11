use std::collections::HashMap;

use serde::Deserialize;

use crate::imp::serde::dz_hashmap;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Volume/operation/VolumeInspect
/// and https://docs.docker.com/engine/api/v1.41/#tag/Volume/operation/VolumeList
#[derive(Clone, Debug, Deserialize)]
pub struct Volume {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Driver")]
    pub driver: String,

    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,

    #[serde(rename = "CreatedAt")]
    pub created_at: String,

    #[serde(rename = "Status", deserialize_with = "dz_hashmap", default)]
    pub status: HashMap<String, String>,

    #[serde(rename = "Labels", deserialize_with = "dz_hashmap")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "Scope")]
    pub scope: String,

    #[serde(rename = "Options", deserialize_with = "dz_hashmap")]
    pub options: HashMap<String, String>,

    #[serde(rename = "UsageData")]
    pub usage_data: Option<VolumeUsage>,
}

// https://docs.docker.com/engine/api/v1.41/#tag/Volume/operation/VolumeInspect
#[derive(Clone, Debug, Deserialize)]
pub struct VolumeUsage {
    // Warning - based on the above linked docs, this value can be negative. Do not retype as u64
    #[serde(rename = "Size")]
    pub size: i64,

    // Warning - based on the above linked docs, this value can be negative. Do not retype as u64
    #[serde(rename = "RefCount")]
    pub ref_count: i64,
}
