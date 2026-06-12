use std::process::Command;
use log::{debug, error, info};

pub fn exec_enable_ceph_maintenance(user: &str, ssh_target: &str, node_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Checking whether local OSDs are ok-to-stop on {}", node_name);
    let check_cmd = if user == "root" {
        r#"
osds=$(basename -a /var/lib/ceph/osd/ceph-* 2>/dev/null | cut -d- -f2)

if [ -z "$osds" ]; then
    echo "No local OSDs found"
    exit 0
fi

for osd in $osds; do
    ceph osd ok-to-stop "$osd" | jq -e '.ok_to_stop == true' >/dev/null || {
        echo "OSD $osd is not ok-to-stop"
        exit 1
    }
done
"#
    } else {
        r#"
osds=$(basename -a /var/lib/ceph/osd/ceph-* 2>/dev/null | cut -d- -f2)

if [ -z "$osds" ]; then
    echo "No local OSDs found"
    exit 0
fi

for osd in $osds; do
    sudo ceph osd ok-to-stop "$osd" | jq -e '.ok_to_stop == true' >/dev/null || {
        echo "OSD $osd is not ok-to-stop"
        exit 1
    }
done
"#
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            ssh_target,
            check_cmd,
        ])
        .output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("✗ One or more OSDs on {} are not ok-to-stop: {} {}", node_name, stdout.trim(), stderr.trim());
        return Err(format!("Ceph cluster does not allow stopping node {}", node_name).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.contains("No local OSDs found") {
        info!("→ No local OSDs found on {}, skipping ok-to-stop checks", node_name);
    } else {
        info!("→ All local OSDs on {} are ok-to-stop", node_name);
    }

    debug!("→ Enabling maintenance mode for node {}", node_name);

    let maintenance_cmd = if user == "root" {
        format!("ceph osd set-group noout {}", node_name)
    } else {
        format!("sudo ceph osd set-group noout {}", node_name)
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            ssh_target,
            &maintenance_cmd,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("✗ Failed to enable ceph maintenance mode for {}: {}", node_name, stderr.trim());
        return Err(format!("Failed to enable ceph maintenance mode for {}", node_name).into());
    }

    info!("→ Set noout for ceph maintenance for host {}", node_name);

    Ok(())
}

pub fn exec_disable_ceph_maintenance(user: &str, ssh_target: &str, node_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Disabling maintenance mode for node {}", node_name);
    let remote_cmd = if user == "root" {
        format!("ceph osd unset-group noout {}", node_name)
    } else {
        format!("sudo ceph osd unset-group noout {}", node_name)
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            ssh_target,
            &remote_cmd,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(
            "✗ Failed to disable ceph maintenance mode for {}: {}",
            node_name,
            stderr.trim()
        );
        return Err(format!(
            "Failed to disable ceph maintenance mode for {}",
            node_name
        )
        .into());
    }

    info!("→ Removed noout for ceph maintenance from host {}", node_name);

    Ok(())
}

pub fn exec_check_ceph_mon_ok_to_stop(user: &str, ssh_target: &str, node_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Checking whether local MONs are ok-to-stop on {}", node_name);
    let check_cmd = if user == "root" {
        r#"
if ! systemctl is-active --quiet ceph-mon@$(hostname); then
    echo "No local MON found"
    exit 0
fi

ceph mon ok-to-stop $(hostname) | jq -e '.ok_to_stop == true' >/dev/null || {
    echo "MON $(hostname) is not ok-to-stop"
    exit 1
}
"#
    } else {
        r#"
if ! systemctl is-active --quiet ceph-mon@$(hostname); then
    echo "No local MON found"
    exit 0
fi

sudo ceph mon ok-to-stop $(hostname) | jq -e '.ok_to_stop == true' >/dev/null || {
    echo "MON $(hostname) is not ok-to-stop"
    exit 1
}
"#
    };

    let output = Command::new("ssh")
        .args([
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            ssh_target,
            check_cmd,
        ])
        .output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("✗ MON on {} is not ok-to-stop: {} {}", node_name, stdout.trim(), stderr.trim());
        return Err(format!("Ceph cluster does not allow stopping MON on {}", node_name).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.contains("No local MON found") {
        info!("→ No local MON found on {}, skipping ok-to-stop checks", node_name);
    } else {
        info!("→ Local MON on {} is ok-to-stop", node_name);
    }

    Ok(())
}