use std::{rc::Rc, cell::RefCell};

use crate::{virtual_dom::VNode, utils::debug};

use super::component_node::AnyComponentNode;



pub struct VDomObserver {
    component_node: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl VDomObserver {
    pub(crate) fn new() -> Self {
        Self {
            component_node: None,
        }
    }

    pub(crate) fn set_observer(&mut self, component_node: Rc<RefCell<AnyComponentNode>>) {
        self.component_node = Some(component_node);
    }

    pub fn notify(&self, new_vdom: VNode) {
        if let Some(any_componend_node) = &self.component_node {
            let mut any_component_node = any_componend_node.borrow_mut();
            any_component_node.vdom_notify(new_vdom);
        } else {
            debug::log("VDomObserver is not attached to a AnyComponentNode");
            panic!("VDomObserver is not attached to a AnyComponentNode");
        }
    }
}

pub struct ToRerenderObserver {
    any_component_node: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl ToRerenderObserver {
    pub(crate) fn new() -> Self {
        Self {
            any_component_node: None,
        }
    }

    pub(crate) fn set_observer(&mut self, any_component_node: Rc<RefCell<AnyComponentNode>>) {
        self.any_component_node = Some(any_component_node);
    }

    pub fn notify(&self) {
        if let Some(any_component_node) = &self.any_component_node {
            any_component_node.borrow_mut().rerender_notify();
        } else {
            debug::log("RerenderObserver is not attached to AnyComponentNode");
            panic!("RerenderObserver is not attached to AnyComponentNode");
        }
    }
}