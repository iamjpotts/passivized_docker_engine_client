#[path = "test_utils/lib.rs"]
mod test_utils;

use std::sync::Arc;
use std::time::{Duration, Instant};

use test_utils::images::web;
use test_utils::random_name;
use tokio::sync::Semaphore;

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::requests::{CreateContainerRequest, ListContainersRequest, WaitCondition};

#[tokio::test]
async fn test_pull_create_list_rename_start_and_stop() {
    const FN: &str = "test_pull_create_list_rename_start_and_stop";

    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let name1 = random_name(FN);

    let request: CreateContainerRequest = CreateContainerRequest::default()
        .name(name1.clone())
        .image(format!("{}:{}", web::IMAGE, web::TAG));

    let container = dec.containers().create(request)
        .await
        .unwrap();

    let list_request = ListContainersRequest::default()
        .all(true);

    {
        let containers = dec.containers().list(list_request.clone())
            .await
            .unwrap();

        let found = containers
            .iter()
            .find(|c| c.id == container.id)
            .unwrap();

        assert!(found.names.contains( &format!("/{}", name1).to_string()));
    }

    let name2 = random_name(FN);

    dec.container(&container.id).rename(&name2)
        .await
        .unwrap();

    {
        let containers = dec.containers().list(list_request)
            .await
            .unwrap();

        let found = containers
            .iter()
            .find(|c| c.id == container.id)
            .unwrap();

        // Has only new name
        assert!(!found.names.contains( &format!("/{}", name1).to_string()), "name1 is gone");
        assert!(found.names.contains( &format!("/{}", name2).to_string()), "name2 is present");
    }

    dec.container(&container.id).start()
        .await
        .unwrap();

    {
        let inspect_response = dec.container(&container.id).inspect()
            .await
            .unwrap();

        assert!(inspect_response.state.running, "running");
        assert_eq!("running", inspect_response.state.status);
    }

    #[cfg(not(windows))]
    {
        dec.container(&container.id).pause()
            .await
            .unwrap();

        {
            let inspect_response = dec.container(&container.id).inspect()
                .await
                .unwrap();

            assert!(inspect_response.state.paused, "paused");
            assert!(inspect_response.state.running, "running");
            assert_eq!("paused", inspect_response.state.status);
        }

        dec.container(&container.id).unpause()
            .await
            .unwrap();

        {
            let inspect_response = dec.container(&container.id).inspect()
                .await
                .unwrap();

            assert!(!inspect_response.state.paused, "not paused");
            assert!(inspect_response.state.running, "running");
            assert_eq!("running", inspect_response.state.status);
        }
    }

    dec.container(&container.id).stop()
        .await
        .unwrap();

    {
        let inspect_response = dec.container(&container.id).inspect()
            .await
            .unwrap();

        assert!(!inspect_response.state.paused, "not paused");
        assert!(!inspect_response.state.running, "not running");
        assert_eq!("exited", inspect_response.state.status);
    }

    dec.container(container.id).remove()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_pull_start_and_kill() {
    const FN: &str = "test_pull_start_and_kill";

    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let request: CreateContainerRequest = CreateContainerRequest::default()
        .name(random_name(FN))
        .image(format!("{}:{}", web::IMAGE, web::TAG));

    let container = dec.containers().create(request)
        .await
        .unwrap();

    dec.container(&container.id).start()
        .await
        .unwrap();

    {
        let inspect_response = dec.container(&container.id).inspect()
            .await
            .unwrap();

        assert!(inspect_response.state.running, "running");
    }

    dec.container(&container.id).kill_with("SIGKILL")
        .await
        .unwrap();

    {
        let inspect_response = dec.container(&container.id).inspect()
            .await
            .unwrap();

        #[cfg(unix)]
        assert_eq!(137, inspect_response.state.exit_code);

        #[cfg(windows)]
        assert_eq!(3221225473, inspect_response.state.exit_code);

        assert!(!inspect_response.state.running, "not running");
        assert_eq!("exited", inspect_response.state.status);
    }

    dec.container(container.id).remove()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_wait_blocks_until_condition_met() {
    const FN: &str = "test_wait_blocks_until_condition_met";

    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let request: CreateContainerRequest = CreateContainerRequest::default()
        .name(random_name(FN))
        .image(format!("{}:{}", web::IMAGE, web::TAG));

    let container = dec.containers().create(request)
        .await
        .unwrap();

    dec.container(&container.id).start()
        .await
        .unwrap();

    {
        let inspect_response = dec.container(&container.id).inspect()
            .await
            .unwrap();

        assert!(inspect_response.state.running, "running");
    }

    let semaphore = Arc::new(Semaphore::new(1));
    let waited_container = container.id.clone();
    let wait_permit = semaphore
        .clone()
        .acquire_owned()
        .await
        .unwrap();

    let waiter = async move {
        println!("Setting flag to indicate wait future has started");
        drop(wait_permit);

        println!("Wait future is now waiting on container");
        let wdc = DockerEngineClient::new()
            .unwrap();

        let started_at = Instant::now();

        #[cfg(windows)]
        let condition = WaitCondition::NextExit;

        #[cfg(not(windows))]
        let condition = WaitCondition::NotRunning;

        wdc.container(waited_container).wait(condition)
            .await
            .unwrap();

        let ended_at = Instant::now();
        let elapsed = ended_at.duration_since(started_at);

        println!("Wait future is exiting");

        elapsed
    };

    println!("Starting wait future");
    let joiner = tokio::task::spawn(waiter);

    println!("Waiting for wait future to start");

    let _permit = semaphore.acquire_owned()
        .await
        .unwrap();

    println!("Wait future has started");

    println!("Sleeping");

    let sleep_for = Duration::from_secs(2);
    tokio::time::sleep(sleep_for.clone())
        .await;

    println!("Stopping container");
    // Will block until container exits
    dec.container(&container.id).stop()
        .await
        .unwrap();
    println!("Stopped container");

    println!("Joining wait future");
    let api_waited_for = joiner
        .await
        .unwrap();
    println!("Joined wait future");

    dec.container(container.id).remove()
        .await
        .unwrap();

    println!("Waited for {}s while sleeping for {}s", api_waited_for.as_secs(), sleep_for.as_secs());
    assert!(api_waited_for.ge(&sleep_for))
}
