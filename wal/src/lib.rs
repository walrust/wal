use wal_component::{Component, App};


pub fn start(root: &impl Component) {
   //let app = App {
   //    root
   //};
}

#[cfg(test)]
mod tests {
    use wal_component::{Component, Html};
    use wal_vdom::virtual_dom::{VText, VNode};

    use crate::start;

    #[test]
    fn test_start() {
        struct Comp {

        }
        impl Component for Comp {
            type Message = ();
            type Properties = ();

            fn new(prop: Self::Properties) -> Self {
                todo!()
            }

            fn view(&self) -> VNode {
                todo!()
            }

            fn update(&mut self, message: Self::Message) -> bool {
                todo!()
            }
        }
        let c = Comp::new(());
        start(&c);
    }
}