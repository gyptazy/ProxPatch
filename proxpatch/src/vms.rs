use std::process::Command;
use crate::models::VmResources;
use log::{info, debug, warn, error};

pub fn get_running_vms(node: &str) -> Result<Vec<VmResources>, Box<dyn std::error::Error>> {

    debug!("Querying running VMs for node: {}", node);

    let output = Command::new("pvesh")
        .args([
            "get",
            &format!("/nodes/{}/qemu", node),
            "--output-format",
            "json",
        ])
        .output()?;

    let json = String::from_utf8(output.stdout)?;

    let vms: Vec<VmResources> = serde_json::from_str::<Vec<VmResources>>(&json)?
        .into_iter()
        .filter(|vm| vm.status == "running")
        .collect();

    debug!("Found {} running VMs on node {}", vms.len(), node);
    debug!("VM details: {:#?}", vms);

    Ok(vms)
}
