use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_ssh_user")]
    pub ssh_user: String,
    #[serde(default)]
    pub deactivate_proxlb: bool,
    #[serde(default)]
    pub excluded_nodes: Vec<String>,
}

fn default_ssh_user() -> String {
    "root".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct NodeResources {
    pub node: String,
    #[serde(default)]
    pub mem: u64,
    #[serde(default)]
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
