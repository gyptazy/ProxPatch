use std::process::{Command, Stdio};

pub fn exec_migrate(
    user: &str,
    current_node_ip: &str,
    current_node: &str,
    target_node: &str,
    guest_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {

    let base_cmd = format!(
        "pvesh create /nodes/{}/qemu/{}/migrate -target {} -online 1 -with-local-disks 1",
        current_node, guest_id, target_node
    );

    let remote_cmd = if user == "root" {
        base_cmd
    } else {
        format!("sudo {}", base_cmd)
    };

    let status = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("{}@{}", user, current_node_ip),
            &remote_cmd,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(format!(
            "Migration of VM {} from {} to {} failed",
            guest_id, current_node, target_node
        ).into());
    }

    Ok(())
}
