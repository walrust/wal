struct CustomComponentPropsTupleStruct;

#[derive(Hash)]
struct PropsTupleStruct(i32);

impl PropsTupleStruct {
    fn new(x: i32) -> Self {
        Self(x)
    }
}

impl Component for CustomComponentPropsTupleStruct {
    type Properties = PropsTupleStruct;
    type Message = ();

    fn new(_props: Self::Properties) -> Self {
        unimplemented!();
    }

    fn view(&self, _behavior: &mut ComponentBehavior<Self>) -> VNode {
        unimplemented!();
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        unimplemented!()
    }
}
