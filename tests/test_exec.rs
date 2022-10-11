#[path = "test_utils/lib.rs"]
mod test_utils;

use test_utils::images::web;
use test_utils::random_name;

use passivized_docker_engine_client::DockerEngineClient;
use passivized_docker_engine_client::model::{StreamKind, StreamLine};
use passivized_docker_engine_client::requests::{CreateContainerRequest, CreateExecRequest, ExecStartRequest, HostConfig};

#[tokio::test]
async fn test_exec_and_get_output() {
    const FN: &str = "test_exec_and_get_output";

    let dec = DockerEngineClient::new()
        .unwrap();

    dec.images().pull_if_not_present(web::IMAGE, web::TAG)
        .await
        .unwrap();

    let request: CreateContainerRequest = CreateContainerRequest::default()
        .name(random_name(FN))
        .image(format!("{}:{}", web::IMAGE, web::TAG))
        .host_config(HostConfig::default()
            .auto_remove()
        );

    let container = dec.containers().create(request)
        .await
        .unwrap();

    dec.container(&container.id).start()
        .await
        .unwrap();

    #[cfg(windows)]
    let cmd = ["cmd", "/C", "echo", "Hello,", "world."];

    #[cfg(not(windows))]
    let cmd = ["echo", "Hello,", "world."];

    let exec_request = CreateExecRequest::default()
        .cmd(Vec::from(cmd))
        .attach_stdout(true);

    let exec = dec.container(&container.id).create_exec(exec_request)
        .await
        .unwrap();

    let exec_start_request = ExecStartRequest {
        detach: false,
        ..ExecStartRequest::default()
    };

    let output = dec.exec(&exec.id).start(exec_start_request)
        .await
        .unwrap();

    let inspected_exec = dec.exec(exec.id).inspect()
        .await
        .unwrap();

    dec.container(container.id).stop()
        .await
        .unwrap();

    // Don't need to remove because auto-remove was set

    #[cfg(windows)]
    const EXPECTED_TEXT: &str = "Hello, world.\r\n";

    #[cfg(not(windows))]
    const EXPECTED_TEXT: &str = "Hello, world.\n";

    assert_eq!(1, output.len());
    assert_eq!(
        Some(
            &StreamLine {
                kind: StreamKind::StdOut,
                text: EXPECTED_TEXT.into(),
            }
        ),
        output.get(0)
    );

    assert!(
        inspected_exec.running || inspected_exec.exit_code == 0,
        "Expected either running or a zero exit code, but was: (running: {}; exit code: {})",
        inspected_exec.running,
        inspected_exec.exit_code
    );
}
