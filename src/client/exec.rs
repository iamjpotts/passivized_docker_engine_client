use hyper::StatusCode;

use crate::client::shared::parse_container_log;
use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::model::StreamLine;
use crate::requests::ExecStartRequest;
use crate::responses::ExecInspectResponse;

pub struct DecExec<'a> {
    pub(super) client: &'a DockerEngineClient,
    pub(super) exec_id: String
}

impl <'a> DecExec<'a> {

    /// Get a description of an existing container command. The command does not
    /// need to be running; it only needs to be defined.
    pub async fn inspect(&self) -> Result<ExecInspectResponse, DecUseError> {
        let uri = self.client.url.exec().inspect(&self.exec_id);
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse()
    }

    // Attached: block and wait for command to exit, and return its stdout and/or stderr if
    // attachment to those was requested.
    //
    // Detached: start command and return immediately. Does not return any stdout or stderr
    pub async fn start(&self, request: ExecStartRequest) -> Result<Vec<StreamLine>, DecUseError> {
        assert!(!request.tty, "Attaching with a TTY is not currently supported by the Rust library.");

        let uri = self.client.url.exec().start(&self.exec_id);
        let response = self.client.http.post_json(uri, &request)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse_with(parse_container_log)
    }

}


#[cfg(test)]
mod tests {

    mod start {
        use const_str::concat;
        use http::StatusCode;
        use serial_test::serial;

        use crate::DockerEngineClient;
        use crate::errors::DecUseError;
        use crate::imp::api::DOCKER_ENGINE_VERSION_PATH;
        use crate::imp::content_type;
        use crate::requests::ExecStartRequest;

        fn mockito_client() -> DockerEngineClient {
            DockerEngineClient::with_server(format!("http://{}", mockito::server_address()))
                .unwrap()
        }

        #[tokio::test]
        #[serial]
        async fn response_has_wrong_content_type() {
            let dec = mockito_client();

            mockito::reset();

            let _m = mockito::mock("POST", concat!(DOCKER_ENGINE_VERSION_PATH, "/exec/some_exec_id/start"))
                .with_status(200)
                .with_header("Content-Type", content_type::JSON)
                .create();

            let exec_start_request = ExecStartRequest {
                detach: false,
                ..ExecStartRequest::default()
            };

            let error = dec.exec("some_exec_id").start(exec_start_request)
                .await
                .unwrap_err();

            if let DecUseError::UnexpectedResponseContentType { expected, actual } = error {
                assert_eq!(content_type::STREAM, expected);
                assert_eq!(Some(content_type::JSON.to_string()), actual);
            }
            else {
                panic!("Unexpected failure: {}", error);
            }
        }

        #[tokio::test]
        #[serial]
        async fn response_has_wrong_status() {
            let dec = mockito_client();

            mockito::reset();

            let _m = mockito::mock("POST", "/exec/some_exec_id/start")
                .with_status(StatusCode::NOT_IMPLEMENTED.as_u16() as usize)
                .with_header("Content-Type", content_type::JSON)
                .create();

            let exec_start_request = ExecStartRequest {
                detach: false,
                ..ExecStartRequest::default()
            };

            let error = dec.exec("some_exec_id").start(exec_start_request)
                .await
                .unwrap_err();

            if let DecUseError::ApiNotImplemented { uri} = error {
                assert!(uri.ends_with("/exec/some_exec_id/start"));
            }
            else {
                panic!("Unexpected failure: {}", error);
            }
        }

    }
}
