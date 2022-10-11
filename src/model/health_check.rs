//! # Example
//!
//! ```rust
//! use std::time::Duration;
//! use passivized_docker_engine_client::DockerEngineClient;
//! use passivized_docker_engine_client::errors::DecError;
//! use passivized_docker_engine_client::model::HealthCheck;
//! use passivized_docker_engine_client::requests::CreateContainerRequest;
//!
//! async fn example() -> Result<(), DecError> {
//!     let dec = DockerEngineClient::new()?;
//!
//!     let request = CreateContainerRequest::default()
//!         .image("nginx")
//!         .health_check(HealthCheck::default()
//!             // Creates a health check that will fail because the command will not be found in $PATH
//!             .test(vec!["CMD", "does_not_exist"])
//!             .start_period(Duration::from_secs(1))
//!             .interval(Duration::from_secs(2))
//!             .retries(3)
//!         );
//!
//!     dec.containers().create(request)
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthCheck {

    #[serde(rename = "Test", skip_serializing_if = "Vec::is_empty")]
    test: Vec<String>,

    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    interval_nanos: Option<u128>,

    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    timeout_nanos: Option<u128>,

    #[serde(rename = "Retries", skip_serializing_if = "Option::is_none")]
    retries: Option<u64>,

    #[serde(rename = "StartPeriod", skip_serializing_if = "Option::is_none")]
    start_period_nanos: Option<u128>
}

impl HealthCheck {

    /// Define a health check command. Note that the first value in the vector has a special meaning
    /// and is defined in the Docker Engine REST API documentation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::model::HealthCheck;
    ///
    /// let check = HealthCheck::default()
    ///     .test(vec!["CMD", "exit 1"]);
    /// ```
    pub fn test<V>(mut self, v: Vec<V>) -> Self
        where V: ToString
    {
        self.test = v
            .iter()
            .map(|item| item.to_string())
            .collect();

        self
    }

    /// Set how often the health check should be checked, using a domain type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use passivized_docker_engine_client::model::HealthCheck;
    ///
    /// let check = HealthCheck::default()
    ///     .interval(Duration::from_secs(3));
    /// ```
    pub fn interval(mut self, v: Duration) -> Self {
        self.interval_nanos = Some(v.as_nanos());
        self
    }

    /// Set how long a check should be allowed to run before it is considered timed out.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use passivized_docker_engine_client::model::HealthCheck;
    ///
    /// let check = HealthCheck::default()
    ///     .timeout(Duration::from_secs(30));
    /// ```
    pub fn timeout(mut self, v: Duration) -> Self {
        self.timeout_nanos = Some(v.as_nanos());
        self
    }

    pub fn retries(mut self, v: u64) -> Self {
        self.retries = Some(v);
        self
    }

    /// Set the amount of time that the health check should be ignored because
    /// the container is starting up.
    ///
    /// The presumption is the health checks will fail while the container is starting,
    /// but should succeed after this period of time has elapsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use passivized_docker_engine_client::model::HealthCheck;
    ///
    /// let check = HealthCheck::default()
    ///     .start_period(Duration::from_secs(5));
    /// ```
    pub fn start_period(mut self, v: Duration) -> Self {
        self.start_period_nanos = Some(v.as_nanos());
        self
    }

}
