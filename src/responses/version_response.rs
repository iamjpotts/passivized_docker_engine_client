use serde::Deserialize;
use serde_json::Value;

// See https://docs.docker.com/engine/api/v1.41/#tag/System/operation/SystemVersion
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct VersionResponse {

    #[serde(rename = "Platform")]
    pub platform: Platform,

    #[serde(rename = "Components")]
    pub components: Vec<Component>,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "ApiVersion")]
    pub api_version: String,

    #[serde(rename = "MinAPIVersion")]
    pub min_api_version: String,

    #[serde(rename = "GitCommit")]
    pub git_commit: String,

    #[serde(rename = "GoVersion")]
    pub go_version: String,

    #[serde(rename = "Os")]
    pub os: String,

    #[serde(rename = "Arch")]
    pub arch: String,

    #[serde(rename = "KernelVersion")]
    pub kernel_version: String,

    #[serde(rename = "Experimental", default)]
    pub experimental: bool,

    #[serde(rename = "BuildTime")]
    pub build_time: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Component {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Details")]
    pub details: Value,

}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Platform {

    #[serde(rename = "Name")]
    pub name: String,

}
