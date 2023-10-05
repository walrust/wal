use crate::component::{any_props::AnyProps, component::AnyComponent};
use std::fmt;

// Example implementation of creting VChild
// pub fn f<C>(props: C::Properties) -> VChild
//     where C: Component + 'static
// {
//     let props = AnyProps::from::<C>(props);
//     let _generator = Box::new(|props: AnyProps| {
//         let props = props.into::<C>();
//         C::new(props)
//     });
//     VChild { props, generator: _generator }
// }

type Generator = Box<dyn Fn(&AnyProps) -> Box<dyn AnyComponent> + 'static>;
pub struct VChild {
    pub props: AnyProps,
    pub generator: Generator,
}

impl VChild {
    pub fn to_any_component(&self) -> Box<dyn AnyComponent> {
        (self.generator)(&self.props)
    }
}

impl fmt::Debug for VChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VChild")
            .field("props", &self.props)
            .finish()
    }
}

impl PartialEq for VChild {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
