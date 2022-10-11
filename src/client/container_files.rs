use hyper::StatusCode;

use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::imp::content_type;
use crate::model::Tar;
use crate::responses::FileSystemChange;

pub struct DecContainerFiles<'a> {
    pub(super) client: &'a DockerEngineClient,
    pub(super) container_id: &'a String
}

impl <'a> DecContainerFiles<'a> {

    /// Get a list of changes to a container's file system.
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
    ///     let changes = dec.container("example").files().changes().await?;
    ///
    ///     for change in changes {
    ///         println!("{:?}", change);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn changes(&self) -> Result<Vec<FileSystemChange>, DecUseError> {
        let uri = self.client.url.containers().files(self.container_id).changes();
        let response = self.client.http.get(uri)?.execute().await?;

        let result: Option<Vec<FileSystemChange>> = response
            .assert_item_status(StatusCode::OK)?
            .parse()?;

        Ok(result.unwrap_or_default())
    }

    /// Get files from inside the container, described by the path, and assemble them into a tar file.
    pub async fn get<P: Into<String>>(&self, path: P) -> Result<Tar, DecUseError> {
        let uri = self.client.url.containers().files(self.container_id).get(path);
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .assert_content_type(content_type::TAR)?
            .parse_with(|r| Ok(Tar(r.body.to_vec())))
    }

}
