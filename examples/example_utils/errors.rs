use std::string::FromUtf8Error;
use hyper_tls::native_tls;

use passivized_docker_engine_client::errors::{DecCreateError, DecUseError};

#[derive(Debug, thiserror::Error)]
pub enum ExampleError {
    #[error("Failed to create Docker engine client: {0}")]
    DockerEngineClientCreate(DecCreateError),

    #[error("Docker engine client error: {0}")]
    DockerEngineClient(DecUseError),

    #[error("I/O error: {0}")]
    Io(std::io::Error),

    #[error("Hyper error: {0}")]
    Hyper(hyper::Error),

    #[error("Hyper http error: {0}")]
    HyperHttp(hyper::http::Error),

    #[error("Did not find an IP address")]
    NoIp(),

    #[error("TLS error: {0}")]
    Tls(native_tls::Error),

    #[error("UTF8 parse error: {0}")]
    Utf8(FromUtf8Error)
}

impl From<DecCreateError> for ExampleError {
    fn from(other: DecCreateError) -> Self {
        Self::DockerEngineClientCreate(other)
    }
}

impl From<DecUseError> for ExampleError {
    fn from(other: DecUseError) -> Self {
        Self::DockerEngineClient(other)
    }
}

impl From<hyper::Error> for ExampleError {
    fn from(other: hyper::Error) -> Self {
        Self::Hyper(other)
    }
}

impl From<hyper::http::Error> for ExampleError {
    fn from(other: hyper::http::Error) -> Self {
        Self::HyperHttp(other)
    }
}

impl From<FromUtf8Error> for ExampleError {
    fn from(other: FromUtf8Error) -> Self {
        Self::Utf8(other)
    }
}

impl From<std::io::Error> for ExampleError {
    fn from(other: std::io::Error) -> Self {
        Self::Io(other)
    }
}

impl From<native_tls::Error> for ExampleError {
    fn from(other: native_tls::Error) -> Self {
        Self::Tls(other)
    }
}