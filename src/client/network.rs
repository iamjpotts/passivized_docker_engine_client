use hyper::StatusCode;

use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::requests::InspectNetworkArgs;
use crate::responses::InspectNetworkResponse;

pub struct DecNetwork<'a> {
    pub(super) client: &'a DockerEngineClient,
    pub(super) network_id: String
}

impl <'a> DecNetwork<'a> {

    /// Get a description of an existing network.
    pub async fn inspect(&self) -> Result<InspectNetworkResponse, DecUseError> {
        self.inspect_with(InspectNetworkArgs::default()).await
    }

    /// Get a description of an existing network, with additional options.
    pub async fn inspect_with(&self, args: InspectNetworkArgs) -> Result<InspectNetworkResponse, DecUseError> {
        let uri = self.client.url.networks().inspect(&self.network_id, args.scope, args.verbose);
        let response = self.client.http.get(uri)?.execute().await?;

        response
            .assert_item_status(StatusCode::OK)?
            .parse()
    }

    /// Delete an existing network.
    ///
    /// Remove requests are NOT idempotent; attempting to remove a removed network will return an error.
    pub async fn remove(&self) -> Result<(), DecUseError> {
        let uri = self.client.url.networks().remove(&self.network_id);
        let response = self.client.http.delete(uri)?.execute().await?;

        response
            .assert_unit_status(StatusCode::NO_CONTENT)
    }

}
