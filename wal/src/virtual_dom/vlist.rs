use itertools::{EitherOrBoth, Itertools};
use web_sys::Node;

use crate::utils::debug;

use super::VNode;

#[derive(PartialEq, Debug)]
pub struct VList {
    pub nodes: Vec<VNode>,
}

impl VList {
    pub fn new(nodes: Vec<VNode>) -> VList {
        VList { nodes }
    }

    pub fn new_empty() -> VList {
        VList { nodes: Vec::new() }
    }

    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        debug::log("Patching list");
        let mut old_virt: Option<VList> = None;

        match last {
            None => {
                debug::log("\tCreating list for the first time");
            }
            Some(VNode::List(vlist)) => {
                debug::log("\tComparing two lists");
                old_virt = Some(vlist);
            }
            Some(VNode::Text(v)) => {
                debug::log("\tCreating list for the first time and swapping with existing text");
                v.erase();
            }
            Some(VNode::Element(v)) => {
                debug::log("\tCreating list for the first time and swapping with existing element");
                v.erase();
            }
            Some(VNode::Component(v)) => {
                debug::log("\tCreating list for the first time and swapping with existing comp");
                v.erase();
            }
        }

        self.render(old_virt, ancestor);
    }

    pub fn erase(&self) {
        for node in self.nodes.iter() {
            node.erase();
        }
    }
}

impl VList {
    fn render(&mut self, last: Option<VList>, ancestor: &Node) {
        for e in self
            .nodes
            .iter_mut()
            .zip_longest(last.map_or_else(|| vec![], |x| x.nodes.into_iter().collect()))
        {
            match e {
                EitherOrBoth::Both(cur, old) => cur.patch(Some(old), ancestor),
                EitherOrBoth::Left(cur) => cur.patch(None, ancestor),
                EitherOrBoth::Right(old) => old.erase(),
            }
        }
    }
}
