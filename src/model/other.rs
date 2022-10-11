use serde::{Deserialize, Serialize};

/// Some of the maps in the Docker REST api have an empty object {} as their value.
///
/// E.g., json like so:
///
/// {
///     "key1": {},
///     "key2": {}
/// }
///
/// This type is used to deserialize and serialize those maps.
///
/// For example, the ExposedPorts property of a container creation request,
/// described at https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Unit;
