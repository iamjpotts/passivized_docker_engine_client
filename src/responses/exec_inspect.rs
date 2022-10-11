
use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Exec/operation/ExecInspect
#[derive(Clone, Debug, Deserialize)]
pub struct ExecInspectResponse {

    #[serde(rename = "CanRemove")]
    pub can_remove: bool,

    #[serde(rename = "DetachKeys")]
    pub detach_keys: String,

    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Running")]
    pub running: bool,

    #[serde(rename = "ExitCode")]
    pub exit_code: i64,

    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,

    #[serde(rename = "OpenStderr")]
    pub open_stderr: bool,

    #[serde(rename = "OpenStdout")]
    pub open_stdout: bool,

    #[serde(rename = "ContainerID")]
    pub container_id: String,

    #[serde(rename = "Pid")]
    pub pid: isize

}