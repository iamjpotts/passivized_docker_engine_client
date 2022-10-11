
/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerWait
#[derive(Clone, Debug)]
pub enum WaitCondition {
    /// Not supported by Docker on Windows.
    NotRunning,

    /// Wait until the container exits again. If the container is stopped, this will wait
    /// until it is started and stopped. If the container is never started after waiting
    /// for NextExit, this waits indefinitely.
    NextExit,

    /// Wait until the container no longer exists.
    Removed,
}
