use std::process::{Command, Stdio};

pub fn exec_upgrade(user: &str, node: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        eprintln!("Upgrade failed on {}", node);
    } else {
        println!("Upgrade completed on {}", node);
    }

    Ok(())
}

pub fn exec_reboot(user: &str, node: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        eprintln!("Reboot failed on {}", node);
    } else {
        println!("Reboot completed on {}", node);
    }

    Ok(())
}

pub fn val_reboot(user: &str,node: &str) -> Result<bool, Box<dyn std::error::Error>> {
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

    Ok(output.status.success())
}
