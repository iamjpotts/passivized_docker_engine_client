use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use native_tls::Certificate;
use serde_json::{from_slice, Value};

pub async fn certificate_from_pem_file<F: AsRef<Path>>(file_name: F) -> Certificate {
    let bytes = tokio::fs::read(file_name)
        .await
        .unwrap();

    let cert = Certificate::from_pem(&bytes)
        .unwrap();

    cert
}

pub async fn extract_ip_address_from_cert_def() -> Ipv4Addr {
    let cert_def_file = PathBuf::from(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("certificate")
        .join("testregistry.locallan.json");

    let cert_def_json = tokio::fs::read(cert_def_file)
        .await
        .unwrap();

    let parsed: Value = from_slice(&cert_def_json)
        .unwrap();

    let host = parsed.get("hosts")
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .unwrap();

    Ipv4Addr::from_str(host)
        .unwrap()
}