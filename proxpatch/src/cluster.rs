use std::process::Command;
use serde::Deserialize;
use log::{info, debug, warn, error};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PveResource {
    #[serde(rename = "node")]
    Node {
        node: String,
        status: String,
    },

    #[serde(rename = "qemu")]
    Qemu {
        name: String,
        node: String,
        status: String,
    },

    #[serde(rename = "storage")]
    Storage {},

    #[serde(rename = "sdn")]
    Sdn {},
}

pub fn val_cluster_status() -> Result<bool, Box<dyn std::error::Error>> {
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

    debug!("Cluster resources: {:#?}", resources);
    debug!("Cluster resources json:: {:#?}", json);

    let mut all_online = true;

    for res in resources {
        if let PveResource::Node { node, status } = res {
            let online = status == "online";

            debug!("Node: {}, online: {}", node, online);

            if !online {
                all_online = false;
            }
        }
    }
    Ok(all_online)
}