use web_sys::Node;

use super::{VElement, VList, VText, VComponent};

#[derive(PartialEq, Debug)]
pub enum VNode {
    Element { velement: VElement },
    Text { vtext: VText },
    List { vlist: VList },
    Component { vcomp: VComponent },
}

impl VNode {
    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        match self {
            VNode::Element { velement } => velement.patch(last, ancestor),
            VNode::Text { vtext } => vtext.patch(last, ancestor),
            VNode::Component { vcomp } => vcomp.patch(last, ancestor),
            VNode::List { .. } => todo!(),
        };
    }

    pub fn get_dom(&self) -> Option<&Node> {
        match self {
            VNode::Element { velement } => velement.dom.as_ref().map(|x| x as &Node),
            VNode::Text { vtext } => vtext.dom.as_ref().map(|x| x as &Node),
            VNode::Component { vcomp } => vcomp.comp.as_ref().unwrap().vdom().get_dom(),
            VNode::List { .. } => todo!(),
        }
    }
}

impl From<VElement> for VNode {
    fn from(velement: VElement) -> Self {
        Self::Element {
            velement,
        }
    }
}

impl From<VComponent> for VNode {
    fn from(vcomp: VComponent) -> Self {
        Self::Component { 
            vcomp
        }
    }
}

impl From<VText> for VNode {
    fn from(vtext: VText) -> Self {
        Self::Text {
            vtext,
        }
    }
}

impl From<VList> for VNode {
    fn from(vlist: VList) -> Self {
        Self::List { vlist }
    }
}

impl<T: ToString> From<T> for VNode {
    fn from(t: T) -> Self {
        Self::Text {
            vtext: VText::new(t),
        }
    }
}

impl<T: Into<VNode>> FromIterator<T> for VNode {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::List {
            vlist: VList::new(iter.into_iter().map(|t| t.into()).collect()),
        }
    }
}
