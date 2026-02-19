use std::process::Command;
use crate::models::{NodeResources, NodeStatus};

pub fn get_nodes(debug: bool) -> Result<Vec<NodeResources>, Box<dyn std::error::Error>> {
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
        node.ip = get_node_ip(debug, &node.node);
        if debug {
            println!("[DEBUG] AAA node: {}, IP: {:?}", node.node, node.ip);
        }
    }
    Ok(nodes)
}

fn get_node_ip(debug: bool, node: &str) -> Option<String> {
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

    if debug {
        println!("[DEBUG] raw /cluster/status json: {}", json);
    }

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
                            if debug {
                                println!("[DEBUG] node {} IP detected: {}", node, ip);
                            }
                            return Some(ip);
                        }
                        None => {
                            if debug {
                                println!("[DEBUG] node {} found but has no IP", node);
                            }
                            return None;
                        }
                    }
                }
            }
        }
    }
    if debug {
        println!("[DEBUG] node {} not found in cluster/status", node);
    }
    None
}
