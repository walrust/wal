use std::{any::TypeId, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use super::Component;


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
        where C: Component + 'static
    {
        let ty = TypeId::of::<C>();
        
        let mut hasher = DefaultHasher::new();
        props.hash(&mut hasher);
        let hash = hasher.finish();

        let boxik = Box::into_raw(Box::new(props));
        let raw: *mut Void = unsafe {
            std::mem::transmute(boxik)
        };

        AnyProps { ty, hash, raw }
    }

    pub fn into<C>(self) -> C::Properties 
           where C: Component + 'static 
    {
        // Check if typeid matches expected component
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

#[cfg(test)]
mod tests {
    use crate::component::{Component, any_props::AnyProps};

    #[derive(Debug, Hash, PartialEq)]
    enum E { X }
    #[derive(Debug, Hash, PartialEq)]
    struct Props {
        i: i32,
        s: String,
        e: E,
    }
    struct MyComp;
    impl Component for MyComp {
        type Message = ();
        type Properties = Props;
        fn new(_props: Self::Properties) -> Self { todo!() }
        fn view(&self) -> crate::virtual_dom::VNode { todo!() }
        fn update(&mut self, _message: Self::Message) -> bool { todo!() }
    }
    
    #[test]
    fn from_and_into() {
        let props = Props { i: 0, s: "".to_string(), e: E::X };
        let any_props = AnyProps::from::<MyComp>(props);
        let props = any_props.into::<MyComp>();
        assert!(props.i == 0 && props.s == "".to_string() && props.e == E::X);
    }

    #[test]
    fn partial_eq() {
        let props1 = Props { i: 0, s: "".to_string(), e: E::X };
        let props2 = Props { i: 0, s: "".to_string(), e: E::X };
        assert_eq!(props1, props2);

        let any_props1 = AnyProps::from::<MyComp>(props1);
        let any_props2 = AnyProps::from::<MyComp>(props2);
        assert_eq!(any_props1, any_props2);

        let props1 = any_props1.into::<MyComp>();
        let props2 = any_props2.into::<MyComp>();
        assert_eq!(props1, props2);
    }

    #[test]
    fn partial_ne() {
        let props1 = Props { i: 0, s: "".to_string(), e: E::X };
        let props2 = Props { i: 1, s: "".to_string(), e: E::X };
        assert_ne!(props1, props2);

        let any_props1 = AnyProps::from::<MyComp>(props1);
        let any_props2 = AnyProps::from::<MyComp>(props2);
        assert_ne!(any_props1, any_props2);

        let props1 = any_props1.into::<MyComp>();
        let props2 = any_props2.into::<MyComp>();
        assert_ne!(props1, props2);
        //html!{ <MyComp props= { MyComp::Properties{ x = 1, y = 2 } }/> }
        //let x = Props { };
    }

    struct MyComp2;
    impl Component for MyComp2 {
        type Message = ();
        type Properties = Props;
        fn new(_props: Self::Properties) -> Self { todo!() }
        fn view(&self) -> crate::virtual_dom::VNode { todo!() }
        fn update(&mut self, _message: Self::Message) -> bool { todo!() }
    }
    #[test]
    fn partial_ne_different_component() {
        let props1 = Props { i: 0, s: "".to_string(), e: E::X };
        let props2 = Props { i: 0, s: "".to_string(), e: E::X };
        assert_eq!(props1, props2);

        let any_props1 = AnyProps::from::<MyComp>(props1);
        let any_props2 = AnyProps::from::<MyComp2>(props2);
        assert_eq!(any_props1.hash, any_props2.hash);
        assert_ne!(any_props1, any_props2);

        let props1 = any_props1.into::<MyComp>();
        let props2 = any_props2.into::<MyComp2>();
        assert_eq!(props1, props2);
    }
}