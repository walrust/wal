use std::fmt;
use crate::component::any_props::AnyProps;
use super::VNode;


type GeneratorParameters = AnyProps;
pub struct VChild
{
    props: AnyProps,
    _generator: Box<dyn FnOnce(GeneratorParameters) -> VNode + 'static>,
}

impl fmt::Debug for VChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VChild").field("props", &self.props).finish()
    }
}

impl PartialEq for VChild {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
