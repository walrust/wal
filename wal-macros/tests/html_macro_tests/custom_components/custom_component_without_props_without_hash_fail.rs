use wal::{
    component::{component_node::ComponentBehavior, Component},
    virtual_dom::VNode,
};
use wal_macros::html;

include!("../utils/custom_components/custom_component_props_struct_without_hash.rs");

fn main() {
    html! { <CustomComponentPropsStructWithoutHash /> };
}
