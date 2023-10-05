use crate::component::{component::Component, context_node::ComponentNode};

pub struct App<C: Component> {
    component_node: ComponentNode<C>,
}

impl<C: Component> App<C> {
    pub fn new(root_component: C) -> Self {
        let app = Self {
            component_node: ComponentNode::new(root_component),
        };
        todo!("Create context for root to call view function");
        // let mut vdom = root.view(/* context here */);
        // vdom.patch(None, &document().body().unwrap());
        // App { root, vdom }
        app
    }
}

pub fn start<C: Component>(root_component: C) {
    let app = App::new(root_component);

    // tworzenie doma, kolejek itp

    // main event loop
}
