use hyper::StatusCode;

use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::model::Volume;

pub struct DecVolume<'a> {
    pub(super) client: &'a DockerEngineClient,
    pub(super) volume_id: String
}

impl <'a> DecVolume<'a> {

    /// Get description of an existing volume.
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
    ///     let volume = dec.volume("example").inspect().await?;
    ///
    ///     println!("Driver: {}", volume.driver);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn inspect(&self) -> Result<Volume, DecUseError> {
        let uri = self.client.url.volumes().inspect(&self.volume_id);
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse()
    }

    /// Remove an existing volume.
    ///
    /// # Arguments
    /// * `force` - Remove volume even if it is in use
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
    ///     dec.volume("example").remove(false).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn remove(&self, force: bool) -> Result<(), DecUseError> {
        let uri = self.client.url.volumes().remove(&self.volume_id, force);
        let response = self.client.http.delete(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::NO_CONTENT)
    }

}