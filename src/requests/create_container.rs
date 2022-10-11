use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::Duration;

use serde::Serialize;

use crate::model::{HealthCheck, ContainerIpamConfig, Unit, MountMode, PortBinding};

// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateContainerRequest {

    // Provided via URL query string, not via POST request body.
    #[serde(skip_serializing)]
    pub name: Option<String>,

    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(rename = "Domainname", skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,

    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(rename = "AttachStdin", skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,

    #[serde(rename = "AttachStdout", skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,

    #[serde(rename = "AttachStderr", skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,

    #[serde(rename = "ExposedPorts", skip_serializing_if = "HashMap::is_empty")]
    pub exposed_ports: HashMap<String, Unit>,

    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,

    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,

    #[serde(rename = "StdinOnce", skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,

    #[serde(rename = "Env", skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<String>,

    #[serde(rename = "Cmd", skip_serializing_if = "Vec::is_empty")]
    pub cmd: Vec<String>,

    #[serde(rename = "Healthcheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,

    #[serde(rename = "ArgsEscaped", skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,

    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(rename = "Volumes", skip_serializing_if = "HashMap::is_empty")]
    pub volumes: HashMap<String, Unit>,

    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    #[serde(rename = "Entrypoint", skip_serializing_if = "Vec::is_empty")]
    pub entry_point: Vec<String>,

    #[serde(rename = "NetworkDisabled", skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,

    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    #[serde(rename = "OnBuild", skip_serializing_if = "Vec::is_empty")]
    pub on_build: Vec<String>,

    #[serde(rename = "Labels", skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,

    #[serde(rename = "StopTimeout", skip_serializing_if = "Option::is_none")]
    pub stop_timeout_seconds: Option<u64>,

    #[serde(rename = "Shell", skip_serializing_if = "Vec::is_empty")]
    pub shell: Vec<String>,

    #[serde(rename = "HostConfig", skip_serializing_if = "Option::is_none")]
    pub host_config: Option<HostConfig>,

    #[serde(rename = "NetworkingConfig", skip_serializing_if = "Option::is_none")]
    pub networking_config: Option<NetworkingConfig>,
}

impl CreateContainerRequest {

    pub fn name<V: Into<String>>(mut self, v: V) -> Self {
        self.name = Some(v.into());
        self
    }

    pub fn host_config(mut self, v: HostConfig) -> Self {
        self.host_config = Some(v);
        self
    }

    pub fn hostname<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.hostname = Some(v.into());
        self
    }

    pub fn domain_name<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.domain_name = Some(v.into());
        self
    }

    pub fn user<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.user = Some(v.into());
        self
    }

    pub fn attach_stdin(mut self, v: bool) -> Self {
        self.attach_stdin = Some(v);
        self
    }

    pub fn attach_stdout(mut self, v: bool) -> Self {
        self.attach_stdout = Some(v);
        self
    }

    pub fn attach_stderr(mut self, v: bool) -> Self {
        self.attach_stderr = Some(v);
        self
    }

    /// Expose port of container to this Config builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let config = CreateContainerRequest::default()
    ///     .image("nginx")
    ///     .expose_port("80/tcp");
    /// ```
    pub fn expose_port<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.exposed_ports.insert(v.into(), Unit {});
        self
    }

    pub fn tty(mut self, v: bool) -> Self {
        self.tty = Some(v);
        self
    }

    pub fn open_stdin(mut self, v: bool) -> Self {
        self.open_stdin = Some(v);
        self
    }

    pub fn stdin_once(mut self, v: bool) -> Self {
        self.stdin_once = Some(v);
        self
    }

    /// Append environment variable for this container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let config = CreateContainerRequest::default()
    ///     .image("registry")
    ///     .env("REGISTRY_AUTH=htpasswd");
    /// ```
    pub fn env<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.env.push(v.into());
        self
    }

    /// Set command for this container.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let builder = CreateContainerRequest::default()
    ///     .image("busybox")
    ///     .cmd(vec!["ls", "-l"]);
    /// ```
    pub fn cmd<V: ToString>(mut self, v: Vec<V>) -> Self {
        self.cmd = v
            .iter()
            .map(|item| item.to_string())
            .collect();
        self
    }

    pub fn health_check(mut self, v: HealthCheck) -> Self {
        self.health_check = Some(v);
        self
    }

    /// Set args escaped for this container.
    ///
    /// # Note
    ///
    /// Only for Windows.
    pub fn args_escaped(mut self, v: bool) -> Self {
        self.args_escaped = Some(v);
        self
    }

    /// Set image for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// // Tagged
    /// let builder = CreateContainerRequest::default()
    ///     .image("nginx:latest");
    /// ```
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// // Untagged
    /// let builder = CreateContainerRequest::default()
    ///     .image("busybox");
    /// ```
    pub fn image<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.image = Some(v.into());
        self
    }

    /// Append volume for this container.
    ///
    /// # Example - host path
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let builder = CreateContainerRequest::default()
    ///     .image("nginx")
    ///     .volume("/path/to/volume");
    /// ```
    ///
    /// # Example - named volume
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let builder = CreateContainerRequest::default()
    ///     .image("postgres")
    ///     .volume("widget-database");
    /// ```
    pub fn volume<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.volumes.insert(v.into(), Unit {});
        self
    }

    pub fn working_dir<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.working_dir = Some(v.into());
        self
    }

    pub fn entry_point<V>(mut self, v: Vec<V>) -> Self
        where V: ToString
    {
        self.entry_point = v
            .iter()
            .map(|item| item.to_string())
            .collect();
        self
    }

    pub fn network_disabled(mut self, v: bool) -> Self {
        self.network_disabled = Some(v);
        self
    }

    pub fn mac_address<T>(mut self, v: T) -> Self
        where T: Into<String>
    {
        self.mac_address = Some(v.into());
        self
    }

    /// Set an on-build command.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let builder = CreateContainerRequest::default()
    ///     .image("nginx")
    ///     .on_build(vec!["some-command", "some-arg"]);
    /// ```
    pub fn on_build<V: ToString>(mut self, v: Vec<V>) -> Self {
        self.on_build = v
            .iter()
            .map(|item| item.to_string())
            .collect();
        self
    }

    /// Append user-defined key/value metadata.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let builder = CreateContainerRequest::default()
    ///     .image("nginx")
    ///     .label("example-label-key", "example-label-value");
    /// ```
    pub fn label<K, V>(mut self, k: K, v: V) -> Self
        where
            K: Into<String>,
            V: Into<String>
    {
        self.labels.insert(k.into(), v.into());
        self
    }

    /// Signal to send to a container when stopping it.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let builder = CreateContainerRequest::default()
    ///     .image("nginx")
    ///     .stop_signal("SIGKILL");
    /// ```
    pub fn stop_signal<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.stop_signal = Some(v.into());
        self
    }

    pub fn stop_timeout(mut self, v: Duration) -> Self {
        self.stop_timeout_seconds = Some(v.as_secs());
        self
    }

    pub fn stop_timeout_seconds(mut self, v: u64) -> Self {
        self.stop_timeout_seconds = Some(v);
        self
    }

    /// Shell for when RUN, CMD, and ENTRYPOINT use a shell.
    ///
    /// # Example
    ///
    /// ```rust
    /// use passivized_docker_engine_client::requests::CreateContainerRequest;
    ///
    /// let config = CreateContainerRequest::default()
    ///     .image("ubuntu")
    ///     .shell(vec!["/bin/bash", "-c"]);
    /// ```
    pub fn shell<V>(mut self, v: Vec<V>) -> Self
        where V: ToString
    {
        self.shell = v
            .iter()
            .map(|item| item.to_string())
            .collect();
        self
    }

    pub fn networking_config(mut self, v: NetworkingConfig) -> Self {
        self.networking_config = Some(v);
        self
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct EndpointConfig {

    #[serde(rename = "IPAMConfig")]
    pub ipam_config: ContainerIpamConfig

}

impl From<Ipv4Addr> for EndpointConfig {

    fn from(value: Ipv4Addr) -> EndpointConfig {
        EndpointConfig {
            ipam_config: value.into()
        }
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct HostConfig {

    #[serde(rename = "Binds", skip_serializing_if = "Vec::is_empty")]
    pub binds: Vec<String>,

    #[serde(rename = "CapAdd", skip_serializing_if = "Vec::is_empty")]
    pub cap_add: Vec<String>,

    #[serde(rename = "NetworkMode", skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,

    #[serde(rename = "PortBindings", skip_serializing_if = "HashMap::is_empty")]
    pub port_bindings: HashMap<String, Vec<PortBinding>>,

    #[serde(rename = "Privileged")]
    pub privileged: bool,

    #[serde(rename = "Sysctls", skip_serializing_if = "HashMap::is_empty")]
    pub sysctls: HashMap<String, String>,

    #[serde(rename = "AutoRemove")]
    pub auto_remove: bool

}

impl HostConfig {

    fn bind<CP: Into<String>, HP: Into<String>>(mut self, container_port: CP, host_ip: Option<String>, host_port: HP) -> Self {
        let binding = PortBinding {
            host_ip,
            host_port: host_port.into()
        };

        self.port_bindings
            .entry(container_port.into())
            .or_insert_with(Default::default)
            .push(binding);

        self
    }

    pub fn bind_ip<CP: Into<String>, HIP: ToString, HP: Into<String>>(self, container_port: CP, host_ip: HIP, host_port: HP) -> Self {
        self.bind(container_port, Some(host_ip.to_string()), host_port)
    }

    pub fn bind_port<CP: Into<String>, HP: Into<String>>(self, container_port: CP, host_port: HP) -> Self {
        self.bind(container_port, None, host_port)
    }

    pub fn cap_add<V: Into<String>>(mut self, v: V) -> Self {
        self.cap_add.push(v.into());
        self
    }

    pub fn mount<HP: Into<String>, CP: Into<String>>(mut self, host_path: HP, container_path: CP, mode: MountMode) -> Self {
        self.binds.push(format!("{}:{}:{}", host_path.into(), container_path.into(), mode));
        self
    }

    pub fn network_mode<V: Into<String>>(mut self, v: V) -> Self {
        self.network_mode = Some(v.into());
        self
    }

    pub fn sysctl<K: Into<String>, V: Into<String>>(mut self, k: K, v: V) -> Self {
        self.sysctls.insert(k.into(), v.into());
        self
    }

    pub fn privileged(mut self) -> Self {
        self.privileged = true;
        self
    }

    pub fn auto_remove(mut self) -> Self {
        self.auto_remove = true;
        self
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct NetworkingConfig {

    #[serde(rename = "EndpointsConfig", skip_serializing_if = "HashMap::is_empty")]
    pub endpoints_config: HashMap<String, EndpointConfig>

}

impl NetworkingConfig {

    pub fn endpoint<N: Into<String>>(mut self, network: N, config: EndpointConfig) -> Self {
        self.endpoints_config.insert(network.into(), config);
        self
    }

}
