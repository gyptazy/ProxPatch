use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ssh_user: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NodeResources {
    pub node: String,
    pub mem: u64,
    pub maxmem: u64,
    pub ip: Option<String>,
    #[serde(default)]
    pub reboot_required: bool,
}

#[derive(Debug, Deserialize)]
pub struct VmResources {
    pub vmid: u64,
    pub name: Option<String>,
    pub status: String,
    pub mem: Option<u64>,
    pub maxmem: Option<u64>,
    pub cpu: Option<f64>,
}

#[derive(Debug)]
pub struct NodeWithVms {
    pub resources: NodeResources,
    pub vms: Vec<VmResources>,
}

#[derive(Debug, Deserialize)]
pub struct NodeStatus {
    #[serde(default)]
    pub ip: Option<String>,
}

pub struct MigrationPlan {
    pub vmid: u64,
    pub from: String,
    pub to: String,
}
