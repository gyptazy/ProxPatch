mod calculations;
mod cli;
mod cluster;
mod config;
mod helpers;
mod logging;
mod migrate;
mod models;
mod nodes;
mod patch;
mod version;
mod vms;

use clap::Parser;
use cli::Cli;
use crate::calculations::calculate_migrations;
use crate::calculations::calculate_migrations_for_node;
use crate::calculations::apply_plan_to_cluster;
use crate::config::load_config;
use crate::cluster::val_cluster_status;
use crate::helpers::node_ssh_target;
use crate::helpers::test_pkg_jq;
use crate::migrate::exec_migrate;
use crate::models::MigrationPlan;
use crate::patch::exec_reboot;
use crate::patch::exec_upgrade;
use crate::patch::val_reboot;
use log::{info, debug, warn, error};
use models::{NodeWithVms};
use nodes::get_nodes;
use nodes::wait_for_node_online;
use std::collections::HashMap;
use version::VERSION;
use vms::get_running_vms;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    logging::init(cli.debug)?;
    info!("→ Starting ProxPatch v{}... (https://gyptazy.com/proxpatch/)", VERSION);

    test_pkg_jq();

    debug!("→ Validating for custom config file...");
    let config = if let Some(path) = cli.config.as_deref() {
        debug!("→ Processing custom config file: {}", path);
        Some(load_config(path)?)
    } else {
        debug!("✓ No custom config file specified. Processing with defaults.");
        None
    };

    debug!("→ Processing user validation...");
    let user = config.as_ref().map_or_else(
        || {
            debug!("✓ Using user: root");
            "root"
        },
        |c| {
            debug!("✓ Using user from config: {}", c.ssh_user);
            c.ssh_user.as_str()
        },
    );

    let nodes = get_nodes()?;
    let mut cluster: HashMap<String, NodeWithVms> = HashMap::new();

    for node in nodes {
        let node_name = node.node.clone();
        let vms = get_running_vms(&node_name)?;

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
        exec_upgrade(user, ssh_target)?;
        data.resources.reboot_required = val_reboot(user, ssh_target)?;
    }

    let node_order: Vec<String> = cluster.keys().cloned().collect();

    for node_name in node_order {
        let reboot_required = cluster
            .get(&node_name)
            .map(|d| d.resources.reboot_required)
            .unwrap_or(false);

        if !reboot_required {
            continue;
        }

        let plans = calculate_migrations_for_node(&node_name, &cluster);

        for plan in plans {
            let from_ip = cluster
                .get(&plan.from)
                .and_then(|d| d.resources.ip.as_deref())
                .unwrap_or(&plan.from);

            exec_migrate(user, from_ip, &plan.from, &plan.to, plan.vmid)?;

            apply_plan_to_cluster(&mut cluster, &plan);
        }

        if !val_cluster_status()? {
            error!("Cluster unhealthy after reboot of {}", node_name);
            return Err(format!("Cluster unhealthy. Not rebooting {}", node_name).into());
        }

        let ssh_target = cluster
            .get(&node_name)
            .and_then(|d| d.resources.ip.as_deref())
            .unwrap_or(&node_name);

        exec_reboot(user, ssh_target)?;
        std::thread::sleep(std::time::Duration::from_secs(120));

        if !wait_for_node_online(&node_name, 30)? {
            error!("Node {} did not come back online in time", node_name);
            return Err(format!("Node {} failed to rejoin cluster", node_name).into());
        }

        if !val_cluster_status()? {
            error!("Cluster unhealthy after reboot of {}", node_name);
            return Err(format!("Cluster unhealthy after reboot of {}", node_name).into());
        }
    }

    info!("✓ All nodes up-to-date. Cluster healthy.");
    debug!("→ Waiting for next update cycle.");
    Ok(())

}