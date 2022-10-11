use passivized_docker_engine_client::DockerEngineClient;

#[tokio::test]
async fn test_get_server_version() {
    let dec = DockerEngineClient::new()
        .unwrap();

    println!("Will connect to {}", dec);

    let version = dec.version()
        .await
        .unwrap();

    println!("Version: {}", version.version);
    println!("API version: {}", version.api_version);
    println!("Min API version: {}", version.min_api_version);

    assert!(
        version.components
            .iter()
            .any(|c| &c.name == "Engine")
    );
}