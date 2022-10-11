
/// See https://docs.docker.com/engine/api/v1.41/#tag/Network/operation/NetworkInspect
#[derive(Clone, Default)]
pub struct InspectNetworkArgs {

    /// "Detailed inspect output for troubleshooting"
    pub verbose: bool,

    /// "Filter the network by scope (swarm, global, or local)"
    pub scope: Option<String>
}
