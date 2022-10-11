use serde::Serialize;

/// Credentials to Docker Hub or a private registry.
///
/// See https://docs.docker.com/engine/api/v1.41/#section/Authentication
#[derive(Clone, Debug, Default, Serialize)]
pub struct RegistryAuth {
    pub username: String,
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Private registry server, typically a FQDN but sometimes an IP address.
    ///
    /// Do not include a http or https prefix.
    #[serde(rename = "serveraddress", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>
}
