use std::process::{Command, Stdio};

pub fn exec_upgrade(node: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("root@{}", node),
            "apt-get update && DEBIAN_FRONTEND=noninteractive apt-get -y dist-upgrade",
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

pub fn exec_reboot(node: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("root@{}", node),
            "reboot",
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

pub fn val_reboot(node: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            &format!("root@{}", node),
            "test -f /var/run/reboot-required",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    Ok(output.status.success())
}
