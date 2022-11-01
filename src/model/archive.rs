
/// The contents of a .tar file.
///
/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerArchive
#[derive(Clone)]
pub struct Tar(pub Vec<u8>);
