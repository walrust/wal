use serde::Serialize;

use super::VNode;

#[derive(Serialize, PartialEq, Debug)]
pub struct VList {
    nodes: Vec<VNode>,
}

impl VList {
    pub fn new(nodes: Vec<VNode>) -> VList {
        VList { nodes }
    }

    pub fn new_empty() -> VList {
        VList { nodes: Vec::new() }
    }
}
