struct CustomComponentPropsStructWithDefaultAndHash;

#[derive(Default, Hash)]
struct PropsWithDefaultAndHash;

impl Component for CustomComponentPropsStructWithDefaultAndHash {
    type Properties = PropsWithDefaultAndHash;
    type Message = ();

    fn new(_props: Self::Properties) -> Self {
        unimplemented!();
    }

    fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
        unimplemented!();
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        unimplemented!()
    }
}
