use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};
use wal_macros::html;

include!("../utils/custom_components/custom_component_props_struct_without_default.rs");

fn main() {
    html! { <CustomComponentPropsStructWithoutDefault /> };
    html! { <CustomComponentPropsStructWithoutDefault key="key" />};
}
