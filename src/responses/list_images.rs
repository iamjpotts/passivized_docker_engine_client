use std::collections::HashMap;

use serde::Deserialize;

use crate::imp::serde::{dz_hashmap, dz_vec};

/// See https://docs.docker.com/engine/api/v1.41/#tag/Image/operation/ImageList
#[derive(Clone, Debug, Deserialize)]
pub struct ListedImage {

    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "ParentId")]
    pub parent_id: String,

    #[serde(rename = "RepoTags", deserialize_with = "dz_vec")]
    pub repo_tags: Vec<String>,

    #[serde(rename = "RepoDigests", deserialize_with = "dz_vec")]
    pub repo_digests: Vec<String>,

    /// "Date and time at which the image was created as a Unix timestamp (number of seconds since EPOCH)."
    #[serde(rename = "Created")]
    pub created: i64,

    #[serde(rename = "Size")]
    pub size: i64,

    #[serde(rename = "SharedSize")]
    pub shared_size: i64,

    #[serde(rename = "VirtualSize")]
    pub virtual_size: i64,

    #[serde(rename = "Labels", deserialize_with = "dz_hashmap")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "Containers")]
    pub containers: i64,

}
