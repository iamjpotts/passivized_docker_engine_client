
mod fixtures {
    use std::fs::read_to_string;
    use std::path::PathBuf;

    pub fn path() -> PathBuf {
        PathBuf::from(file!())
            .parent()
            .unwrap()
            .join("fixtures")
    }

    pub fn json<S: Into<String>>(name: S) -> String {
        let file_name = path().join(name.into());

        read_to_string(file_name)
            .unwrap()
    }

}

pub mod client {

    pub mod version {

        pub mod response {
            use serde_json::Value::Object;
            use passivized_docker_engine_client::responses::{Component, Platform, VersionResponse};

            #[test]
            pub fn deserializes() {
                let text = super::super::super::fixtures::json("version-response.json");

                let actual: VersionResponse = serde_json::from_str(&text)
                    .unwrap();

                assert_eq!(
                    VersionResponse {
                        platform: Platform {
                            name: "string".into()
                        },
                        components: vec![
                            Component {
                                name: "Engine".to_string(),
                                version: "19.03.12".to_string(),
                                details: Object(Default::default())
                            }
                        ],
                        version: "19.03.12".to_string(),
                        api_version: "1.40".to_string(),
                        min_api_version: "1.12".to_string(),
                        git_commit: "48a66213fe".to_string(),
                        go_version: "go1.13.14".to_string(),
                        os: "linux".to_string(),
                        arch: "amd64".to_string(),
                        kernel_version: "4.19.76-linuxkit".to_string(),
                        experimental: true,
                        build_time: "2020-06-22T15:49:27.000000000+00:00".to_string()
                    },
                    actual
                )
            }
        }
    }
}

pub mod container {

    pub mod inspect {

        pub mod response {
            use passivized_docker_engine_client::responses::InspectContainerResponse;

            #[test]
            pub fn deserialize_with_null_volumes() {
                let text = super::super::super::fixtures::json("container-create-response.json");

                let response: InspectContainerResponse = serde_json::from_str(&text)
                    .unwrap();

                assert_eq!(0, response.config.volumes.len());
            }

            #[test]
            pub fn parses_on_fedora_host() {
                let text = super::super::super::fixtures::json("fedora-inspect-running-container-response.json");

                let response: InspectContainerResponse = serde_json::from_str(&text)
                    .unwrap();

                // Null on Fedora hosts. Deserialize as an empty map.
                assert_eq!(0, response.graph_driver.data.len());
            }

            #[test]
            pub fn parses_config() {
                let text = super::super::super::fixtures::json("container-create-response.json");

                let response: InspectContainerResponse = serde_json::from_str(&text)
                    .unwrap();

                assert_eq!(Some("SIGQUIT".to_string()), response.config.stop_signal);
            }

            #[test]
            pub fn parses_host_config() {
                let text = super::super::super::fixtures::json("container-create-response.json");

                let response: InspectContainerResponse = serde_json::from_str(&text)
                    .unwrap();

                assert_eq!("default".to_string(), response.host_config.network_mode);
            }
        }
    }

}

pub mod ipam_config {
    use passivized_docker_engine_client::model::ContainerIpamConfig;

    #[test]
    pub fn parses_ipv4() {
        let text = super::fixtures::json("ipam_config_ipv4.json");
        let actual: ContainerIpamConfig = serde_json::from_str(&text)
            .unwrap();

        assert_eq!(
            ContainerIpamConfig {
                ipv4_address: "10.0.0.230".into(),
                ipv6_address: None,
                link_local_ips: vec![]
            },
            actual
        )
    }
}

pub mod network_inspect_response {
    use passivized_docker_engine_client::responses::InspectNetworkResponse;

    #[test]
    pub fn parses() {
        let text = super::fixtures::json("network-inspect-response.json");
        let actual: InspectNetworkResponse = serde_json::from_str(&text)
            .unwrap();

        assert_eq!(
            "172.24.0.0/16",
            actual
                .ipam
                .config
                .get(0)
                .unwrap()
                .subnet
                .as_ref()
                .unwrap()
        );
    }

    #[test]
    pub fn parses_containers() {
        let text = super::fixtures::json("network-inspect-response-containers.json");
        let actual: InspectNetworkResponse = serde_json::from_str(&text)
            .unwrap();

        assert_eq!("bridge", actual.name);

        assert_eq!(
            "172.17.0.0/16",
            actual
                .ipam
                .config
                .get(0)
                .unwrap()
                .subnet
                .as_ref()
                .unwrap()
        );

        assert_eq!(
            "172.17.0.1",
            actual
                .ipam
                .config
                .get(0)
                .unwrap()
                .gateway
                .as_ref()
                .unwrap()
        );

        assert_eq!(1, actual.containers.len());
        let container = actual.containers
            .get("d8d14a2546ff1bcb9cc94fcc05679a052c3999638cf7f5fa8b8f884633806277")
            .unwrap();

        assert_eq!(Some("test_remove_running_container_15271630609986573176".into()), container.name.to_owned());
        assert_eq!("57aef4e8533a4472d4df3592e496072f1b099806b3ca77e47bafc45093807597", container.endpoint_id);
        assert_eq!("02:42:ac:11:00:02", container.mac_address);
        assert_eq!(Some("172.17.0.2/16".into()), container.ipv4_address);
        assert_eq!(None, container.ipv6_address);

        assert_eq!("true", actual.options.get("com.docker.network.bridge.default_bridge").unwrap());

        assert_eq!(0, actual.labels.len());
    }
}

pub mod network_settings {
    use passivized_docker_engine_client::model::PortBinding;
    use passivized_docker_engine_client::responses::NetworkSettings;

    #[test]
    pub fn parses_ports_without_bindings() {
        let text = super::fixtures::json("networksettings-ports-wo-portbindings.json");
        let actual: NetworkSettings = serde_json::from_str(&text)
            .unwrap();

        assert_eq!(2, actual.ports.len());

        let empty: Vec<PortBinding> = Vec::new();

        assert_eq!(Some(&empty), actual.ports.get("443/tcp"));
        assert_eq!(Some(&empty), actual.ports.get("5000/tcp"));
    }

}

pub mod portinfo {
    use passivized_docker_engine_client::responses::PortMapping;

    #[test]
    pub fn parses_private_port_and_ip() {
        let text = super::fixtures::json("portinfo-tcp-privateport.json");
        let actual: PortMapping = serde_json::from_str(&text)
            .unwrap();

        assert_eq!(
            PortMapping {
                ip: None,
                port_type: "tcp".into(),
                private_port: 80,
                public_port: None
            },
            actual
        )
    }
}