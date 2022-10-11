use serde::Serialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Exec/operation/ContainerExec
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateExecRequest {

    #[serde(rename = "Cmd")]
    pub cmd: Vec<String>,

    #[serde(rename = "User", skip_serializing_if = "String::is_empty")]
    pub user: String,

    #[serde(rename = "AttachStderr")]
    pub attach_stderr: bool,

    #[serde(rename = "AttachStdin")]
    pub attach_stdin: bool,

    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,

}

impl CreateExecRequest {

    /// Set the command to execute, with its arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateExecRequest;
    ///
    /// let config = CreateExecRequest::default()
    ///     .cmd(Vec::from(["echo", "Hello,", "world."]))
    ///     .attach_stdout(true);
    /// ```
    pub fn cmd<V: ToString>(mut self, v: Vec<V>) -> Self {
        self.cmd = v
            .iter()
            .map(|item| item.to_string())
            .collect();
        self
    }

    pub fn user<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.user = v.into();
        self
    }

    pub fn attach_stdin(mut self, v: bool) -> Self {
        self.attach_stdin = v;
        self
    }

    pub fn attach_stdout(mut self, v: bool) -> Self {
        self.attach_stdout = v;
        self
    }

    pub fn attach_stderr(mut self, v: bool) -> Self {
        self.attach_stderr = v;
        self
    }

}