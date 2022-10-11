use std::fmt::{Display, Formatter};

/// Set whether a volume is mounted in read-only mode or can be written to
/// by the container.
///
/// # Example
///
/// ```rust
///
/// use passivized_docker_engine_client::model::MountMode::ReadOnly;
/// use passivized_docker_engine_client::requests::{CreateContainerRequest, HostConfig};
///
/// CreateContainerRequest::default()
///     .host_config(HostConfig::default()
///         .mount("/home/username/scratch/htpasswd", "/secrets/htpasswd", ReadOnly)
///     );
/// ```
#[derive(Debug)]
pub enum MountMode {
    ReadOnly,
    Writable
}

impl Display for MountMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            MountMode::ReadOnly => "ro",
            MountMode::Writable => "rw"
        };

        write!(f, "{}", result)
    }
}


#[cfg(test)]
mod test_mount_mode {
    use super::MountMode;

    #[test]
    fn display_read_only() {
        let actual = format!("{}", MountMode::ReadOnly);
        assert_eq!("ro", actual);
    }
}