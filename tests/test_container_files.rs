#![cfg(not(windows))]
#[path = "test_utils/lib.rs"]
mod test_utils;

use std::io::Read;
use tar::Header;
use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::model::Tar;
use passivized_docker_engine_client::requests::{CreateContainerRequest, HostConfig};
use crate::test_utils::images::web;
use crate::test_utils::random_name;

#[tokio::test]
async fn test_put_and_get_files() {
    const FN: &str = "test_put_and_get_files";

    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let name = random_name(FN);

    let request = CreateContainerRequest::default()
        .name(&name)
        .image(format!("{}:{}", web::IMAGE, web::TAG))
        .host_config(HostConfig::default().auto_remove());

    let container = dec.containers().create(request.clone())
        .await
        .unwrap();

    dec.container(&container.id).start()
        .await
        .unwrap();

    let mut tar_buffer = Vec::new();
    {
        let mut tar = tar::Builder::new(&mut tar_buffer);

        let greetings = "Hello, world.".as_bytes();

        let mut header = Header::new_gnu();
        header.set_path("greetings.txt").unwrap();
        header.set_size(greetings.len() as u64);
        header.set_cksum();

        tar.append(&header, greetings).unwrap();
        tar.finish().unwrap();
    }

    dec.container(&container.id).files().put("/var", Tar(tar_buffer))
        .await
        .unwrap();

    let tar_output = dec.container(&container.id).files().get("/var/greetings.txt")
        .await
        .unwrap()
        .0;

    let mut archive_reader = tar::Archive::new(tar_output.as_slice());
    {
        let mut entry = archive_reader
            .entries()
            .unwrap()
            .next()
            .unwrap()
            .unwrap();

        let p = entry.path().unwrap();

        assert_eq!("greetings.txt", p.to_str().unwrap());

        let mut content = Vec::new();
        entry.read_to_end(&mut content).unwrap();

        let message = String::from_utf8(content).unwrap();

        assert_eq!("Hello, world.", message);
    }

    dec.container(container.id).stop()
        .await
        .unwrap();
}