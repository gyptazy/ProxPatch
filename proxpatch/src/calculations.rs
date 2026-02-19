use std::collections::HashMap;
use crate::models::{NodeWithVms, MigrationPlan};

pub fn calculate_migrations(
    cluster: &HashMap<String, NodeWithVms>,
) -> Vec<MigrationPlan> {

    let mut plans = Vec::new();
    let mut free_mem: HashMap<String, i64> = HashMap::new();

    for (node, data) in cluster {
        let free = data.resources.maxmem as i64 - data.resources.mem as i64;
        free_mem.insert(node.clone(), free);
    }

    for (node_name, data) in cluster {
        if !data.resources.reboot_required {
            continue;
        }

        println!("Calculating migrations for {}", node_name);

        for vm in &data.vms {
            let vm_mem = vm.mem.unwrap_or(0) as i64;

            let mut best_node: Option<String> = None;
            let mut best_free: i64 = -1;

            for (candidate, free) in &free_mem {
                if candidate == node_name {
                    continue;
                }

                if *free > best_free {
                    best_free = *free;
                    best_node = Some(candidate.clone());
                }
            }

            let target = match best_node {
                Some(t) => t,
                None => continue,
            };

            plans.push(MigrationPlan {
                vmid: vm.vmid,
                from: node_name.clone(),
                to: target.clone(),
            });

            *free_mem.get_mut(&target).unwrap() -= vm_mem;
            *free_mem.get_mut(node_name).unwrap() += vm_mem;
        }
    }

    plans
}
