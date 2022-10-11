use std::collections::HashMap;
use serde::Deserialize;

use crate::model::{ContainerIpamConfig, PortBinding};
use crate::imp::serde::dz_hashmap_of_nullable;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
/// and https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Network {

    #[serde(rename = "IPAMConfig", skip_serializing_if = "Option::is_none")]
    pub ipam_config: Option<ContainerIpamConfig>,

    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,

    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    #[serde(rename = "NetworkID")]
    pub network_id: String,

    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,

    #[serde(rename = "Gateway")]
    pub gateway: String,

    #[serde(rename = "IPAddress")]
    pub ip_address: String,

    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: i32,

    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,

    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,

    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: i64,

    #[serde(rename = "MacAddress")]
    pub mac_address: String,

    #[serde(rename = "DriverOpts")]
    pub driver_opts: Option<HashMap<String, String>>
}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
/// and https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Deserialize)]
pub struct NetworkSettings {

    #[serde(rename = "Ports", default, deserialize_with = "dz_hashmap_of_nullable", skip_serializing_if = "HashMap::is_empty")]
    pub ports: HashMap<String, Vec<PortBinding>>,

    #[serde(rename = "Networks")]
    pub networks: HashMap<String, Network>

}
