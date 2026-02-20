mod calculations;
mod cli;
mod helpers;
mod migrate;
mod models;
mod nodes;
mod patch;
mod vms;

use clap::Parser;
use cli::Cli;
use crate::calculations::calculate_migrations;
use crate::helpers::node_ssh_target;
use crate::helpers::test_pkg_jq;
use crate::migrate::exec_migrate;
use crate::models::MigrationPlan;
use crate::patch::exec_reboot;
use crate::patch::exec_upgrade;
use crate::patch::val_reboot;
use models::{NodeWithVms};
use nodes::get_nodes;
use std::collections::HashMap;
use vms::get_running_vms;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let nodes = get_nodes(cli.debug)?;
    let mut cluster: HashMap<String, NodeWithVms> = HashMap::new();

    test_pkg_jq();

    for node in nodes {
        let node_name = node.node.clone();
        let vms = get_running_vms(cli.debug, &node_name)?;

        cluster.insert(
            node_name.clone(),
            NodeWithVms {
                resources: node,
                vms,
            },
        );
    }

    for (node_name, data) in cluster.iter_mut() {
        let ssh_target = data.resources.ip.as_deref().unwrap_or(node_name);

        exec_upgrade(ssh_target)?;
        data.resources.reboot_required = val_reboot(ssh_target)?;
    }

    let plans = calculate_migrations(&cluster);

    for plan in plans {
        let from_ip = cluster
            .get(&plan.from)
            .and_then(|d| d.resources.ip.as_deref())
            .unwrap_or(&plan.from);

        exec_migrate(
            from_ip,
            &plan.from,
            &plan.to,
            plan.vmid,
        )?;

    }

    for (node_name, data) in &cluster {
        if !data.resources.reboot_required {
            continue;
        }

        let ssh_target = data.resources.ip.as_deref().unwrap_or(node_name);

        println!("Rebooting {}", ssh_target);
        exec_reboot(ssh_target)?;
    }

    Ok(())
}
