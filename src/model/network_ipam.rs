use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::imp::serde::dz_hashmap;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Network/operation/NetworkCreate
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct NetworkIpam {

    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    #[serde(rename = "Config")]
    pub config: Vec<NetworkIpamConfig>,

    #[serde(rename = "Options", deserialize_with = "dz_hashmap")]
    pub options: HashMap<String, String>
}

impl NetworkIpam {

    pub fn driver<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.driver = Some(v.into());
        self
    }

    /// Add a configuration. Can be called more than once to add multiple configurations.
    pub fn config(mut self, v: NetworkIpamConfig) -> Self {
        self.config.push(v);
        self
    }

    /// Add an option. Can be called more than once to add multiple options.
    pub fn option<K, V>(mut self, k: K, v: V) -> Self
        where
            K: Into<String>,
            V: Into<String>
    {
        self.options.insert(k.into(), v.into());
        self
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Network/operation/NetworkCreate
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct NetworkIpamConfig {

    /// CIDR
    #[serde(rename = "Subnet", skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,

    /// CIDR
    #[serde(rename = "IPRange", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,

    /// IP address
    #[serde(rename = "Gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,

    /// device_name:IP address
    #[serde(rename = "AuxAddress", skip_serializing_if = "Option::is_none")]
    pub aux_address: Option<String>,
}

impl NetworkIpamConfig {

    /// A CIDR, e.g. "172.3.4.5/24"
    pub fn subnet<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.subnet = Some(v.into());
        self
    }

    /// A CIDR, e.g. "172.3.4.5/24"
    pub fn ip_range<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.ip_range = Some(v.into());
        self
    }

    /// An IP address, e.g. "172.3.4.1"
    pub fn gateway<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.gateway = Some(v.into());
        self
    }

    /// device_name:IP address
    pub fn aux_address<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.aux_address = Some(v.into());
        self
    }
}