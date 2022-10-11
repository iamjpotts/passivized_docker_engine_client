use hyper::{Body, Response};
use hyper::client::HttpConnector;
use hyper_tls::native_tls::TlsConnector;
use super::errors::ExampleError;

fn build_client_http() -> hyper::Client<hyper::client::HttpConnector> {
    hyper::Client::builder()
        .pool_max_idle_per_host(0)
        .build::<_, hyper::Body>(hyper::client::HttpConnector::new())
}

fn build_client_from_tls(tls: TlsConnector) -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>> {
    let mut inner = HttpConnector::new();
    inner.enforce_http(false);

    hyper::Client::builder()
        .pool_max_idle_per_host(0)
        .build::<_, hyper::Body>(hyper_tls::HttpsConnector::from((inner, tls.into())))
}

pub(crate) async fn get_text_http(url: &str) -> Result<String, ExampleError> {
    let client = build_client_http();

    let request: hyper::Request<hyper::Body> = hyper::http::Request::get(url)
        .body(hyper::Body::empty())?;

    let response = client
        .request(request)
        .await?;

    parse_response(response).await
}

pub(crate) async fn get_text_https(url: &str, tls: TlsConnector) -> Result<String, ExampleError> {
    let client = build_client_from_tls(tls);

    let request: hyper::Request<hyper::Body> = hyper::http::Request::get(url)
        .body(hyper::Body::empty())?;

    let response = client
        .request(request)
        .await?;

    parse_response(response).await
}

async fn parse_response(response: Response<Body>) -> Result<String, ExampleError> {
    let response_body = hyper::body::to_bytes(response)
        .await?;

    Ok(String::from_utf8(response_body.into())?)
}
