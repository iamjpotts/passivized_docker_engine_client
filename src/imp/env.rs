use std::env::VarError;
use log::warn;

/// A default configuration is only available on Linux and Mac systems.
///
/// Docker Engine on Windows uses named pipes by default, but this Rust
/// library does not support named pipes.
#[cfg(not(windows))]
pub(crate) fn default_server() -> String {
    #[cfg(unix)]
    return "/var/run/docker.sock".to_string();

    #[cfg(not(unix))]
    return "tcp://localhost:2375".to_string();
}

pub(crate) fn docker_host() -> Option<String> {
    match std::env::var("DOCKER_HOST") {
        Err(e) => {
            match e {
                VarError::NotPresent => {
                    None
                }
                _ => {
                    warn!("Unable to read DOCKER_HOST environment variable: {}", e);
                    None
                }
            }
        },
        Ok(value) => {
            Some(value)
        }
    }
}

#[cfg(test)]
mod test_docker_host {
    use super::docker_host;

    #[test]
    fn gets() {
        // While it may not be present on the machine running the tests,
        // attempting to get the value should never fail.

        docker_host();
    }

}