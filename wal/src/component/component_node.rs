use crate::virtual_dom::VNode;
use gloo::console::log;
use std::{cell::RefCell, fmt, marker::PhantomData, mem, rc::Rc};
use web_sys::Node;

use super::{
    callback::Callback,
    component::{AnyComponent, Component},
    scheduler::Scheduler,
};

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

    fn rerender_notify(&mut self) {
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

    fn vdom_notify(&mut self, mut new_vdom: VNode) {
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
    component_node: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl VDomObserver {
    fn new() -> Self {
        Self {
            component_node: None,
        }
    }

    fn set_observer(&mut self, component_node: Rc<RefCell<AnyComponentNode>>) {
        self.component_node = Some(component_node);
    }

    pub fn notify(&self, new_vdom: VNode) {
        if let Some(any_componend_node) = &self.component_node {
            let mut any_component_node = any_componend_node.borrow_mut();
            any_component_node.vdom_notify(new_vdom);
        } else {
            log!("VDomObserver is not attached to a AnyComponentNode");
            panic!("VDomObserver is not attached to a AnyComponentNode");
        }
    }
}

pub struct ToRerenderObserver {
    any_component_node: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl ToRerenderObserver {
    fn new() -> Self {
        Self {
            any_component_node: None,
        }
    }

    fn set_observer(&mut self, any_component_node: Rc<RefCell<AnyComponentNode>>) {
        self.any_component_node = Some(any_component_node);
    }

    pub fn notify(&self) {
        if let Some(any_component_node) = &self.any_component_node {
            any_component_node.borrow_mut().rerender_notify();
        } else {
            log!("RerenderObserver is not attached to AnyComponentNode");
            panic!("RerenderObserver is not attached to AnyComponentNode");
        }
    }
}
