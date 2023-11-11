use hyper::{Body, Client, Error, Request, Response};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper_tls::native_tls::TlsConnector;
#[cfg(unix)]
use hyperlocal::UnixConnector;

const POOL_MAX_IDLE_PER_HOST: usize = 0;

/// A thin proxy class that abstracts details and idiosyncrasies of the Hyper http client library.
///
/// Presents a unified interface for interacting with http servers, https servers, and unix sockets.
#[derive(Debug, Clone)]
pub(crate) struct HyperHttpClient {
    config: HyperHttpClientConfig
}

impl HyperHttpClient {

    pub(super) async fn apply(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        let future = match self.config {
            HyperHttpClientConfig::Http { ref client, ..} =>
                client.request(request),

            HyperHttpClientConfig::Https { ref client, ..} =>
                client.request(request),

            #[cfg(unix)]
            HyperHttpClientConfig::Unix { ref client, ..} =>
                client.request(request)

        };

        future.await
    }

    pub(crate) fn http() -> Self {
        Self {
            config: HyperHttpClientConfig::http()
        }
    }

    pub(crate) fn https(tls: TlsConnector) -> Self {
        Self {
            config: HyperHttpClientConfig::https(tls)
        }
    }

    #[cfg(unix)]
    pub(crate) fn unix() -> Self {
        Self {
            config: HyperHttpClientConfig::unix()
        }
    }

}

/// Internal configuration of the proxy class.
#[derive(Debug, Clone)]
enum HyperHttpClientConfig {
    Http {
        client: Client<HttpConnector, Body>,
    },
    Https {
        client: Client<HttpsConnector<HttpConnector>, Body>,
    },
    #[cfg(unix)]
    Unix {
        client: Client<UnixConnector, Body>,
    }
}

impl HyperHttpClientConfig {
    fn http() -> Self {
        Self::Http {
            client: Client::builder()
                .pool_max_idle_per_host(POOL_MAX_IDLE_PER_HOST)
                .build::<_, Body>(HttpConnector::new())
        }
    }

    fn https(tls: TlsConnector) -> Self {
        let mut inner = HttpConnector::new();
        inner.enforce_http(false);

        let connector = HttpsConnector::from((inner, tls.into()));

        Self::Https {
            client: Client::builder()
                .pool_max_idle_per_host(POOL_MAX_IDLE_PER_HOST)
                .build::<_, Body>(connector)
        }
    }

    #[cfg(unix)]
    fn unix() -> Self {
        Self::Unix {
            client: Client::builder()
                .pool_max_idle_per_host(POOL_MAX_IDLE_PER_HOST)
                .build::<_, Body>(UnixConnector {})
        }
    }

}

#[cfg(test)]
mod test_hyper_http_client {
    use http::StatusCode;
    use hyper::{Body, Request};

    use crate::imp::hyper_proxy::HyperHttpClient;

    #[tokio::test]
    async fn gets_from_http_server() {
        let mut server = mockito::Server::new_async().await;
        let path = "/some/file";

        server.mock("GET", path)
            .with_status(200)
            .create_async()
            .await;

        let client = HyperHttpClient::http();

        let request = Request::get(format!("{}{}", server.url(), path))
            .body(Body::empty())
            .unwrap();

        let response = client
            .apply(request)
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, response.status());
    }

}