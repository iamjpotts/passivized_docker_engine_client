#[path = "test_utils/lib.rs"]
mod test_utils;

use test_utils::random_name;

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::requests::CreateVolumeRequest;

/// Cargo runs tests in parallel, grouped by file, one file at a time. Consequentially,
/// if any test has a race with another test, those tests must be in separate files.
///
/// This test presents a race with other volume tests. Another test could have created
/// a volume it needs for a container it has not created yet. Prune will see that
/// volume as unused, and ready to prune.
#[tokio::test]
async fn test_create_and_prune_volume() {
    const FN: &str = "test_create_and_prune_volume";

    let dec = DockerEngineClient::new()
        .unwrap();

    let volume_name = random_name(FN);

    let request = CreateVolumeRequest::default()
        .name(&volume_name);

    dec.volumes().create(request)
        .await
        .unwrap();

    let pruning = dec.volumes().prune()
        .await
        .unwrap();

    assert!(pruning.volumes_deleted.contains(&volume_name));
}
