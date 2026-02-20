use std::collections::HashMap;
use std::fs;
use std::process;
use crate::models::NodeWithVms;

pub fn node_ssh_target<'a>(
    cluster: &'a HashMap<String, NodeWithVms>,
    node: &'a str,
) -> &'a str {
    cluster
        .get(node)
        .and_then(|d| d.resources.ip.as_deref())
        .unwrap_or(node)
}

pub fn test_pkg_jq() {
    match fs::metadata("/usr/bin/jq") {
        Ok(metadata) => {
        }
        Err(e) => {
            eprintln!("Error: jq is not installed. Please install jq to use this ProxPatch.");
            process::exit(2);
        }
    }
}