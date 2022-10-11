use std::collections::HashMap;

use serde::Serialize;

use crate::model::NetworkIpam;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Network/operation/NetworkCreate
#[derive(Clone, Debug, Serialize)]
pub struct CreateNetworkRequest {

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "CheckDuplicate")]
    check_duplicate: bool,

    #[serde(rename = "Driver")]
    driver: Option<String>,

    #[serde(rename = "Internal")]
    internal: Option<bool>,

    #[serde(rename = "Attachable")]
    attachable: Option<bool>,

    #[serde(rename = "Ingress")]
    ingress: Option<bool>,

    #[serde(rename = "IPAM")]
    ipam: NetworkIpam,

    #[serde(rename = "EnableIPv6")]
    enable_ipv6: Option<bool>,

    #[serde(rename = "Options")]
    options: HashMap<String, String>,

    #[serde(rename = "Labels")]
    labels: HashMap<String, String>
}

impl CreateNetworkRequest {

    pub fn name<V: Into<String>>(mut self, v: V) -> Self {
        self.name = v.into();
        self
    }

    pub fn check_duplicate(mut self, v: bool) -> Self {
        self.check_duplicate = v;
        self
    }

    pub fn driver<V: Into<String>>(mut self, v: V) -> Self {
        self.driver = Some(v.into());
        self
    }

    pub fn internal(mut self, v: bool) -> Self {
        self.internal = Some(v);
        self
    }

    pub fn attachable(mut self, v: bool) -> Self {
        self.attachable = Some(v);
        self
    }

    pub fn ingress(mut self, v: bool) -> Self {
        self.ingress = Some(v);
        self
    }

    pub fn ipam(mut self, v: NetworkIpam) -> Self {
        self.ipam = v;
        self
    }

    pub fn enable_ipv6(mut self, v: bool) -> Self {
        self.enable_ipv6 = Some(v);
        self
    }

    /// Add an option. Can be called multiple times.
    pub fn option<K, V>(mut self, k: K, v: V) -> Self
        where
            K: Into<String>,
            V: Into<String>
    {
        self.options.insert(k.into(), v.into());
        self
    }

    /// Add a label. Can be called multiple times.
    pub fn label<K, V>(mut self, k: K, v: V) -> Self
        where
            K: Into<String>,
            V: Into<String>
    {
        self.labels.insert(k.into(), v.into());
        self
    }
}

impl Default for CreateNetworkRequest {
    fn default() -> Self {
        Self {
            name: Default::default(),
            // The official documentation doesn't specify what the default is. True seems like a reasonable choice,
            // so we explicitly set the check to True.
            check_duplicate: true,
            driver: Default::default(),
            internal: Default::default(),
            attachable: Default::default(),
            ingress: Default::default(),
            ipam: Default::default(),
            enable_ipv6: Default::default(),
            options: Default::default(),
            labels: Default::default()
        }
    }
}
