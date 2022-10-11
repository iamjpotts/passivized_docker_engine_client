use hyper::StatusCode;

use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::requests::CreateVolumeRequest;
use crate::responses::{ListVolumesResponse, PruneVolumesResponse};

pub struct DecVolumes<'a> {
    pub(super) client: &'a DockerEngineClient
}

impl <'a> DecVolumes<'a> {

    /// Create a new volume.
    ///
    /// # Arguments
    /// * `request` - description of volume to create
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    /// use passivized_docker_engine_client::requests::CreateVolumeRequest;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     let request = CreateVolumeRequest::default()
    ///         .name("example");
    ///
    ///     dec.volumes().create(request).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, request: CreateVolumeRequest) -> Result<(), DecUseError> {
        let uri = self.client.url.volumes().create();
        let response = self.client.http.post_json(uri, &request)?.execute().await?;

        response
            .assert_unit_status(StatusCode::CREATED)
    }

    /// Get a list of volumes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///     let response = dec.volumes().list().await?;
    ///
    ///     println!("Found {} volumes.", response.volumes.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self) -> Result<ListVolumesResponse, DecUseError> {
        let uri = self.client.url.volumes().list();
        let response = self.client.http.get(&uri)?.execute().await?;

        response
            .assert_list_status(StatusCode::OK)?
            .parse()
    }

    /// Remove volumes not associated with a container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///     let response = dec.volumes().prune().await?;
    ///
    ///     println!("Pruned {} volumes.", response.volumes_deleted.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn prune(&self) -> Result<PruneVolumesResponse, DecUseError> {
        let uri = self.client.url.volumes().prune();
        let response = self.client.http.post(&uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse()
    }

}