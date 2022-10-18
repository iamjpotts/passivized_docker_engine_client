#[path = "example_utils/lib.rs"]
mod example_utils;

use std::process::ExitCode;
use log::*;

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::requests::CreateContainerRequest;
use passivized_test_support::cli;
use example_utils::errors::ExampleError;

const IMAGE_NAME: &str = "nginx";
const IMAGE_TAG: &str = "1.20";

#[tokio::main]
async fn main() -> ExitCode {
    cli::run(run).await
}

async fn run() -> Result<(), ExampleError> {
    let dec = DockerEngineClient::new()?;
    info!("Connecting to {}", dec);

    info!("Pulling image");

    let pull_result = dec.images().pull_if_not_present(IMAGE_NAME, IMAGE_TAG)
        .await?;

    info!("Pull result: {}", pull_result);

    let create_request = CreateContainerRequest::default()
        .image(format!("{}:{}", IMAGE_NAME, IMAGE_TAG));

    info!("Creating container");

    let container = dec.containers().create(create_request)
        .await?;

    info!("Created container with id {}", container.id);
    for w in &container.warnings {
        info!("Container warning: {}", w)
    }

    dec.container(&container.id).rename("test-nginx")
        .await?;

    dec.container(&container.id).start()
        .await?;

    let inspected = dec.container(&container.id).inspect()
        .await?;

    let ip = inspected.first_ip_address()
        .ok_or(ExampleError::NoIp())?;

    let url = format!("http://{}", ip);

    let response = example_utils::retry::wait_for_http_server(&url)
        .await?;

    info!("{}", response);

    info!("Stopping container {}", container.id);

    dec.container(&container.id).stop()
        .await?;

    info!("Removing container {}", container.id);

    dec.container(container.id).remove()
        .await?;

    Ok(())
}
