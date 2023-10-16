use gloo::console::log;
use web_sys::Node;

use crate::component::{component::Component, component_node::AnyComponentNode};
use std::{
    any::Any,
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
};

use super::VNode;

pub(crate) type PropertiesHash = u64;
pub(crate) type AnyProps = Option<Box<dyn Any>>;
pub(crate) type ComponentNodeGenerator =
    Box<dyn Fn(AnyProps, &Node) -> Box<AnyComponentNode> + 'static>;
pub struct VComponent {
    props: AnyProps,
    hash: PropertiesHash,
    generator: ComponentNodeGenerator,

    // Sth stinks here
    pub comp: Option<Box<AnyComponentNode>>,
}

impl VComponent {
    pub fn new<C>(props: C::Properties) -> VComponent
    where
        C: Component + 'static,
    {
        let mut hasher = DefaultHasher::new();
        props.hash(&mut hasher);
        let hash = hasher.finish();
        let props = Box::new(props);
        let generator = Box::new(Self::generator::<C>);
        VComponent {
            props: Some(props),
            generator,
            hash,
            comp: None,
        }
    }

    pub fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        log!("Patching component");
        let mut old_virt: Option<VComponent> = None;

        match last {
            Some(VNode::Component { vcomp }) => {
                log!("\tComparing two components");
                old_virt = Some(vcomp);
            }
            Some(VNode::Element { .. }) | Some(VNode::Text { .. }) => {
                log!("\tNew component over element/text");
            }
            None => {
                log!("\tCreating the comp for the first time");
            }
            Some(VNode::List { .. }) => todo!(),
        }

        self.render(old_virt, ancestor);
    }
}

impl VComponent {
    fn render(&mut self, last: Option<VComponent>, ancestor: &Node) {
        match last {
            Some(mut old_vcomp) if old_vcomp.hash == self.hash => {
                log!("\t\tHashes are the same");
                self.comp = old_vcomp.comp.take();
            }
            Some(old_vcomp) => {
                log!("\t\tHashes differ");
                let mut any_component_node = (self.generator)(self.props.take(), ancestor);
                any_component_node.patch(old_vcomp.comp.unwrap(), ancestor);
                self.comp = Some(any_component_node);
            }
            None => {
                log!("\t\tThere was no component before");
                self.comp = Some((self.generator)(self.props.take(), ancestor));
            }
        }
    }

    fn generator<C: Component + 'static>(
        props: AnyProps,
        ancestor: &Node,
    ) -> Box<AnyComponentNode> {
        let props = props
            .unwrap()
            .downcast::<C::Properties>()
            .expect("Trying to unpack others component props");

        let any_node = AnyComponentNode::new(C::new(*props), ancestor.clone());
        Box::new(any_node)
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
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
