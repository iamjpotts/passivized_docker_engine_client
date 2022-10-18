use std::collections::HashMap;

use serde::Deserialize;

use crate::imp::serde::{dz_hashmap_keys, dz_vec};
use crate::model::{HealthCheck, Unit};
use crate::responses::NetworkSettings;
use crate::responses::inspect_container_detail::{GraphDriver, MountPoint, State};

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct InspectContainerResponse {

    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "Created")]
    pub created: String,

    #[serde(rename = "Path")]
    pub path: String,

    #[serde(rename = "Args")]
    pub args: Vec<String>,

    #[serde(rename = "State")]
    pub state: State,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: String,

    #[serde(rename = "HostnamePath")]
    pub hostname_path: String,

    #[serde(rename = "HostsPath")]
    pub hosts_path: String,

    #[serde(rename = "LogPath")]
    pub log_path: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "RestartCount")]
    pub restart_count: i32,

    #[serde(rename = "Driver")]
    pub driver: String,

    #[serde(rename = "Platform")]
    pub platform: String,

    #[serde(rename = "MountLabel")]
    pub mount_label: String,

    #[serde(rename = "ProcessLabel")]
    pub process_label: String,

    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: String,

    #[serde(rename = "ExecIDs", deserialize_with = "dz_vec")]
    pub exec_ids: Vec<String>,

    #[serde(rename = "HostConfig")]
    pub host_config: InspectedContainerHostConfig,

    #[serde(rename = "GraphDriver")]
    pub graph_driver: GraphDriver,

    #[serde(rename = "SizeRw")]
    pub size_rw: Option<i64>,

    #[serde(rename = "SizeRootFs")]
    pub size_root_fs: Option<i64>,

    #[serde(rename = "Mounts")]
    pub mounts: Vec<MountPoint>,

    #[serde(rename = "Config")]
    pub config: InspectedContainerConfig,

    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings
}

impl InspectContainerResponse {

    /// Get the first ip address of the first network, without regard
    /// to what kind of network it is on.
    ///
    /// Useful for simple cases in controlled environments, like automated tests.
    pub fn first_ip_address(&self) -> Option<&str> {
        self.network_settings.first_ip_address()
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct InspectedContainerConfig {

    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,

    #[serde(rename = "Domainname")]
    pub domain_name: String,

    #[serde(rename = "User")]
    pub user: String,

    #[serde(rename = "AttachStdin")]
    pub attach_stdin: bool,

    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,

    #[serde(rename = "AttachStderr")]
    pub attach_stderr: bool,

    #[serde(rename = "ExposedPorts", default, deserialize_with = "dz_hashmap_keys")]
    pub exposed_ports: HashMap<String, Unit>,

    #[serde(rename = "Tty")]
    pub tty: bool,

    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,

    #[serde(rename = "StdinOnce")]
    pub stdin_once: bool,

    #[serde(rename = "Env", deserialize_with = "dz_vec")]
    pub env: Vec<String>,

    #[serde(rename = "Cmd", deserialize_with = "dz_vec")]
    pub cmd: Vec<String>,

    #[serde(rename = "Healthcheck")]
    pub health_check: Option<HealthCheck>,

    #[serde(rename = "Image")]
    pub image: Option<String>,

    #[serde(rename = "Volumes", default, deserialize_with = "dz_hashmap_keys")]
    pub volumes: HashMap<String, Unit>,

    #[serde(rename = "WorkingDir")]
    pub working_dir: String,

    #[serde(rename = "Entrypoint", deserialize_with = "dz_vec")]
    pub entry_point: Vec<String>,

    #[serde(rename = "NetworkDisabled")]
    pub network_disabled: Option<bool>,

    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

    #[serde(rename = "OnBuild", deserialize_with = "dz_vec")]
    pub on_build: Vec<String>,

    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "StopSignal")]
    pub stop_signal: Option<String>,

    #[serde(rename = "StopTimeout")]
    pub stop_timeout_seconds: Option<u64>,

    #[serde(rename = "Shell", default, deserialize_with = "dz_vec")]
    pub shell: Vec<String>,
}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct InspectedContainerHostConfig {

    #[serde(rename = "NetworkMode")]
    pub network_mode: String,

    #[serde(rename = "Privileged")]
    pub privileged: bool,

}
