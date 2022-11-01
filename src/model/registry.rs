use std::collections::HashMap;
use serde::Serialize;

/// Credentials to Docker Hub or a private registry.
///
/// See https://docs.docker.com/engine/api/v1.41/#section/Authentication
///
/// Used with X-Registry-Auth header.
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

impl RegistryAuth {

    pub(crate) fn as_config(&self) -> HashMap<String, RegistryConfig> {
        if let Some(server) = self.server.as_ref() {
            HashMap::from([
                (server.clone(), RegistryConfig::with_auth(self))
            ])
        }
        else {
            HashMap::new()
        }
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Image/operation/ImageBuild
///
/// This is an entry in a map for X-Registry-Config and is
/// different from X-Registry-Auth
#[derive(Clone, Debug, Serialize)]
pub(crate) struct RegistryConfig {
    pub username: String,
    pub password: String,
}

impl RegistryConfig {

    fn with_auth(auth: &RegistryAuth) -> Self {
        RegistryConfig {
            username: auth.username.clone(),
            password: auth.password.clone()
        }
    }

}