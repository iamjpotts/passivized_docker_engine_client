use std::fmt::{Display, Formatter};

use hyper::StatusCode;

use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::imp::content_type;
use crate::model::Tar;
use crate::requests::{BuildImageRequest, CreateImageRequest};
use crate::responses::{BuildImageResponseStreamItem, ListedImage};

pub struct DecImages<'a> {
    pub(super) client: &'a DockerEngineClient
}

impl <'a> DecImages<'a> {

    /// Build a new image.
    ///
    /// Context is a tar file that includes a Dockerfile and any files referenced by it.
    ///
    /// See https://docs.docker.com/engine/api/v1.41/#tag/Image/operation/ImageBuild
    ///
    /// Failure can be returned two ways:
    ///     1. Via Result::Err
    ///     2. Via Result::Ok with stream items containing error messages
    pub async fn build(&self, request: BuildImageRequest, context: Tar) -> Result<Vec<BuildImageResponseStreamItem>, DecUseError> {
        let uri = self.client.url.images().build(request)?;
        let response = self.client.http
            .post_with_auth_config(
                uri,
                &self.client.registry_auth,
                content_type::TAR,
                context.0
            )?
            .execute()
            .await?;

        let items = response
            .assert_item_status(StatusCode::OK)?
            .parse_stream()?;

        Ok(items)
    }

    /// "Create an image by either pulling it from a registry or importing it."
    ///
    /// See https://docs.docker.com/engine/api/v1.41/#tag/Image/operation/ImageCreate
    ///
    /// Establish a container image in the Docker Engine. The most common action
    /// is to cause an image to be pulled. Note that in this case, in spite of the
    /// api name, the image is not being created, but rather copied from a registry.
    ///
    /// For clarity, a ::pull method is also offered, and is a shortcut to calling
    /// this method.
    pub async fn create(&self, request: CreateImageRequest) -> Result<(), DecUseError> {
        let uri = self.client.url.images().create(request)?;
        let response = self.client.http.post_with_auth(uri, &self.client.registry_auth)?.execute().await?;

        // Pull responses return a JSON stream with status messages reporting progress, but we ignore the stream and wait for completion.
        response
            .assert_unit_status(StatusCode::OK)
    }

    /// Get a list of the images stored by the Docker engine.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///     let list = dec.images().list().await?;
    ///
    ///     for image in list {
    ///         println!("Image id {}: {:?}", image.id, image.repo_tags);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self) -> Result<Vec<ListedImage>, DecUseError> {
        let uri = self.client.url.images().list();
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_list_status(StatusCode::OK)?
            .parse()
    }

    /// Pull an image. If the image already exists, pull it again, if the image on the remote server
    /// is different than the local image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example_docker_hub() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     dec.images().pull("nginx", "latest").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    /// use passivized_docker_engine_client::model::RegistryAuth;
    ///
    /// async fn example_private_registry() -> Result<(), DecError> {
    ///     let credential = RegistryAuth {
    ///         username: "mary".into(),
    ///         password: "Don't hard code your passwords".into(),
    ///         server: Some("registry.locallan".into()),
    ///         ..RegistryAuth::default()
    ///     };
    ///
    ///     let dec = DockerEngineClient::new()?
    ///         .with_registry_auth(credential);
    ///
    ///     dec.images().pull("registry.locallan/corporate-app", "2.0").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn pull<R: Into<String>, T: Into<String>>(&self, repo: R, tag: T) -> Result<(), DecUseError> {
        let request = CreateImageRequest::default()
            .image(repo)
            .tag(tag);

        self.create(request).await
    }

    /// If an image does not exist in the Docker Engine, pull it; but if it
    /// already exists, do nothing.
    ///
    /// This is a convenience/wrapper method over multiple api calls.
    pub async fn pull_if_not_present<R: Into<String>, T: Into<String>>(&self, repo: R, tag: T) -> Result<DecImagesPullIfNotPresentResult, DecUseError> {
        let image_repo = repo.into();
        let image_tag = tag.into();
        let sought = format!("{}:{}", image_repo, image_tag);

        let present = self.list().await?;

        let found = present
            .iter()
            .any(|item| item.repo_tags.contains(&sought));

        if found {
           Ok(DecImagesPullIfNotPresentResult::AlreadyPresent)
        }
        else {
            self.pull(image_repo, image_tag).await?;

            Ok(DecImagesPullIfNotPresentResult::Pulled)
        }
    }

    /// Copy an image from the Docker Engine to a Docker image registry.
    pub async fn push<R: Into<String>, T: Into<String>>(&self, repo: R, tag: T) -> Result<(), DecUseError> {
        let uri = self.client.url.images().push(repo, tag);
        let response = self.client.http.post_with_auth(uri, &self.client.registry_auth)?.execute().await?;

        response
            .assert_unit_status(StatusCode::OK)
    }

    /// Tag an image that exists in the Docker Engine with an additional name or tag.
    pub async fn tag<ID: Into<String>, R: Into<String>, T: Into<String>>(&self, image_id_or_name: ID, new_repo: R, new_tag: T) -> Result<(), DecUseError> {
        let uri = self.client.url.images().tag(image_id_or_name, new_repo, new_tag);
        let response = self.client.http.post(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::CREATED)
    }

    /// Remove a name or tag from an image.
    ///
    /// The image will still exist even after the last tag is removed, can be
    /// returned by the list() method, and can be referenced by its hash.
    pub async fn untag<ID: Into<String>>(&self, image_id_or_name_and_tag: ID) -> Result<(), DecUseError> {
        let uri = self.client.url.images().untag(image_id_or_name_and_tag);
        let response = self.client.http.delete(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::OK)
    }
}

pub enum DecImagesPullIfNotPresentResult {
    /// A pull was requested for a specific image tag, but an image with that tag already exists in the Docker Engine.
    AlreadyPresent,

    /// A pull was requested, and the image was copied to the Docker Engine from a remote image registry.
    Pulled
}

impl Display for DecImagesPullIfNotPresentResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::AlreadyPresent => "already present",
            Self::Pulled => "pulled"
        };

        write!(f, "{}", message)
    }
}