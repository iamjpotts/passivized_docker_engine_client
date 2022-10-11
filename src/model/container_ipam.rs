use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContainerIpamConfig {

    #[serde(rename = "IPv4Address")]
    pub ipv4_address: String,

    #[serde(rename = "IPv6Address", skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,

    #[serde(rename = "LinkLocalIPs", default, skip_serializing_if = "Vec::is_empty")]
    pub link_local_ips: Vec<String>

}

impl ContainerIpamConfig {

    /// Set the IPv4 address
    pub fn ipv4_address<V: ToString>(mut self, v: V) -> Self {
        self.ipv4_address = v.to_string();
        self
    }

    /// Set the IPv6 address
    pub fn ipv6_address<V: ToString>(mut self, v: V) -> Self {
        self.ipv6_address = Some(v.to_string());
        self
    }

    /// Add a link-local IP address
    pub fn link_local_ip<V: ToString>(mut self, v: V) -> Self {
        self.link_local_ips.push(v.to_string());
        self
    }

}

impl From<Ipv4Addr> for ContainerIpamConfig {
    fn from(value: Ipv4Addr) -> Self {
        ContainerIpamConfig {
            ipv4_address: value.to_string(),
            ..ContainerIpamConfig::default()
        }
    }
}

#[cfg(test)]
mod test_container_ipam_config {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::ContainerIpamConfig;

    #[test]
    fn create_from_ipv4() {
        let address = Ipv4Addr::new(10, 20, 30, 40);
        let actual: ContainerIpamConfig = address.into();

        let expected = ContainerIpamConfig {
            ipv4_address: "10.20.30.40".into(),
            ipv6_address: None,
            link_local_ips: vec![]
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn set_ipv4() {
        let actual = ContainerIpamConfig::default()
            .ipv4_address(Ipv4Addr::new(1, 2, 3, 4));

        assert_eq!("1.2.3.4", actual.ipv4_address);
    }

    #[test]
    fn set_ipv6() {
        let actual = ContainerIpamConfig::default()
            .ipv6_address(Ipv6Addr::new(0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8));

        assert_eq!("1:2:3:4:5:6:7:8", actual.ipv6_address.unwrap());
    }

}
