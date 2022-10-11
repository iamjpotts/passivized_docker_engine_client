#[path = "test_utils/lib.rs"]
mod test_utils;

use std::time::Duration;

use test_utils::images::web;
use test_utils::random_name;

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::errors::DecUseError;
use passivized_docker_engine_client::model::HealthCheck;
use passivized_docker_engine_client::requests::{CreateContainerRequest, HostConfig};
use passivized_docker_engine_client::responses::Health;

#[tokio::test]
async fn test_configure_and_inspect_failing_health_check() {
    const FN: &str = "test_configure_and_inspect_failing_health_check";

    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let create_request = CreateContainerRequest::default()
        .name(random_name(FN))
        .image(format!("{}:{}", web::IMAGE, web::TAG))
        .host_config(HostConfig::default()
            .auto_remove()
        )
        .health_check(HealthCheck::default()
            .test(vec!["CMD", "does_not_exist"])
            .start_period(Duration::from_secs(2))
            .interval(Duration::from_secs(1))
            .retries(3)
        );

    let container = dec.containers().create(create_request)
        .await
        .unwrap();

    dec.container(&container.id).start()
        .await
        .unwrap();

    {
        let inspected = dec.container(&container.id).inspect()
            .await
            .unwrap();

        // A container with passing health checks will have a "running" status. If the health checks fail right away,
        // (which is typical during a normal startup), the container will have a "starting" status, and eventually
        // it will have an "unhealthy" status.

        let health = inspected.state.health.unwrap();
        assert_eq!("starting".to_string(), health.status);
    }

    // Wait for the unhealthy status
    let inspected_health = wait_for_not_starting(&dec, &container.id)
        .await
        .unwrap();

    assert_eq!("unhealthy".to_string(), inspected_health.status);
    assert!(inspected_health.failing_streak > 0);
    assert!(inspected_health.log.len() > 0);
    println!("Log: {:?}", inspected_health.log);

    #[cfg(windows)]
    const EXPECTED_CONTENT: &str = "The system cannot find the file specified. (0x2)";

    #[cfg(not(windows))]
    const EXPECTED_CONTENT: &str = "does_not_exist";

    let output = inspected_health.log
        .get(0)
        .unwrap()
        .output
        .as_str();

    assert!(
        output.contains(EXPECTED_CONTENT),
        "The text '{}' is contained in output: {}",
        EXPECTED_CONTENT,
        output
    );

    dec.container(&container.id).stop()
        .await
        .unwrap();
}

async fn wait_for_not_starting(dc: &DockerEngineClient, container_id: &str) -> Result<Health, DecUseError> {
    const RETRY_DELAY: Duration = Duration::from_secs(3);

    let mut attempts_remaining: usize = 5;

    let result = loop {
        let inspected = dc.container(container_id).inspect()
            .await?;

        let health = inspected.state.health.unwrap();

        if health.status != "starting".to_string() {
            break health;
        }

        attempts_remaining -= 1;

        if attempts_remaining == 0 {
            break health
        }

        tokio::time::sleep(RETRY_DELAY).await;
    };

    Ok(result)
}
