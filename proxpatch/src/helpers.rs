use std::collections::HashMap;
use std::fs;
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

    match fs::metadata("/tmp/test.txt") {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("File exists");
            }
        }
        Err(e) => {
            println!("File not found: {}", e);
        }
    }