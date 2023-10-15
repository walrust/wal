use crate::virtual_dom::VNode;
use std::{
    cell::RefCell,
    fmt,
    marker::PhantomData,
    mem,
    rc::{Rc, Weak},
};
use web_sys::Node;

use super::{
    callback::Callback,
    component::{AnyComponent, Component},
    scheduler::Scheduler,
};

pub struct AnyComponentNode {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    depth: u32,
    to_rerender: bool,
    behavior: Rc<AnyComponentBehavior>,
    pub vdom: VNode,
    ancestor: Node,
    vdom_observer: Rc<RefCell<VDomObserver>>,
    to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
}

impl AnyComponentNode {
    pub fn new<C: Component + 'static>(component: C, ancestor: Node) -> Self {
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
            to_rerender: false,
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

        Rc::try_unwrap(node_rc)
            .expect("Failed to unwrap the Rc")
            .into_inner()
    }

    fn rerender_notify(&mut self) {
        if !self.to_rerender {
            self.to_rerender = true;
            Scheduler::add_rerender_message(
                self.component.clone(),
                self.behavior.clone(),
                self.vdom_observer.clone(),
                self.depth,
            );
        }
    }

    fn vdom_notify(&mut self, mut new_vdom: VNode) {
        mem::swap(&mut new_vdom, &mut self.vdom);
        self.vdom.patch(Some(new_vdom), &self.ancestor);
    }

    pub fn patch(&mut self, last_component_node: Box<AnyComponentNode>, ancestor: &Node) {
        self.vdom.patch(Some(last_component_node.vdom), ancestor);
    }
}

impl fmt::Debug for AnyComponentNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "AnyComponentNode".fmt(f)
    }
}

pub struct AnyComponentBehavior {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    rerender_observer: Rc<RefCell<ToRerenderObserver>>,
}

impl AnyComponentBehavior {
    pub fn new(
        component: Rc<RefCell<Box<dyn AnyComponent>>>,
        rerender_observer: Rc<RefCell<ToRerenderObserver>>,
    ) -> Self {
        Self {
            component,
            rerender_observer,
        }
    }
}

pub struct ComponentBehavior<C: Component> {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    rerender_observer: Rc<RefCell<ToRerenderObserver>>,
    _pd: PhantomData<C>,
}

impl<C: Component> ComponentBehavior<C> {
    pub fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static,
    {
        let component = self.component.clone();
        let rerender_observer = self.rerender_observer.clone();
        Callback::new(move |data| {
            let message = wrapper(data);
            Scheduler::add_update_message(
                component.clone(),
                Box::new(message),
                rerender_observer.clone(),
            );
        })
    }
}

impl<C: Component> From<&AnyComponentBehavior> for ComponentBehavior<C> {
    fn from(value: &AnyComponentBehavior) -> Self {
        Self {
            component: value.component.clone(),
            rerender_observer: value.rerender_observer.clone(),
            _pd: PhantomData,
        }
    }
}

pub struct VDomObserver {
    component_node: Option<Weak<RefCell<AnyComponentNode>>>,
}

impl VDomObserver {
    fn new() -> Self {
        Self {
            component_node: None,
        }
    }

    fn set_observer(&mut self, component_node: Rc<RefCell<AnyComponentNode>>) {
        self.component_node = Some(Rc::downgrade(&component_node));
    }

    pub fn notify(&self, new_vdom: VNode) {
        if let Some(weak_ref) = &self.component_node {
            if let Some(component_node_ref) = weak_ref.upgrade() {
                let mut component_node = component_node_ref.borrow_mut();
                component_node.vdom_notify(new_vdom);
            } else {
                panic!("VDomObserver's reference to AnyComponentNode has expired");
            }
        } else {
            panic!("VDomObserver is not attached to a AnyComponentNode");
        }
    }
}

pub struct ToRerenderObserver {
    component_node_data: Option<Weak<RefCell<AnyComponentNode>>>,
}

impl ToRerenderObserver {
    fn new() -> Self {
        Self {
            component_node_data: None,
        }
    }

    fn set_observer(&mut self, component_node_data: Rc<RefCell<AnyComponentNode>>) {
        self.component_node_data = Some(Rc::downgrade(&component_node_data));
    }

    pub fn notify(&self) {
        if let Some(weak_ref) = &self.component_node_data {
            if let Some(any_component_node_data) = weak_ref.upgrade() {
                any_component_node_data.borrow_mut().rerender_notify();
            } else {
                panic!("RerenderObserver's reference to AnyComponentNode has expired");
            }
        } else {
            panic!("RerenderObserver is not attached to AnyComponentNode");
        }
    }
}
