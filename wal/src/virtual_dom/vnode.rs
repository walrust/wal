use web_sys::Node;

use super::{VComponent, VElement, VList, VText};

#[derive(PartialEq, Debug)]
pub enum VNode {
    Element(VElement),
    Text(VText),
    List(VList),
    Component(VComponent),
}

impl VNode {
    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        match self {
            VNode::Element(velement) => velement.patch(last, ancestor),
            VNode::Text(vtext) => vtext.patch(last, ancestor),
            VNode::Component(vcomp) => vcomp.patch(last, ancestor),
            VNode::List(vlist) => vlist.patch(last, ancestor),
        };
    }

    pub fn erase(&self) {
        match self {
            VNode::Element(v) => v.erase(),
            VNode::Text(v) => v.erase(),
            VNode::List(v) => v.erase(),
            VNode::Component(v) => v.erase(),
        }
    }
}

impl From<VElement> for VNode {
    fn from(velement: VElement) -> Self {
        Self::Element(velement)
    }
}

impl From<VComponent> for VNode {
    fn from(vcomp: VComponent) -> Self {
        Self::Component(vcomp)
    }
}

impl From<VText> for VNode {
    fn from(vtext: VText) -> Self {
        Self::Text(vtext)
    }
}

impl From<VList> for VNode {
    fn from(vlist: VList) -> Self {
        Self::List(vlist)
    }
}

impl<T: ToString> From<T> for VNode {
    fn from(t: T) -> Self {
        Self::Text(VText::new(t))
    }
}

impl<T: Into<VNode>> FromIterator<T> for VNode {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::List(VList::new(iter.into_iter().map(Into::into).collect(), None))
    }
}
