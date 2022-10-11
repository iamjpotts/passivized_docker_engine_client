use hyper::StatusCode;

use crate::client::container_files::DecContainerFiles;
use crate::client::shared::parse_container_log;
use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::requests::{CreateExecRequest, InspectContainerArgs, RemoveContainerArgs, WaitCondition};
use crate::responses::{CreateExecResponse, InspectContainerResponse, TopResponse, WaitResponse};
use crate::model::StreamLine;

pub struct DecContainer<'a> {
    pub(super) client: &'a DockerEngineClient,
    pub(super) container_id: String
}

impl <'a> DecContainer<'a> {

    /// Create a command to run within a container, but don't start or execute the command.
    pub async fn create_exec(&self, request: CreateExecRequest) -> Result<CreateExecResponse, DecUseError> {
        let uri = self.client.url.containers().create_exec(&self.container_id);
        let response = self.client.http.post_json(uri, &request)?.execute().await?;

        response
            .assert_item_status(StatusCode::CREATED)?
            .parse()
    }

    /// Work with files inside a container.
    pub fn files(&'_ self) -> DecContainerFiles<'_> {
        DecContainerFiles {
            client: self.client,
            container_id: &self.container_id
        }
    }

    /// Get the console output (stdout and/or stderr) of a container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///     let log = dec.container("example").logs().await?;
    ///
    ///     println!("Container log:");
    ///     for line in log {
    ///         println!("{}", line.text);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn logs(&self) -> Result<Vec<StreamLine>, DecUseError> {
        let uri = self.client.url.containers().logs(&self.container_id);
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse_with(parse_container_log)
    }

    /// Inspect a container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///     let container = dec.container("example").inspect().await?;
    ///
    ///     println!("Container status: {}", container.state.status);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn inspect(&self) -> Result<InspectContainerResponse, DecUseError> {
        self.inspect_with(InspectContainerArgs::default()).await
    }

    /// Inspect a container, with additional request arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::requests::InspectContainerArgs;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///     let container = dec.container("example")
    ///         .inspect_with(InspectContainerArgs::default().size(true))
    ///         .await?;
    ///
    ///     let container_size = container.size_root_fs
    ///         .map(|size| size.to_string())
    ///         .unwrap_or("unknown".into());
    ///
    ///     println!("Container status: {}", container.state.status);
    ///     println!("Container root file system size: {}", container_size);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn inspect_with(&self, args: InspectContainerArgs) -> Result<InspectContainerResponse, DecUseError> {
        let uri = self.client.url.containers().inspect(&self.container_id, args.size.unwrap_or_default());
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse()
    }

    /// Abort a running container, using the default process signal (as determined
    /// by the Docker Engine and/or the container's configuration or the configuration
    /// of the image the container is based on).
    pub async fn kill<S: Into<String>>(&self) -> Result<(), DecUseError> {
        self.kill_opt(None).await
    }

    /// Send a signal to a container.
    ///
    /// Multiple signals are supported, not just SIGKILL.
    ///
    /// # Arguments
    /// * `signal` - name of signal to send
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.container("example").kill_with("SIGTERM").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn kill_with<S: Into<String>>(&self, signal: S) -> Result<(), DecUseError> {
        self.kill_opt(Some(signal.into())).await
    }

    async fn kill_opt(&self, signal: Option<String>) -> Result<(), DecUseError> {
        let uri = self.client.url.containers().kill(&self.container_id, signal)?;
        let response = self.client.http.post(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::NO_CONTENT)
    }

    /// Pause a running container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.container("example").pause().await?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(not(windows))]  // Docker for Windows does not support pausing containers.
    pub async fn pause(&self) -> Result<(), DecUseError> {
        let uri = self.client.url.containers().pause(&self.container_id);
        let response = self.client.http.post(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::NO_CONTENT)
    }

    /// Delete a stopped container.
    ///
    /// Remove requests are NOT idempotent.
    pub async fn remove(&self) -> Result<(), DecUseError> {
        self.remove_with(RemoveContainerArgs::default()).await
    }

    /// Delete a container.
    ///
    /// Remove requests are NOT idempotent; attempting to remove a removed container will return an error.
    ///
    /// # Arguments
    /// * `args` are additional request arguments for the removal.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    /// use passivized_docker_engine_client::requests::RemoveContainerArgs;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     let args = RemoveContainerArgs::default()
    ///         .remove_volumes(true);
    ///
    ///     dec.container("example").remove_with(args).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn remove_with(&self, args: RemoveContainerArgs) -> Result<(), DecUseError> {
        let uri = self.client.url.containers().remove(&self.container_id, args)?;
        let response = self.client.http.delete(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::NO_CONTENT)
    }

    /// Rename an existing container.
    ///
    /// # Arguments
    /// * `new_name` - New name for the container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.container("example").rename("example2").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn rename<NN: Into<String>>(&self, new_name: NN) -> Result<(), DecUseError> {
        let uri = self.client.url.containers().rename(&self.container_id, new_name.into());
        let response = self.client.http.post(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::NO_CONTENT)
    }

    /// Start an existing container.
    ///
    /// This is idempotent.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.container("example").start().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(&self) -> Result<(), DecUseError> {
        let uri = self.client.url.containers().start(&self.container_id);
        let response = self.client.http.post(uri)?.execute().await?;

        response
            // See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerStart
            .assert_unit_status_in(&[
                // No error
                StatusCode::NO_CONTENT,
                // Already started
                StatusCode::NOT_MODIFIED
            ])
    }

    /// Stop a running container.
    ///
    /// This is idempotent.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.container("example").stop().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn stop(&self) -> Result<(), DecUseError> {
        let uri = self.client.url.containers().stop(&self.container_id);
        let response = self.client.http.post(uri)?.execute().await?;

        response
            // See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerStop
            .assert_unit_status_in(&[
                // No error
                StatusCode::NO_CONTENT,
                // Already stopped
                StatusCode::NOT_MODIFIED
            ])
    }

    /// Get a list of processes running inside the container.
    pub async fn top(&self) -> Result<TopResponse, DecUseError> {
        self.top_opt(None).await
    }

    /// Get a list of processes running inside the container, customizing the output.
    pub async fn top_with(&self, ps_args: String) -> Result<TopResponse, DecUseError> {
        self.top_opt(Some(ps_args)).await
    }

    async fn top_opt(&self, ps_args: Option<String>) -> Result<TopResponse, DecUseError> {
        let uri = self.client.url.containers().top(&self.container_id, ps_args)?;
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse()
    }

    /// Unpause a paused container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.container("example").unpause().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg(not(windows))]  // Docker for Windows does not support pausing containers.
    pub async fn unpause(&self) -> Result<(), DecUseError> {
        let uri = self.client.url.containers().unpause(&self.container_id);
        let response = self.client.http.post(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::NO_CONTENT)
    }

    /// Wait for a container to reach a specific state.
    ///
    /// For process stop states, blocks until the container stops, then returns the exit code.
    ///
    /// # Arguments
    /// `condition` - Desired condition to wait for.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    /// use passivized_docker_engine_client::requests::WaitCondition;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.container("example").wait(WaitCondition::NotRunning).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn wait(&self, condition: WaitCondition) -> Result<WaitResponse, DecUseError> {
        let uri = self.client.url.containers().wait(&self.container_id, condition);
        let response = self.client.http.post(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse()
    }

}