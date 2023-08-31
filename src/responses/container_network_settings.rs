use std::collections::HashMap;
use serde::Deserialize;

use crate::model::{ContainerIpamConfig, PortBinding};
use crate::imp::serde::{dz_empty_object_as_none, dz_hashmap_of_nullable};

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
/// and https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Network {

    #[serde(rename = "IPAMConfig", skip_serializing_if = "Option::is_none", deserialize_with="dz_empty_object_as_none")]
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
#[derive(Clone, Debug, Default, Deserialize)]
pub struct NetworkSettings {

    #[serde(rename = "Ports", default, deserialize_with = "dz_hashmap_of_nullable", skip_serializing_if = "HashMap::is_empty")]
    pub ports: HashMap<String, Vec<PortBinding>>,

    #[serde(rename = "Networks")]
    pub networks: HashMap<String, Network>

}

impl NetworkSettings {

    /// Get the first ip address of the first network, without regard
    /// to what kind of network it is on.
    ///
    /// Useful for simple cases in controlled environments, like automated tests.
    pub fn first_ip_address(&self) -> Option<&str> {
        self.networks
            .values()
            .map(|n| n.ip_address.as_str())
            .find(|ip| !ip.is_empty())
    }
}

#[cfg(test)]
mod test_inspect_container_response {

    mod test_first_ip_address {
        use std::collections::HashMap;
        use crate::responses::{Network, NetworkSettings};

        #[test]
        fn no_networks() {
            let response = NetworkSettings::default();

            assert_eq!(None, response.first_ip_address());
        }

        #[test]
        fn one_network_one_ip() {
            let settings = NetworkSettings {
                networks: HashMap::from([
                    (
                        "a".into(),
                        Network {
                            ip_address: "4.3.2.1".into(),
                            ..Network::default()
                        }
                    )
                ]),
                ..NetworkSettings::default()
            };

            assert_eq!("4.3.2.1", settings.first_ip_address().unwrap());
        }

        // Other networks have no ip addresses
        #[test]
        fn multiple_networks_one_ip() {
            let settings = NetworkSettings {
                networks: HashMap::from([
                    (
                        "a".into(),
                        Network::default()
                    ),
                    (
                        "b".into(),
                        Network {
                            ip_address: "5.4.3.2".into(),
                            ..Network::default()
                        }
                    ),
                    (
                        "c".into(),
                        Network::default()
                    ),
                ]),
                ..NetworkSettings::default()
            };

            assert_eq!("5.4.3.2", settings.first_ip_address().unwrap());
        }

    }
}