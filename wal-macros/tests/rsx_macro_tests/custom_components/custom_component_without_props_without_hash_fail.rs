use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};
use wal_macros::rsx;

include!("../utils/custom_components/custom_component_props_struct_without_hash.rs");

fn main() {
    rsx! { <CustomComponentPropsStructWithoutHash /> };
}
