
pub(crate) struct LogsArgs {
    pub stdout: bool,
    pub stderr: bool,
    pub timestamps: bool
}

impl Default for LogsArgs {
    fn default() -> Self {
        Self {
            stdout: true,
            stderr: true,
            timestamps: Default::default()
        }
    }
}

impl LogsArgs {

    pub fn timestamps(mut self) -> Self {
        self.timestamps = true;
        self
    }

}