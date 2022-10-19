use log::*;
use std::time::Duration;
use backoff::backoff::{Constant};
use passivized_test_support::retry::Limit;
use tokio::net::TcpStream;

pub async fn connect_tcp_server(host: &str, port: u16) -> Result<(), std::io::Error> {
    info!("Will wait until can connect to {}:{}", host, port);

    let connection = TcpStream::connect(format!("{}:{}", host, port)).await?;
    drop(connection);
    Ok(())
}

pub async fn wait_for_tcp_server(host: &str, port: u16) -> Result<(), std::io::Error> {
    let interval = Duration::from_secs(2);

    tokio::time::sleep(interval).await;

    backoff::future::retry_notify(
        Limit::new(4, Constant::new(interval)),
        || async {
            connect_tcp_server(host, port)
                .await
                .map_err(backoff::Error::transient)
        },
        |error, _| warn!("Retrying after failure: {:?}", error)
    ).await
}