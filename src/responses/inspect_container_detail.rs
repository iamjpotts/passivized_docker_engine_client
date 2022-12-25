use serde::Deserialize;
use std::collections::HashMap;
use crate::imp::serde::dz_hashmap;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct GraphDriver {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Data", deserialize_with = "dz_hashmap")]
    pub data: HashMap<String, String>
}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Health {

    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "FailingStreak")]
    pub failing_streak: i32,

    #[serde(rename = "Log")]
    pub log: Vec<HealthCheckResult>

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct HealthCheckResult {

    #[serde(rename = "Start")]
    pub start: String,

    #[serde(rename = "End")]
    pub end: String,

    #[serde(rename = "ExitCode")]
    pub exit_code: i64,

    #[serde(rename = "Output")]
    pub output: String

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct State {

    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "Running")]
    pub running: bool,

    #[serde(rename = "Paused")]
    pub paused: bool,

    #[serde(rename = "Restarting")]
    pub restarting: bool,

    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,

    #[serde(rename = "Dead")]
    pub dead: bool,

    #[serde(rename = "Pid")]
    pub pid: i32,

    #[serde(rename = "ExitCode")]
    pub exit_code: i64,

    #[serde(rename = "StartedAt")]
    pub started_at: String,

    #[serde(rename = "FinishedAt")]
    pub finished_at: String,

    #[serde(rename = "Health")]
    pub health: Option<Health>,

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect
#[derive(Clone, Debug, Default, Deserialize)]
pub struct MountPoint {

    #[serde(rename = "Type")]
    pub mount_type: String,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "Source")]
    pub source: String,

    #[serde(rename = "Destination")]
    pub destination: String,

    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    #[serde(rename = "Mode")]
    pub mode: String,

    #[serde(rename = "RW")]
    pub rw: bool,

    #[serde(rename = "Propagation")]
    pub propagation: String

}
