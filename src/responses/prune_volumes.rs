
use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Volume/operation/VolumePrune
#[derive(Clone, Debug, Deserialize)]
pub struct PruneVolumesResponse {

    #[serde(rename = "VolumesDeleted")]
    pub volumes_deleted: Vec<String>,

    #[serde(rename = "SpaceReclaimed")]
    pub space_reclaimed_bytes: u64,
}
