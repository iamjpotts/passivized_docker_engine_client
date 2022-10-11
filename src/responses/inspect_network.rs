use std::collections::HashMap;

use serde::Deserialize;

use crate::model::NetworkIpam;
use crate::imp::serde::dz_empty_as_none;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Network/operation/NetworkInspect
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct InspectNetworkResponse {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "Scope")]
    pub scope: String,

    #[serde(rename = "Driver")]
    pub driver: String,

    #[serde(rename = "IPAM")]
    pub ipam: NetworkIpam,

    #[serde(rename = "Internal")]
    pub internal: bool,

    #[serde(rename = "Attachable")]
    pub attachable: bool,

    #[serde(rename = "Ingress")]
    pub ingress: bool,

    #[serde(rename = "Containers")]
    pub containers: HashMap<String, InspectNetworkResponseContainer>,

    #[serde(rename = "Options", default)]
    pub options: HashMap<String, String>,

    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,
}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Network/operation/NetworkInspect
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct InspectNetworkResponseContainer {

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,

    #[serde(rename = "MacAddress")]
    pub mac_address: String,

    #[serde(rename = "IPv4Address", deserialize_with="dz_empty_as_none")]
    pub ipv4_address: Option<String>,

    #[serde(rename = "IPv6Address", deserialize_with="dz_empty_as_none")]
    pub ipv6_address: Option<String>,

}