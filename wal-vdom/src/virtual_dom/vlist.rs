use serde::Serialize;

use super::VNode;

#[derive(Serialize)]
pub struct VList {
    nodes: Vec<VNode>,
}

impl VList {
    pub fn new_empty() -> VList {
        VList { nodes: Vec::new() }
    }
}
