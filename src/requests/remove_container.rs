
/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerDelete
#[derive(Clone, Debug, Default)]
pub struct RemoveContainerArgs {
    pub force: Option<bool>,
    pub remove_link: Option<bool>,
    pub remove_volumes: Option<bool>,
}

impl RemoveContainerArgs {

    pub fn force(mut self, v: bool) -> Self {
        self.force = Some(v);
        self
    }

    pub fn remove_link(mut self, v: bool) -> Self {
        self.remove_link = Some(v);
        self
    }

    pub fn remove_volumes(mut self, v: bool) -> Self {
        self.remove_volumes = Some(v);
        self
    }

}
