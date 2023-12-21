use crate::virtual_dom::VNode;
use std::{any::Any, cell::RefCell, fmt, rc::Rc};
use web_sys::Node;

use super::{behavior::AnyComponentBehavior, AnyComponent, Component};

pub(crate) struct AnyComponentNode {
    component: Box<dyn AnyComponent>,
    pub depth: Option<u32>,
    to_rerender: bool,
    behavior: AnyComponentBehavior,
    pub vdom: Option<VNode>,
    ancestor: Node,
}

impl AnyComponentNode {
    pub(crate) fn new_root<C: Component + 'static>(
        component: C,
        ancestor: Node,
    ) -> Rc<RefCell<Self>> {
        Self::new_internal(component, ancestor, true, Some(0))
    }

    pub(crate) fn new<C: Component + 'static>(component: C, ancestor: Node) -> Rc<RefCell<Self>> {
        Self::new_internal(component, ancestor, false, None)
    }

    pub(crate) fn new_root_routing<C: Component + 'static>(
        component: C,
        ancestor: Node,
    ) -> Rc<RefCell<Self>> {
        Self::new_internal(component, ancestor, false, Some(0))
    }

    fn new_internal<C: Component + 'static>(
        component: C,
        ancestor: Node,
        to_patch: bool,
        depth: Option<u32>,
    ) -> Rc<RefCell<Self>> {
        let component_box = Box::new(component) as Box<dyn AnyComponent>;
        let behavior = AnyComponentBehavior::new();

        let node = Self {
            component: component_box,
            depth,
            to_rerender: false,
            behavior,
            vdom: None,
            ancestor,
        };

        let node_rc = Rc::new(RefCell::new(node));

        node_rc
            .borrow_mut()
            .behavior
            .set_any_component_node(node_rc.clone());

        if to_patch {
            node_rc.borrow_mut().view_and_patch();
        }

        node_rc
    }

    pub(crate) fn view(&mut self) {
        self.vdom = Some(self.view_internal());
    }

    pub(crate) fn view_and_patch(&mut self) {
        let mut new_vdom = self.view_internal();
        new_vdom.patch(self.vdom.take(), &self.ancestor);
        self.vdom = Some(new_vdom);
        self.to_rerender = false;
    }

    fn view_internal(&mut self) -> VNode {
        let mut new_vdom = self.component.view(&mut self.behavior);
        new_vdom.set_depth(self.depth.unwrap() + 1);
        new_vdom
    }

    pub(crate) fn update(&mut self, message: Box<dyn Any>) -> bool {
        let to_rerender = self.component.update(message);
        if !self.to_rerender && to_rerender {
            self.to_rerender = true;
            return true;
        }
        false
    }

    pub fn patch(
        &mut self,
        last_component_node: Option<Rc<RefCell<AnyComponentNode>>>,
        ancestor: &Node,
    ) {
        if let Some(last_component_node) = last_component_node {
            let last_component_node = last_component_node.clone();
            let last_component_node_vdom = last_component_node.borrow_mut().vdom.take();

            self.vdom
                .as_mut()
                .expect("Vdom should not be None while patching")
                .patch(last_component_node_vdom, ancestor);
        } else {
            self.vdom
                .as_mut()
                .expect("Vdom should not be None while patching")
                .patch(None, ancestor)
        }
    }
}

impl fmt::Debug for AnyComponentNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "AnyComponentNode".fmt(f)
    }
}
