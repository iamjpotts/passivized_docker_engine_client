#[path = "test_utils/lib.rs"]
mod test_utils;

use test_utils::random_name;

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::model::{NetworkIpam, NetworkIpamConfig};
use passivized_docker_engine_client::requests::CreateNetworkRequest;

#[tokio::test]
async fn test_create_inspect_and_remove_network() {
    const FN: &str = "test_create_inspect_and_remove_network";

    let dec = DockerEngineClient::new()
        .unwrap();

    let name = random_name(FN);
    let subnet = "172.97.97.0/24";
    let gateway = "172.97.97.1";

    let create_request = CreateNetworkRequest::default()
        .name(&name)
        .ipam(NetworkIpam::default()
            .config(NetworkIpamConfig::default()
                .subnet(subnet)
                .gateway(gateway)
            )
        );

    let create_response = dec.networks().create(create_request)
        .await
        .unwrap();

    assert_eq!("", create_response.warning);

    let inspected = dec.network(&name).inspect()
        .await
        .unwrap();

    dec.network(&name).remove()
        .await
        .unwrap();

    let ipam_config = inspected
        .ipam
        .config
        .get(0)
        .unwrap();

    #[cfg(windows)]
    const EXPECTED_DRIVER: &str = "nat";

    #[cfg(windows)]
    const EXPECTED_IPAM_DRIVER: &str = "windows";

    #[cfg(not(windows))]
    const EXPECTED_DRIVER: &str = "bridge";

    #[cfg(not(windows))]
    const EXPECTED_IPAM_DRIVER: &str = "default";

    assert_eq!(name, inspected.name);
    assert_eq!(EXPECTED_DRIVER, inspected.driver);
    assert_eq!(EXPECTED_IPAM_DRIVER, inspected.ipam.driver.unwrap());
    assert_eq!(subnet, ipam_config.subnet.as_ref().unwrap());
    assert_eq!(gateway, ipam_config.gateway.as_ref().unwrap());
}
