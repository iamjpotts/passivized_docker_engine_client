use std::collections::HashMap;

use serde::Serialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Volume/operation/VolumeCreate
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateVolumeRequest {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,

    #[serde(rename = "DriverOpts", skip_serializing_if = "HashMap::is_empty")]
    pub driver_opts: HashMap<String, String>,

    #[serde(rename = "Labels", skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
}

impl CreateVolumeRequest {

    pub fn name<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.name = Some(v.into());
        self
    }

    pub fn driver<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.driver = Some(v.into());
        self
    }

    /// Add a driver option. Can be called multiple times.
    pub fn driver_opt<K, V>(mut self, k: K, v: V) -> Self
        where
            K: Into<String>,
            V: Into<String>
    {
        self.driver_opts.insert(k.into(), v.into());
        self
    }

    /// Add a label. Can be called multiple times.
    pub fn label<K, V>(mut self, k: K, v: V) -> Self
        where
            K: Into<String>,
            V: Into<String>
    {
        self.labels.insert(k.into(), v.into());
        self
    }

}