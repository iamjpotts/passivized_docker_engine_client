#[path = "example_utils/lib.rs"]
mod example_utils;

use std::iter::FromIterator;
use std::process::ExitCode;
use log::*;

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::requests::{CreateContainerRequest, WaitCondition};
use passivized_test_support::cli;
use example_utils::errors::ExampleError;

const IMAGE_NAME: &str = "busybox";

#[tokio::main]
async fn main() -> ExitCode {
    cli::run(run).await
}

/// Run the BusyBox container with a file listing command, and get the file listing.
async fn run() -> Result<(), ExampleError> {
    let dec = DockerEngineClient::new()?;
    info!("Connecting to {}", dec);

    info!("Pulling image");

    let pull_result = dec.images().pull_if_not_present(IMAGE_NAME, "latest")
        .await?;

    info!("Pull result: {}", pull_result);

    let create_request = CreateContainerRequest::default()
        .image(IMAGE_NAME)
        .cmd(vec!["ls", "-l"]);

    info!("Creating container");

    let container = dec.containers().create(create_request)
        .await?;

    info!("Created container with id {}", container.id);
    for w in &container.warnings {
        info!("Container warning: {}", w)
    }

    dec.container(&container.id).start()
        .await?;

    info!("Waiting for container {} to stop", container.id);

    dec.container(&container.id).wait(WaitCondition::NotRunning)
        .await?;

    info!("Getting logs of {}", container.id);

    let logs = dec.container(&container.id).logs()
        .await?;

    let log = String::from_iter(logs.iter().map(|entry| entry.text.clone()));
    info!("\n{}", log);

    info!("Removing container {}", container.id);

    dec.container(&container.id).remove()
        .await?;

    Ok(())
}
