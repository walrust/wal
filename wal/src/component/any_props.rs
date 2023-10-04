use std::{
    any::TypeId,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::component::Component;

pub(crate) struct Void;
pub(crate) type PropertiesHash = u64;

#[derive(Debug)]
pub struct AnyProps {
    ty: TypeId,
    hash: PropertiesHash,
    raw: *mut Void,
}

impl AnyProps {
    pub fn from<C>(props: C::Properties) -> Self
    where
        C: Component + 'static,
    {
        let ty = TypeId::of::<C>();

        let mut hasher = DefaultHasher::new();
        props.hash(&mut hasher);
        let hash = hasher.finish();

        let boxik = Box::into_raw(Box::new(props));
        let raw: *mut Void = unsafe { std::mem::transmute(boxik) };

        AnyProps { ty, hash, raw }
    }

    pub fn into<C>(self) -> C::Properties
    where
        C: Component + 'static,
    {
        let exp_ty = TypeId::of::<C>();
        if exp_ty != self.ty {
            panic!("Trying to unpack others component props");
        }

        unsafe {
            let x: *mut C::Properties = std::mem::transmute(self.raw);
            *Box::from_raw(x)
        }
    }
}

impl PartialEq for AnyProps {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.hash == other.hash
    }
}
