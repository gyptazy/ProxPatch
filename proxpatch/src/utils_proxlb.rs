use std::process::Command;
use log::{debug, info, warn, error};

pub fn is_package_proxlb_installed() -> bool {
    debug!("→ Checking if package 'proxlb' is installed...");
    match Command::new("dpkg").args(["-s", "proxlb"]).output() {
        Ok(output) => {
            if output.status.success() {
                debug!("✓ ProxLB is installed");
                true
            } else {
                debug!("✓ ProxLB is not installed");
                false
            }
        }
        Err(e) => {
            error!("✗ Failed to check is ProxLB is installed.");
            false
        }
    }
}

pub fn set_systemd_proxlb(action: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ {} systemd unit ProxLB", action);
    let output = Command::new("systemctl")
        .args([action, "proxlb.service"])
        .output()?;

    if output.status.success() {
        debug!("✓ ProxLB got signal to {}", action);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("✗ Failed to {} ProxLB: {}", action, stderr.trim());
        Err(format!("✗ systemctl {} failed for ProxLB", action).into())
    }
}