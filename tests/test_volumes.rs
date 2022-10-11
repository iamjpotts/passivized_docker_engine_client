#[path = "test_utils/lib.rs"]
mod test_utils;

use test_utils::{label_key, label_value, random_name};

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::errors::DecUseError;
use passivized_docker_engine_client::requests::CreateVolumeRequest;

#[tokio::test]
async fn test_create_list_and_delete_volume() {
    const FN: &str = "test_create_list_and_delete_volume";

    let dec = DockerEngineClient::new()
        .unwrap();

    let volume_name = random_name(FN);

    let request = CreateVolumeRequest::default()
        .name(&volume_name);

    dec.volumes().create(request)
        .await
        .unwrap();

    let volumes = dec.volumes().list()
        .await
        .unwrap()
        .volumes;

    assert!(volumes.iter().find(|vi| vi.name == volume_name).is_some());

    dec.volume(volume_name).remove(false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_create_inspect_list_and_remove_volume() {
    const FN: &str = "test_create_inspect_list_and_remove_volume";

    let dec = DockerEngineClient::new()
        .unwrap();

    let volume_name = random_name(FN);

    let label_key = label_key();
    let label_value = label_value();

    let create_request = CreateVolumeRequest::default()
        .name(&volume_name)
        .label(&label_key, &label_value);

    dec.volumes().create(create_request)
        .await
        .unwrap();

    {
        let inspected = dec.volume(&volume_name).inspect()
            .await
            .unwrap();

        assert_eq!(&label_value, inspected.labels.get(&label_key).unwrap());
    }

    {
        let volumes = dec.volumes().list()
            .await
            .unwrap();

        let found = volumes
            .volumes
            .iter()
            .find(|item| item.name == volume_name)
            .unwrap();

        assert_eq!(&label_value, found.labels.get(&label_key).unwrap());
    }

    dec.volume(&volume_name).remove(false)
        .await
        .unwrap();

    {
        let inspect_error = dec.volume(&volume_name).inspect()
            .await
            .unwrap_err();

        if let DecUseError::NotFound { message } = inspect_error {
            assert!(message.contains("no such volume"));
        }
        else {
            panic!("Unexpected inspect error: {}", inspect_error)
        }
    }

    {
        let volumes = dec.volumes().list()
            .await
            .unwrap();

        assert!(volumes
            .volumes
            .iter()
            .find(|item| item.name == volume_name)
            .is_none()
        );
    }

}
