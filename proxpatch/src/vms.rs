use std::process::Command;
use crate::models::VmResources;
use log::{info, debug, warn, error};

pub fn get_running_vms(node: &str) -> Result<Vec<VmResources>, Box<dyn std::error::Error>> {
    debug!("→ Validating for running VMs on node: {}", node);
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

    for vm in &vms {
        let mem = vm.mem
            .map(|m| format!("{}", m / 1024 / 1024))
            .unwrap_or_else(|| "unknown".into());

        let maxmem = vm.maxmem
            .map(|m| format!("{}", m / 1024 / 1024))
            .unwrap_or_else(|| "unknown".into());

        debug!(
            "  • VM {} ({}) | CPU: {:.2} | MEM: {}/{} MB",
            vm.vmid,
            vm.name.as_deref().unwrap_or("unnamed"),
            vm.cpu.unwrap_or(0.0),
            mem,
            maxmem,
        );
    }

    debug!("✓ Found {} running VMs on node {}", vms.len(), node);
    debug!("✓ Finished validating running VMs on node: {}", node);
    Ok(vms)
}
