use std::{any::Any, cell::RefCell, marker::PhantomData, rc::Rc};

use crate::virtual_dom::VNode;

use super::{
    callback::Callback,
    component::{AnyComponent, Component},
};

pub struct ComponentNode<C: Component> {
    component: C,
    vdom: VNode,
    children: Vec<AnyComponentNode>,
    behavior: ComponentBehavior<C>,
}

pub struct AnyComponentNode {
    component: Box<dyn AnyComponent>,
    vdom: VNode,
    children: Vec<AnyComponentNode>,
    behavior: AnyComponentBehavior,
}

impl<C: Component> ComponentNode<C> {
    pub fn new(component: C) -> Self {
        let mut behavior = ComponentBehavior::new();
        let vdom = component.view(&mut behavior);
        let mut component_node = Self {
            component,
            vdom,
            children: Vec::new(),
            behavior,
        };
        generate_children(&mut component_node.children, &component_node.vdom);
        component_node
    }
}

fn generate_children(children: &mut Vec<AnyComponentNode>, vdom: &VNode) {
    match vdom {
        VNode::Element { velement } => {
            for child_vdom in &velement.children {
                generate_children(children, child_vdom);
            }
        }
        VNode::List { vlist } => {
            for child_vdom in &vlist.nodes {
                generate_children(children, child_vdom);
            }
        }
        VNode::Child { vchild } => {
            let child_component = vchild.to_any_component();
            let child_behavior = AnyComponentBehavior::new();
            let child_vdom = child_component.view(&child_behavior);
            let mut child_component_node = AnyComponentNode {
                component: child_component,
                vdom: child_vdom,
                children: Vec::new(),
                behavior: child_behavior,
            };
            generate_children(
                &mut child_component_node.children,
                &child_component_node.vdom,
            );
            children.push(child_component_node);
        }
        _ => {}
    }
}

pub struct AnyComponentBehavior {
    message_queue: Rc<RefCell<Vec<Box<dyn Any>>>>,
}

impl AnyComponentBehavior {
    fn new() -> Self {
        Self {
            message_queue: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

pub struct ComponentBehavior<C: Component> {
    message_queue: Rc<RefCell<Vec<Box<dyn Any>>>>,
    _marker: PhantomData<C>,
}

impl<C: Component> ComponentBehavior<C> {
    pub fn new() -> Self {
        Self {
            message_queue: Rc::new(RefCell::new(Vec::new())),
            _marker: PhantomData,
        }
    }

    pub fn create_callback<IN, F>(&mut self, wrapper: F) -> Callback<IN>
    where
        F: Fn(IN) -> C::Message + 'static,
    {
        let message_queue = self.message_queue.clone();
        Callback::new(move |data| {
            let message = wrapper(data);
            message_queue.borrow_mut().push(Box::new(message));
        })
    }
}

impl<C: Component> From<&AnyComponentBehavior> for ComponentBehavior<C> {
    fn from(value: &AnyComponentBehavior) -> Self {
        Self {
            message_queue: value.message_queue.clone(),
            _marker: PhantomData,
        }
    }
}
