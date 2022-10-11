use hyper::StatusCode;

use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::requests::{CreateContainerRequest, ListContainersRequest};
use crate::responses::{CreateContainerResponse, ListedContainer};

pub struct DecContainers<'a> {
    pub(super) client: &'a DockerEngineClient
}

impl <'a> DecContainers<'a> {

    /// Create a container and return its ID.
    ///
    /// # Arguments
    /// * `request` describes the container to create.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     let request = CreateContainerRequest::default()
    ///         .name("example")
    ///         .image("nginx");
    ///
    ///     let container = dec.containers().create(request).await?;
    ///
    ///     println!("Created container with id {}.", container.id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, request: CreateContainerRequest) -> Result<CreateContainerResponse, DecUseError> {
        let uri = self.client.url.containers().create(request.name.as_ref());
        let response = self.client.http.post_json(uri, &request)?.execute().await?;

        response
            .assert_item_status(StatusCode::CREATED)?
            .parse()
    }

    /// Get a list of containers that meet the filter criteria.
    pub async fn list(&self, request: ListContainersRequest) -> Result<Vec<ListedContainer>, DecUseError> {
        let uri = self.client.url.containers().list(request)?;
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_list_status(StatusCode::OK)?
            .parse()
    }

}