use log::*;
use std::time::Duration;
use backoff::backoff::{Backoff, Constant};
use hyper_tls::native_tls::TlsConnector;
use tokio::net::TcpStream;

use super::errors::ExampleError;

struct LimitedRetry {
    configured_remaining: usize,
    remaining: usize,
    policy: Box<dyn Backoff>
}

impl LimitedRetry {

    fn new<B: Backoff + 'static>(remaining: usize, policy: B) -> Self {
        LimitedRetry {
            configured_remaining: remaining,
            remaining,
            policy: Box::new(policy)
        }
    }

}

impl Backoff for LimitedRetry {

    fn reset(&mut self) {
        self.policy.reset();
        self.remaining = self.configured_remaining;
    }

    fn next_backoff(&mut self) -> Option<Duration> {
        if self.remaining > 0 {
            self.remaining = self.remaining - 1;
            self.policy.next_backoff()
        }
        else {
            None
        }
    }

}

pub async fn wait_for_http_server(url: &str) -> Result<String, ExampleError> {
    info!("Will wait until can connect to {}", url);

    let interval = Duration::from_secs(2);

    tokio::time::sleep(interval).await;

    backoff::future::retry_notify(
        LimitedRetry::new(7, Constant::new(interval)),
        || async {
            super::http::get_text_http(url)
                .await
                .map_err(backoff::Error::transient)
        },
        |error, _| warn!("Retrying after failure: {:?}", error)
    ).await
}

pub async fn wait_for_https_server(url: &str, tls: TlsConnector) -> Result<String, ExampleError> {
    info!("Will wait until can connect to {}", url);

    let interval = Duration::from_secs(2);

    tokio::time::sleep(interval).await;

    backoff::future::retry_notify(
        LimitedRetry::new(5, Constant::new(interval)),
        || async {
            super::http::get_text_https(url, tls.clone())
                .await
                .map_err(backoff::Error::transient)
        },
        |error, _| warn!("Retrying after failure: {:?}", error)
    ).await
}

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
        LimitedRetry::new(4, Constant::new(interval)),
        || async {
            connect_tcp_server(host, port)
                .await
                .map_err(backoff::Error::transient)
        },
        |error, _| warn!("Retrying after failure: {:?}", error)
    ).await
}