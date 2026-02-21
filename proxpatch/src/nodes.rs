use std::process::Command;
use crate::models::{NodeResources, NodeStatus};
use crate::cluster::PveResource;
use log::{info, debug, warn, error};

pub fn get_nodes() -> Result<Vec<NodeResources>, Box<dyn std::error::Error>> {
    let output = Command::new("pvesh")
        .args([
            "get",
            "/cluster/resources",
            "--type",
            "node",
            "--output-format",
            "json",
        ])
        .output()?;

    let json = String::from_utf8(output.stdout)?;
    let mut nodes: Vec<NodeResources> = serde_json::from_str(&json)?;

    for node in &mut nodes {
        node.ip = get_node_ip(&node.node);

    }
    Ok(nodes)
}

fn get_node_ip(node: &str) -> Option<String> {
    let output = Command::new("pvesh")
        .args([
            "get",
            "/cluster/status",
            "--output-format",
            "json",
        ])
        .output()
        .ok()?;

    let json = String::from_utf8(output.stdout).ok()?;

    #[derive(serde::Deserialize)]
    struct ClusterEntry {
        #[serde(rename = "type")]
        entry_type: String,
        name: Option<String>,
        ip: Option<String>,
    }

    let entries: Vec<ClusterEntry> = serde_json::from_str(&json).ok()?;

    for entry in entries {
        if entry.entry_type == "node" {
            if let Some(name) = entry.name {
                if name == node {
                    match entry.ip {
                        Some(ip) => {
                            debug!("Node {} IP detected: {}", node, ip);
                            return Some(ip);
                        }
                        None => {
                            debug!("Node {} found but has no IP", node);
                            return None;
                        }
                    }
                }
            }
        }
    }

    debug!("Node {} not found in cluster/status", node);
    None
}

pub fn val_node_online(
    node_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("pvesh")
        .args([
            "get",
            "/cluster/resources",
            "--output-format",
            "json",
        ])
        .output()?;

    let json = String::from_utf8(output.stdout)?;
    let resources: Vec<PveResource> = serde_json::from_str(&json)?;

    for res in resources {
        if let PveResource::Node { node, status } = res {
            if node == node_name {
                let online = status == "online";
                debug!("Node {} status: {}, online: {}", node, status, online);
                debug!("Reboot completed on node: {}", node_name);
                return Ok(online);
            }
        }
    }

    debug!("Node {} not found in cluster resources", node_name);
    Ok(false)
}

pub fn wait_for_node_online(
    node_name: &str,
    timeout_secs: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let attempts = timeout_secs / 5;

    for _ in 0..attempts {
        debug!("Checking if node {} is online...", node_name);

        if val_node_online(node_name)? {
            debug!("Node {} is now online", node_name);
            return Ok(true);
        }

        debug!("Waiting for node {} to come online...", node_name);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    Ok(false)
}