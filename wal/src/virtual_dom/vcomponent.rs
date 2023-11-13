use web_sys::Node;

use crate::{
    component::{component_node::AnyComponentNode, Component},
    utils::debug,
};

use std::{
    any::Any,
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

use super::VNode;

pub(crate) type PropertiesHash = u64;
pub(crate) type AnyProps = Option<Box<dyn Any>>;
pub(crate) type ComponentNodeGenerator =
    Box<dyn Fn(AnyProps, &Node) -> Rc<RefCell<AnyComponentNode>> + 'static>;

pub struct VComponent {
    props: AnyProps,
    hash: PropertiesHash,
    generator: ComponentNodeGenerator,
    _key: Option<String>, // TODO: add logic for key attribute

    // Sth stinks here
    pub comp: Option<Rc<RefCell<AnyComponentNode>>>,
}

impl VComponent {
    pub fn new<C>(props: C::Properties, key: Option<String>) -> VComponent
    where
        C: Component + 'static,
    {
        let hash = Self::calculate_hash(&props);
        let generator = Box::new(Self::generator::<C>);
        VComponent {
            props: Some(Box::new(props)),
            generator,
            hash,
            _key: key,
            comp: None,
        }
    }

    fn calculate_hash<T: Hash>(props: &T) -> PropertiesHash {
        let mut hasher = DefaultHasher::new();
        props.hash(&mut hasher);
        hasher.finish()
    }

    fn generator<C: Component + 'static>(
        props: AnyProps,
        ancestor: &Node,
    ) -> Rc<RefCell<AnyComponentNode>> {
        let props = props
            .unwrap()
            .downcast::<C::Properties>()
            .expect("Trying to unpack others component props");

        AnyComponentNode::new(C::new(*props), ancestor.clone())
    }

    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        debug::log("Patching component");
        let mut old_virt: Option<VComponent> = None;

        match last {
            Some(VNode::Component(vcomp)) => {
                debug::log("\tComparing two components");
                old_virt = Some(vcomp);
            }
            Some(VNode::Element(v)) => {
                debug::log("\tNew component over element");
                v.erase();
            }
            Some(VNode::Text(v)) => {
                debug::log("\tNew component over text");
                v.erase();
            }
            None => {
                debug::log("\tCreating the comp for the first time");
            }
            Some(VNode::List(v)) => {
                debug::log("\tNew component over list");
                v.erase();
            }
        }

        self.render(old_virt, ancestor);
    }

    pub fn erase(&self) {
        if let Some(node) = self.comp.as_ref() {
            debug::log("Erasing vcomponent, feels kinda fucking sus");
            node.borrow_mut().vdom.as_ref().unwrap().erase();
        }
    }

    fn render(&mut self, last: Option<VComponent>, ancestor: &Node) {
        match last {
            Some(old_vcomp) if old_vcomp.hash == self.hash => {
                debug::log("\t\tHashes are equal");
                self.comp = old_vcomp.comp.clone();
            }
            Some(old_vcomp) => {
                debug::log("\t\tHashes differ");
                let any_component_node_rc = (self.generator)(self.props.take(), ancestor);
                {
                    let mut any_component_node = any_component_node_rc.borrow_mut();
                    any_component_node.patch(old_vcomp.comp.clone(), ancestor);
                }
                self.comp = Some(any_component_node_rc);
            }
            None => {
                debug::log("\t\tThere was no component before");
                let any_component_node_rc = (self.generator)(self.props.take(), ancestor);
                {
                    let mut any_component_node = any_component_node_rc.borrow_mut();
                    any_component_node.patch(None, ancestor);
                }
                self.comp = Some(any_component_node_rc);
            }
        }
    }
}

impl fmt::Debug for VComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VComponent")
            .field("props", &self.props)
            .field("hash", &self.hash)
            .field("comp", &self.comp)
            .finish()
    }
}

impl PartialEq for VComponent {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
            && match (&self.props, &other.props) {
                (Some(self_props), Some(other_props)) => {
                    self_props.type_id() == other_props.type_id()
                }
                (None, None) => true,
                _ => false,
            }
    }
}
