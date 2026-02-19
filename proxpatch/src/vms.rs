use std::process::Command;
use crate::models::VmResources;

pub fn get_running_vms(debug: bool, node: &str) -> Result<Vec<VmResources>, Box<dyn std::error::Error>> {
    if debug {
        println!("[DEBUG] querying running VMs for node: {}", node);
    }

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

    if debug {
        println!("VM list: {:#?}", vms);
    }

    Ok(vms)
}
