use std::process::{Command, Stdio};
use log::{info, debug, warn, error};

pub fn exec_upgrade(user: &str, node: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Starting upgrade on node: {}", node);
    let remote_cmd = if user == "root" {
        String::from("apt-get update && DEBIAN_FRONTEND=noninteractive apt-get -y dist-upgrade")
    } else {
        String::from("sudo apt-get update && DEBIAN_FRONTEND=noninteractive apt-get -y dist-upgrade")
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("{}@{}", user, node),
            &remote_cmd,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        error!("✗ Upgrade failed on {}", node);
    } else {
        info!("✓ Upgrade completed on {}", node);
    }

    Ok(())
}

pub fn exec_reboot(user: &str, node: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Starting reboot for node: {}", node);
    let remote_cmd = if user == "root" {
        String::from("reboot")
    } else {
        String::from("sudo reboot")
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("{}@{}", user, node),
            &remote_cmd,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        error!("✗ Reboot failed on {}", node);
    } else {
        info!("→ Rebooting node {}", node);
    }

    Ok(())
}

pub fn val_reboot(user: &str,node: &str) -> Result<bool, Box<dyn std::error::Error>> {
    debug!("→ Validating if reboot is required for node: {}", node);
    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("{}@{}", user, node),
            "test -f /var/run/reboot-required",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    let reboot_required = output.status.success();

    if reboot_required {
        debug!("! Reboot required for node: {}", node);
    } else {
        debug!("✓ Reboot not required for node: {}", node);
    }

    Ok(reboot_required)
}

pub fn exec_enable_maintenance(user: &str, node: &str, node_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Setting node {} into maintenance mode.", node_name);
    let remote_cmd = if user == "root" {
        format!("ha-manager crm-command node-maintenance enable {}", node_name)
    } else {
        format!("sudo ha-manager crm-command node-maintenance enable {}", node_name)
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("{}@{}", user, node),
            &remote_cmd,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        error!("✗ Unable to set node {} into maintenance mode.", node_name);
    } else {
        info!("✓ Node {} is now in maintenance mode.", node_name);
    }

    Ok(())
}

pub fn exec_disable_maintenance(user: &str, node: &str, node_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Disabling maintenance mode on node {}.", node_name);
    let remote_cmd = if user == "root" {
        format!("ha-manager crm-command node-maintenance disable {}", node_name)
    } else {
        format!("sudo ha-manager crm-command node-maintenance disable {}", node_name)
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("{}@{}", user, node),
            &remote_cmd,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        error!("✗ Unable to disable maintenance mode on node {}.", node_name);
    } else {
        info!("✓ Maintenance mode disabled on node {}.", node_name);
    }

    Ok(())
}