use passivized_docker_engine_client::responses::InspectContainerResponse;
use super::errors::ExampleError;

pub fn extract_ip_address(inspected: &InspectContainerResponse) -> Result<String, ExampleError> {
    let ip_address = inspected
        .network_settings
        .networks
        .values()
        .map(|v| v.ip_address.clone())
        .next();

    match ip_address {
        None => {
            Err(ExampleError::Message("Could not find IP address".into()))
        }
        Some(ip) => {
            Ok(ip)
        }
    }
}
