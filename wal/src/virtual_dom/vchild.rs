use std::fmt;
use crate::component::any_props::AnyProps;
use super::VNode;

// Example implementation of creting VChild
// pub fn f<C>(props: C::Properties) -> VChild 
//     where C: Component + 'static
// {
//     let props = AnyProps::from::<C>(props);
//     let _generator = Box::new(|props: AnyProps| {
//         let props = props.into::<C>();
//         C::new(props).view()
//     });

//     VChild { props, _generator }
// }

type GeneratorParameters = AnyProps;
type Generator = Box<dyn FnOnce(GeneratorParameters) -> VNode + 'static>;
pub struct VChild
{
    props: AnyProps,
    _generator: Generator,
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
