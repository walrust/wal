use std::{
    any::Any,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::virtual_dom::VNode;

use super::{
    callback::Callback,
    component::{AnyComponent, Component},
    scheduler::Scheduler,
};

pub struct AnyComponentNode {
    data: AnyComponentNodeData,
    rerender_observer: RerenderObserver,
}

pub struct AnyComponentNodeData {
    component: Arc<Mutex<Box<dyn AnyComponent>>>,
    depth: u32,
    to_rerender: bool,
    behavior: Arc<AnyComponentBehavior>,
    any_component_node_vdom: AnyComponentNodeVDom,
}

pub struct AnyComponentNodeVDom {
    vdom: VNode,
    children: Vec<AnyComponentNode>,
}

impl AnyComponentNodeData {
    fn rerender_notify(&mut self) {
        if !self.to_rerender {
            self.to_rerender = true;
            Scheduler::add_rerender_message(
                self.component.clone(),
                self.behavior.clone(),
                self.depth,
            );
        }
        // maybe we should get notified with the new VNode?
    }
}

impl AnyComponentNode {
    pub fn new<C: Component + 'static>(component: C) -> Self {
        let component = Box::new(component) as Box<dyn AnyComponent>;
        Self::new_any(component, 0)
    }

    fn new_any(component: Box<dyn AnyComponent>, depth: u32) -> Self {
        let component = Arc::new(Mutex::new(component));
        let rerender_observer = RerenderObserver::new();
        let rerenderer_observer = Arc::new(rerender_observer);
        let behavior = AnyComponentBehavior::new(component.clone(), rerenderer_observer.clone());
        let vdom = component.lock().unwrap().view(&behavior);
        let mut children = Vec::new();
        Self::generate_children(&mut children, &vdom, depth + 1);
        let any_component_node_vdom = AnyComponentNodeVDom { vdom, children };
        let any_component_node_data = AnyComponentNodeData {
            component,
            depth,
            to_rerender: false,
            behavior: Arc::new(behavior),
            any_component_node_vdom,
        };
        rerender_observer.set_observer(any_component_node_data);
        Self {
            data: any_component_node_data,
            rerender_observer,
        }
    }

    // pub fn update(&mut self, message: Box<dyn Any>) -> bool {
    //     self.component.lock().unwrap().update(message)
    // }

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
    component: Arc<Mutex<Box<dyn AnyComponent>>>,
    rerender_observer: Arc<RerenderObserver>,
}

impl AnyComponentBehavior {
    pub fn new(
        component: Arc<Mutex<Box<dyn AnyComponent>>>,
        rerender_observer: Arc<RerenderObserver>,
    ) -> Self {
        Self {
            component,
            rerender_observer,
        }
    }
}

pub struct ComponentBehavior<C: Component> {
    component: Arc<Mutex<Box<dyn AnyComponent>>>,
    rerender_observer: Arc<RerenderObserver>,
    _marker: PhantomData<C>,
}

impl<C: Component> ComponentBehavior<C> {
    // pub fn new(component: Arc<Mutex<Box<dyn AnyComponent>>>) -> Self {
    //     Self {
    //         component,
    //         _marker: PhantomData,
    //     }
    // }

    pub fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static,
    {
        let component = self.component.clone();
        let rerender_observer = self.rerender_observer.clone();
        Callback::new(move |data| {
            let message = wrapper(data);
            Scheduler::add_update_message(component.clone(), Box::new(message), rerender_observer);
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

pub trait Observer {
    fn notify(&self);
}

pub struct RerenderObserver {
    component_node_data: Arc<Mutex<Option<AnyComponentNodeData>>>,
}

impl Observer for RerenderObserver {
    fn notify(&self) {
        let any_component_node_data = self.component_node_data.lock().unwrap();
        if let Some(any_component_node_data) = &*any_component_node_data {
            any_component_node_data.rerender_notify();
        } else {
            panic!("RerenderObserver is not attached to a component node");
        }
    }
}

impl RerenderObserver {
    fn new() -> Self {
        Self {
            component_node_data: Arc::new(Mutex::new(None)),
        }
    }

    fn set_observer(&self, component_node_data: AnyComponentNodeData) {
        let mut any_component_node_data = self.component_node_data.lock().unwrap();
        *any_component_node_data = Some(component_node_data);
    }
}
