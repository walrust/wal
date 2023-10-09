use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::virtual_dom::VNode;

use super::{
    callback::Callback,
    component::{AnyComponent, Component},
    scheduler::Scheduler,
};

pub struct AnyComponentNode {
    data: Rc<RefCell<AnyComponentNodeData>>,
    to_rerender_observer: Rc<RefCell<ToRerenderObserver>>,
}

pub struct AnyComponentNodeData {
    component: Rc<RefCell<Box<dyn AnyComponent>>>,
    depth: u32,
    to_rerender: bool,
    behavior: Rc<AnyComponentBehavior>,
    any_component_node_vdom: Rc<RefCell<AnyComponentNodeVDom>>,
    vdom_observer: Rc<RefCell<VDomObserver>>,
}

pub struct AnyComponentNodeVDom {
    vdom: VNode,
    children: Vec<AnyComponentNode>,
}

impl AnyComponentNodeVDom {
    fn vdom_notify(&mut self, new_vdom: VNode) {
        todo!("Logic regarding updating the vdom and updating children, probably in AnyComponentNodeVDom we should hold a reference to dom");
    }
}

impl AnyComponentNodeData {
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
}

impl AnyComponentNode {
    pub fn new<C: Component + 'static>(component: C) -> Self {
        let component = Box::new(component) as Box<dyn AnyComponent>;
        Self::new_any(component, 0)
    }

    fn new_any(component: Box<dyn AnyComponent>, depth: u32) -> Self {
        let component = Rc::new(RefCell::new(component));
        let rerender_observer = Rc::new(RefCell::new(ToRerenderObserver::new()));
        let behavior = Rc::new(AnyComponentBehavior::new(
            component.clone(),
            rerender_observer.clone(),
        ));
        let vdom = component.borrow().view(&behavior);
        let mut children = Vec::new();
        Self::generate_children(&mut children, &vdom, depth + 1);
        let any_component_node_vdom = AnyComponentNodeVDom { vdom, children };
        let any_component_node_vdom = Rc::new(RefCell::new(any_component_node_vdom));
        let vdom_observer = Rc::new(RefCell::new(VDomObserver::new()));
        let any_component_node_data = AnyComponentNodeData {
            component,
            depth,
            to_rerender: false,
            behavior,
            any_component_node_vdom: any_component_node_vdom.clone(),
            vdom_observer: vdom_observer.clone(),
        };
        vdom_observer
            .borrow_mut()
            .set_observer(any_component_node_vdom);
        let any_component_node_data = Rc::new(RefCell::new(any_component_node_data));
        rerender_observer
            .borrow_mut()
            .set_observer(any_component_node_data.clone());
        Self {
            data: any_component_node_data,
            to_rerender_observer: rerender_observer,
        }
    }

    fn generate_children(children: &mut Vec<AnyComponentNode>, vdom: &VNode, current_depth: u32) {
        match vdom {
            VNode::Element { velement } => {
                for child_vdom in &velement.children {
                    Self::generate_children(children, child_vdom, current_depth);
                }
            }
            VNode::List { vlist } => {
                for child_vdom in &vlist.nodes {
                    Self::generate_children(children, child_vdom, current_depth);
                }
            }
            VNode::Child { vchild } => {
                let child_component = vchild.to_any_component();
                children.push(Self::new_any(child_component, current_depth));
            }
            _ => {}
        }
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
    _marker: PhantomData<C>,
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
            _marker: PhantomData,
        }
    }
}

pub struct ToRerenderObserver {
    component_node_data: Option<Rc<RefCell<AnyComponentNodeData>>>,
}

impl ToRerenderObserver {
    fn new() -> Self {
        Self {
            component_node_data: None,
        }
    }

    fn set_observer(&mut self, component_node_data: Rc<RefCell<AnyComponentNodeData>>) {
        self.component_node_data = Some(component_node_data);
    }

    pub fn notify(&self) {
        if let Some(any_component_node_data) = &self.component_node_data {
            any_component_node_data.borrow_mut().rerender_notify();
        } else {
            panic!("RerenderObserver is not attached to a component node");
        }
    }
}

pub struct VDomObserver {
    component_node_vdom: Option<Rc<RefCell<AnyComponentNodeVDom>>>,
}

impl VDomObserver {
    fn new() -> Self {
        Self {
            component_node_vdom: None,
        }
    }

    fn set_observer(&mut self, component_node_vdom: Rc<RefCell<AnyComponentNodeVDom>>) {
        self.component_node_vdom = Some(component_node_vdom);
    }

    pub fn notify(&self, new_vdom: VNode) {
        if let Some(any_component_node_vdom) = &self.component_node_vdom {
            any_component_node_vdom.borrow_mut().vdom_notify(new_vdom);
        } else {
            panic!("VDomObserver is not attached to a component node");
        }
    }
}
