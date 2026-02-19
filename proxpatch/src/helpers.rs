use std::collections::HashMap;
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
