use hyper::StatusCode;

use crate::DockerEngineClient;
use crate::errors::DecUseError;
use crate::requests::CreateNetworkRequest;
use crate::responses::CreateNetworkResponse;

pub struct DecNetworks<'a> {
    pub(super) client: &'a DockerEngineClient
}

impl <'a> DecNetworks<'a> {

    /// Establish a new Docker network, and return a description of it.
    pub async fn create(&self, request: CreateNetworkRequest) -> Result<CreateNetworkResponse, DecUseError> {
        let uri = self.client.url.networks().create();
        let response = self.client.http.post_json(uri, &request)?.execute().await?;

        response
            .assert_item_status(StatusCode::CREATED)?
            .parse()
    }

}
