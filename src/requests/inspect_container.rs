
/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default)]
pub struct InspectContainerArgs {

    /// Return the size of container in fields size_rw and size_root_fs.
    pub size: Option<bool>

}

impl InspectContainerArgs {

    pub fn size(mut self, v: bool) -> Self {
        self.size = Some(v);
        self
    }

}
