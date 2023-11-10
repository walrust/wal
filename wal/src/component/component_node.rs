use crate::virtual_dom::VNode;
use std::{any::Any, cell::RefCell, fmt, rc::Rc};
use web_sys::Node;

use super::{behavior::AnyComponentBehavior, AnyComponent, Component};

pub struct AnyComponentNode {
    component: Box<dyn AnyComponent>,
    pub depth: u32,
    to_rerender: bool,
    behavior: AnyComponentBehavior,
    pub vdom: Option<VNode>,
    ancestor: Node,
}

impl AnyComponentNode {
    pub(crate) fn new_root<C: Component + 'static>(component: C, ancestor: Node) -> Rc<RefCell<Self>> {
        Self::new_internal(component, ancestor, true)
    }

    pub(crate) fn new<C: Component + 'static>(component: C, ancestor: Node) -> Rc<RefCell<Self>> {
        Self::new_internal(component, ancestor, false)
    }

    fn new_internal<C: Component + 'static>(
        component: C,
        ancestor: Node,
        to_patch: bool,
    ) -> Rc<RefCell<Self>> {
        let component_box = Box::new(component) as Box<dyn AnyComponent>;
        let behavior = AnyComponentBehavior::new();

        let node = Self {
            component: component_box,
            depth: 0,
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
        } else {
            node_rc.borrow_mut().view();
        }

        node_rc
    }

    fn view(&mut self) {
        self.vdom = Some(self.component.view(&mut self.behavior));
    }

    pub(crate) fn view_and_patch(&mut self) {
        let mut new_vdom = self.component.view(&mut self.behavior);
        new_vdom.patch(self.vdom.take(), &self.ancestor);
        self.vdom = Some(new_vdom);
        self.to_rerender = false;
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
