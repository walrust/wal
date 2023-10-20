use crate::virtual_dom::VNode;
use std::{cell::RefCell, fmt, mem, rc::Rc};
use web_sys::Node;

use super::{scheduler::Scheduler, AnyComponent, Component, behavior::AnyComponentBehavior, observer::{VDomObserver, ToRerenderObserver}};

pub struct AnyComponentNode {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    depth: u32,
    to_rerender: Rc<RefCell<bool>>,
    behavior: Rc<AnyComponentBehavior>,
    pub vdom: VNode,
    ancestor: Node,
    vdom_observer: Rc<RefCell<VDomObserver>>,
    to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
}

impl AnyComponentNode {
    pub fn new_root<C: Component + 'static>(component: C, ancestor: Node) -> Rc<RefCell<Self>> {
        let root_rc = Self::new(component, ancestor);
        {
            let mut root = root_rc.borrow_mut();
            root.new_root_patch();
        }
        root_rc
    }

    fn new_root_patch(&mut self) {
        self.vdom.patch(None, &self.ancestor);
    }

    pub fn new<C: Component + 'static>(component: C, ancestor: Node) -> Rc<RefCell<Self>> {
        let component_box = Box::new(component) as Box<dyn AnyComponent>;
        let component_rc = Rc::new(RefCell::new(component_box));
        let to_rerender_observer_rc = Rc::new(RefCell::new(ToRerenderObserver::new()));
        let behavior_rc = Rc::new(AnyComponentBehavior::new(
            component_rc.clone(),
            to_rerender_observer_rc.clone(),
        ));
        let vdom = component_rc.borrow().view(&behavior_rc);
        let vdom_observer_rc = Rc::new(RefCell::new(VDomObserver::new()));

        let node = Self {
            component: component_rc,
            depth: 0,
            to_rerender: Rc::new(RefCell::new(false)),
            behavior: behavior_rc,
            vdom,
            ancestor,
            vdom_observer: vdom_observer_rc,
            to_rerender_observer: to_rerender_observer_rc,
        };

        let node_rc = Rc::new(RefCell::new(node));

        node_rc
            .borrow()
            .to_rerender_observer
            .borrow_mut()
            .set_observer(node_rc.clone());
        node_rc
            .borrow()
            .vdom_observer
            .borrow_mut()
            .set_observer(node_rc.clone());

        node_rc
    }

    pub(crate) fn rerender_notify(&mut self) {
        let mut to_rerender = self.to_rerender.borrow_mut();
        if !*to_rerender {
            *to_rerender = true;
            Scheduler::add_rerender_message(
                self.component.clone(),
                self.behavior.clone(),
                self.vdom_observer.clone(),
                self.to_rerender.clone(),
                self.depth,
            );
        }
    }

    pub(crate) fn vdom_notify(&mut self, mut new_vdom: VNode) {
        mem::swap(&mut new_vdom, &mut self.vdom);
        self.vdom.patch(Some(&new_vdom), &self.ancestor);
    }

    pub fn patch(
        &mut self,
        last_component_node: Option<Rc<RefCell<AnyComponentNode>>>,
        ancestor: &Node,
    ) {
        if let Some(last_component_node) = last_component_node {
            let last_component_node = last_component_node.clone();
            let last_component_node_vdom = &last_component_node.borrow().vdom;

            self.vdom.patch(Some(last_component_node_vdom), ancestor);
        } else {
            self.vdom.patch(None, ancestor)
        }
    }

    pub fn get_dom(&self) -> Option<Node> {
        self.vdom.get_dom()
    }
}

impl fmt::Debug for AnyComponentNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "AnyComponentNode".fmt(f)
    }
}

