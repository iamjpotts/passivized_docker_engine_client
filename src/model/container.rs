use serde::{Deserialize, Serialize};

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
///
/// # Example - port only
///
/// ```rust
/// use passivized_docker_engine_client::requests::HostConfig;
///
/// let host_config = HostConfig::default()
///     .bind_port("80", "8080");
/// ```
///
/// # Example - port and ip
///
/// ```rust
/// use std::net::Ipv4Addr;
/// use passivized_docker_engine_client::requests::HostConfig;
///
/// let host_config = HostConfig::default()
///     .bind_ip("80", Ipv4Addr::new(127, 0, 0, 4), "8080");
/// ```
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PortBinding {

    #[serde(rename = "HostIP", skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,

    #[serde(rename = "HostPort")]
    pub host_port: String

}
