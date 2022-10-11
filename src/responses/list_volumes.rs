use serde::Deserialize;

use crate::model::Volume;
use crate::imp::serde::dz_vec;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Volume/operation/VolumeList
#[derive(Clone, Debug, Deserialize)]
pub struct ListVolumesResponse {

    #[serde(rename = "Volumes", deserialize_with = "dz_vec")]
    pub volumes: Vec<Volume>,

    #[serde(rename = "Warnings", deserialize_with = "dz_vec")]
    pub warnings: Vec<String>,

}
