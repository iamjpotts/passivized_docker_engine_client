//!
//! A library for interacting with the Docker Engine and its images, containers, and volumes.
//!
//! # Example
//!
//!```rust
//! use passivized_docker_engine_client::DockerEngineClient;
//! use passivized_docker_engine_client::errors::DecError;
//! use passivized_docker_engine_client::requests::CreateContainerRequest;
//!
//! async fn example() -> Result<(), DecError> {
//!     let dec = DockerEngineClient::new()?;
//!
//!     let create_request = CreateContainerRequest::default()
//!        .name("example")
//!        .image("nginx:latest");
//!
//!     let container = dec.containers().create(create_request).await?;
//!
//!     dec.container(&container.id).start().await?;
//!     dec.container(&container.id).stop().await?;
//!     dec.container(&container.id).remove().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod errors;
pub mod model;
pub mod requests;
pub mod responses;

pub use client::DockerEngineClient;

// Internal use only
mod imp;
