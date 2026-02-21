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
            if vm.status != "running" {
                continue;
            }

            let vm_mem = vm.mem.unwrap_or(0) as i64;
            let mut best_node: Option<String> = None;
            let mut best_free: i64 = -1;

            for (candidate, free) in &free_mem {
                if candidate == node_name {
                    continue;
                }

                if cluster
                    .get(candidate)
                    .map(|n| n.resources.reboot_required)
                    .unwrap_or(false)
                {
                    continue;
                }

                if *free < vm_mem {
                    continue;
                }

                if *free > best_free {
                    best_free = *free;
                    best_node = Some(candidate.clone());
                }
            }

            let target = match best_node {
                Some(t) => t,
                None => {
                    println!(
                        "WARNING: No migration target for VM {} on {}",
                        vm.vmid, node_name
                    );
                    continue;
                }
            };

            println!(
                "Plan: move VM {} from {} → {}",
                vm.vmid, node_name, target
            );

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

pub fn calculate_migrations_for_node(
    node_name: &str,
    cluster: &HashMap<String, NodeWithVms>,
) -> Vec<MigrationPlan> {
    let mut plans = Vec::new();
    let mut free_mem: HashMap<String, i64> = HashMap::new();

    for (node, data) in cluster {
        let free = data.resources.maxmem as i64 - data.resources.mem as i64;
        free_mem.insert(node.clone(), free);
    }

    let data = match cluster.get(node_name) {
        Some(d) => d,
        None => return plans,
    };

    for vm in &data.vms {
        if vm.status != "running" {
            continue;
        }

        let vm_mem = vm.mem.unwrap_or(0) as i64;

        let mut best_node: Option<String> = None;
        let mut best_free = -1;

        for (candidate, free) in &free_mem {
            if candidate == node_name {
                continue;
            }

            if *free < vm_mem {
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
            from: node_name.to_string(),
            to: target.clone(),
        });

        *free_mem.get_mut(&target).unwrap() -= vm_mem;
    }

    plans
}

pub fn apply_plan_to_cluster(
    cluster: &mut HashMap<String, NodeWithVms>,
    plan: &MigrationPlan,
) {
    let vm = {
        let from_node = match cluster.get_mut(&plan.from) {
            Some(n) => n,
            None => return,
        };

        let pos = match from_node.vms.iter().position(|v| v.vmid == plan.vmid) {
            Some(p) => p,
            None => return,
        };

        let vm = from_node.vms.remove(pos);
        let mem = vm.mem.unwrap_or(0);
        from_node.resources.mem = from_node.resources.mem.saturating_sub(mem);
        vm
    };

    if let Some(to_node) = cluster.get_mut(&plan.to) {
        let mem = vm.mem.unwrap_or(0);

        to_node.resources.mem += mem;
        to_node.vms.push(vm);
    }
}