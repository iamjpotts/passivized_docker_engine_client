#[path = "example_utils/lib.rs"]
mod example_utils;

use std::process::ExitCode;
use passivized_docker_engine_client::{DockerEngineClient};
use passivized_test_support::cli;
use log::*;
use example_utils::errors::ExampleError;

const PRINT_LIMIT: usize = 10;

#[tokio::main]
async fn main() -> ExitCode {
    cli::run(run).await
}

async fn run() -> Result<(), ExampleError> {
    let dec = DockerEngineClient::new()?;
    info!("Connecting to {}", dec);

    info!("Getting image list");

    let images = dec.images().list().await?;

    info!("Found {} images", images.len());

    let mut printed = 0;

    for (i, image) in images.iter().enumerate() {
        if image.repo_tags.iter().filter(|item| item.to_string() != "<none>:<none>".to_string()).count() > 0 {
            info!("Image {}: {:?}", i, image.id);

            for repo_tag in &image.repo_tags {
                info!("    {}", repo_tag);
            }

            printed = printed + 1;
        }

        if printed >= PRINT_LIMIT {
            break;
        }
    }

    Ok(())
}