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
    component: Arc<Mutex<Box<dyn AnyComponent>>>,
    vdom: VNode,
    children: Vec<AnyComponentNode>,
    behavior: AnyComponentBehavior,
}

impl AnyComponentNode {
    pub fn new<C: Component + 'static>(component: C) -> Self {
        let component = Box::new(component) as Box<dyn AnyComponent>;
        let component = Arc::new(Mutex::new(component));
        let behavior = AnyComponentBehavior::new(component.clone());
        let vdom = component.lock().unwrap().view(&behavior);
        let mut component_node = Self {
            component,
            vdom,
            children: Vec::new(),
            behavior,
        };
        Self::generate_children(&mut component_node.children, &component_node.vdom);
        component_node
    }

    pub fn update(&mut self, message: Box<dyn Any>) -> bool {
        self.component.lock().unwrap().update(message)
    }

    fn generate_children(children: &mut Vec<AnyComponentNode>, vdom: &VNode) {
        match vdom {
            VNode::Element { velement } => {
                for child_vdom in &velement.children {
                    Self::generate_children(children, child_vdom);
                }
            }
            VNode::List { vlist } => {
                for child_vdom in &vlist.nodes {
                    Self::generate_children(children, child_vdom);
                }
            }
            VNode::Child { vchild } => {
                let child_component = vchild.to_any_component();
                let child_component = Arc::new(Mutex::new(child_component));
                let child_behavior = AnyComponentBehavior::new(child_component.clone());
                let child_vdom = child_component.lock().unwrap().view(&child_behavior);
                let mut child_component_node = AnyComponentNode {
                    component: child_component,
                    vdom: child_vdom,
                    children: Vec::new(),
                    behavior: child_behavior,
                };
                Self::generate_children(
                    &mut child_component_node.children,
                    &child_component_node.vdom,
                );
                children.push(child_component_node);
            }
            _ => {}
        }
    }
}

pub struct AnyComponentBehavior {
    component: Arc<Mutex<Box<dyn AnyComponent>>>,
}

impl AnyComponentBehavior {
    fn new(component: Arc<Mutex<Box<dyn AnyComponent>>>) -> Self {
        Self { component }
    }
}

pub struct ComponentBehavior<C: Component> {
    component: Arc<Mutex<Box<dyn AnyComponent>>>,
    _marker: PhantomData<C>,
}

impl<C: Component> ComponentBehavior<C> {
    pub fn new(component: Arc<Mutex<Box<dyn AnyComponent>>>) -> Self {
        Self {
            component,
            _marker: PhantomData,
        }
    }

    pub fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static,
    {
        let component = self.component.clone();
        Callback::new(move |data| {
            let message = wrapper(data);
            Scheduler::add_update_message(component.clone(), Box::new(message));
        })
    }
}

impl<C: Component> From<&AnyComponentBehavior> for ComponentBehavior<C> {
    fn from(value: &AnyComponentBehavior) -> Self {
        Self {
            component: value.component.clone(),
            _marker: PhantomData,
        }
    }
}
