use hyper_tls::native_tls;

/// An error during the creation of Docker Engine client.
#[derive(Debug, thiserror::Error)]
pub enum DecCreateError {

    #[error("Failed to create TlsConnector: {0}")]
    TlsConnector(native_tls::Error),

    #[error("Unsupported scheme in Docker Engine url")]
    UnsupportedUrlScheme,

    #[cfg(target_os = "windows")]
    #[error("On Windows, by default, Docker Engine only listens on a named pipe, but this library does not support named pipes. Reconfigure Docker Engine to listen on a TCP port, and then either set the DOCKER_HOST environment variable, or connect using DockerEngineClient::with_server.")]
    NamedPipesNotSupported,

    #[cfg(not(unix))]
    #[error("Use of Unix style socket URLs or paths such as {0} requires a Unix-like platform")]
    NixPlatformFeatureDisabled(String)

}

impl From<native_tls::Error> for DecCreateError {
    fn from(other: native_tls::Error) -> Self {
        DecCreateError::TlsConnector(other)
    }
}