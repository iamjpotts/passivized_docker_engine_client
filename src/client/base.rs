use std::fmt::{Display, Formatter};

use hyper_tls::native_tls::TlsConnector;

use crate::client::{DecContainer, DecContainers, DecExec, DecImages, DecNetwork, DecNetworks, DecVolume, DecVolumes};
use crate::errors::{DecCreateError, DecUseError};
use crate::imp::api::{DockerEngineApi, DockerEngineServer, SchemedUrl};
use crate::imp::http_proxy::DockerEngineHttpClient;
use crate::imp::hyper_proxy::HyperHttpClient;
use crate::model::RegistryAuth;
use crate::responses::VersionResponse;

/// Docker Engine REST api version that this version of the Rust library uses when talking to Docker Engine.
pub const DOCKER_ENGINE_VERSION: &str = "v1.41";

/// Public interface for interacting with a Docker Engine API endpoint.
///
/// Construct using ::new to use your environment's default configuration.
///
/// # Example
///
/// ```rust
/// use passivized_docker_engine_client::DockerEngineClient;
/// use passivized_docker_engine_client::errors::DecError;
///
/// async fn example() -> Result<(), DecError> {
///     let dec = DockerEngineClient::new()?;
///     let listing = dec.images().list().await?;
///
///     println!("Found {} images.", listing.len());
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct DockerEngineClient {
    pub(super) http: DockerEngineHttpClient,
    pub(super) registry_auth: Option<RegistryAuth>,
    pub(super) url: DockerEngineApi
}

impl Display for DockerEngineClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl DockerEngineClient {

    /// Configure a client. No connection is made to server until first request.
    ///
    /// Because named pipes are not yet supported by this library, this method
    /// will fail on a default Windows configuration.
    ///
    /// On Windows, you need to reconfigure Docker Engine to listen on a TCP port,
    /// and then set the DOCKER_HOST environment variable.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecCreateError;
    ///
    /// fn example() -> Result<(), DecCreateError> {
    ///     let dec = DockerEngineClient::new()?;
    ///
    ///     println!("Will connect to {}", dec);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Result<DockerEngineClient, DecCreateError> {
        match crate::imp::env::docker_host() {
            Some(host) => {
                Self::with_server(host)
            },
            #[cfg(not(windows))]
            None => {
                Self::with_server(crate::imp::env::default_server())
            }
            #[cfg(windows)]
            None => {
                Err(DecCreateError::NamedPipesNotSupported)
            }
        }
    }

    /// Set the authentication credentials to use when pulling or pushing images on a
    /// Docker container registry that requires authentication.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    /// use passivized_docker_engine_client::model::RegistryAuth;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let credential = RegistryAuth {
    ///         username: "john".into(),
    ///         password: "Don't hard code your passwords".into(),
    ///         server: Some("registry.locallan".into()),
    ///         ..RegistryAuth::default()
    ///     };
    ///
    ///     let dec = DockerEngineClient::new()?
    ///         .with_registry_auth(credential);
    ///
    ///     dec.images().push("registry.locallan/foo", "latest").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_registry_auth(mut self, v: RegistryAuth) -> Self {
        self.registry_auth = Some(v);
        self
    }

    /// Connect to a specific Docker Engine.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::DockerEngineClient;
    /// use passivized_docker_engine_client::errors::DecError;
    ///
    /// async fn example() -> Result<(), DecError> {
    ///     let dec = DockerEngineClient::with_server("http://some-machine:1234")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_server<U: ToString>(uri_or_unix_socket: U) -> Result<DockerEngineClient, DecCreateError> {
        let server = DockerEngineServer::new(uri_or_unix_socket.to_string())?;

        let hyper_client = match server.base.implied_url() {
            SchemedUrl::Http(_) =>
                HyperHttpClient::http(),

            SchemedUrl::Https(_) => {
                let tls = TlsConnector::new()?;

                HyperHttpClient::https(tls)
            },

            #[cfg(unix)]
            SchemedUrl::Unix(_) =>
                HyperHttpClient::unix(),

        };

        let result = DockerEngineClient {
            http: DockerEngineHttpClient::new(hyper_client),
            registry_auth: None,
            url: DockerEngineApi::new(server)
        };

        Ok(result)
    }

    /// Connect to a TLS-secured Docker Engine, using a specific TLS configuration on the client.
    ///
    /// See test_registry.rs for a complex example.
    pub fn with_tls_config<U: ToString>(https_url: U, tls: TlsConnector) -> Result<DockerEngineClient, DecCreateError> {
        let result = DockerEngineClient {
            http: DockerEngineHttpClient::new(HyperHttpClient::https(tls)),
            registry_auth: None,
            url: DockerEngineApi::with_server(https_url.to_string())?
        };

        Ok(result)
    }

    /// Work with a specific existing container, referenced by its container ID or container name.
    pub fn container<C: Into<String>>(&'_ self, name_or_id: C) -> DecContainer<'_> {
        DecContainer {
            client: self,
            container_id: name_or_id.into()
        }
    }

    /// Work with containers as a group, or create a new container.
    pub fn containers(&'_ self) -> DecContainers<'_> {
        DecContainers {
            client: self
        }
    }

    /// Work with a specific existing container exec, referenced by its exec ID.
    pub fn exec<E: Into<String>>(&'_ self, id: E) -> DecExec<'_> {
        DecExec {
            client: self,
            exec_id: id.into()
        }
    }

    /// Work with images.
    pub fn images(&'_ self) -> DecImages<'_> {
        DecImages {
            client: self
        }
    }

    /// Work with a specific existing network.
    pub fn network<N: Into<String>>(&'_ self, id: N) -> DecNetwork<'_> {
        DecNetwork {
            client: self,
            network_id: id.into()
        }
    }

    /// Work with networks as a collection/group, or create a new network.
    pub fn networks(&'_ self) -> DecNetworks<'_> {
        DecNetworks {
            client: self
        }
    }

    pub async fn version(&self) -> Result<VersionResponse, DecUseError> {
        let url = self.url.version();

        self.http
            .get(url)?
            .execute()
            .await?
            .parse()
    }

    /// Work with a specific existing volume.
    pub fn volume<V: Into<String>>(&'_ self, id: V) -> DecVolume<'_> {
        DecVolume {
            client: self,
            volume_id: id.into()
        }
    }

    /// Work with volumes as a collection/group, or create a new volume.
    pub fn volumes(&'_ self) -> DecVolumes<'_> {
        DecVolumes {
            client: self
        }
    }

}

#[cfg(test)]
mod test_docker_engine_client {
    use hyper_tls::native_tls::TlsConnector;

    use crate::DockerEngineClient;

    #[test]
    fn display_http() {
        let dec = DockerEngineClient::with_server("http://foo")
            .unwrap();
        let actual = format!("{}", dec);

        assert_eq!("Docker engine at http://foo".to_string(), actual);
    }

    #[test]
    fn display_https() {
        let tls = TlsConnector::builder()
            .build()
            .unwrap();
        let dec = DockerEngineClient::with_tls_config("https://foo", tls)
            .unwrap();
        let actual = format!("{}", dec);

        assert_eq!("Docker engine at https://foo".to_string(), actual);
    }

    // Some Windows environments have tcp:// at the start of the URI instead of http:// - for those
    // environments, accept tcp:// as if it were http://, and display the original tcp:// URL.
    #[test]
    fn display_tcp() {
        let dec = DockerEngineClient::with_server("tcp://foo")
            .unwrap();
        let actual = format!("{}", dec);

        assert_eq!("Docker engine at tcp://foo".to_string(), actual);
    }

    #[test]
    #[cfg(unix)]
    fn display_unix() {
        let dec = DockerEngineClient::with_server("/var/run/docker.sock")
            .unwrap();
        let actual = format!("{}", dec);

        assert_eq!("Docker engine at /var/run/docker.sock".to_string(), actual);
    }

    #[test]
    #[cfg(unix)]
    fn display_unix_stripped() {
        let dec = DockerEngineClient::with_server("unix:///var/run/docker.sock")
            .unwrap();
        let actual = format!("{}", dec);

        assert_eq!("Docker engine at /var/run/docker.sock".to_string(), actual);
    }
}
