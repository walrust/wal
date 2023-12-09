use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};
use wal_rsx::rsx;

include!("../utils/custom_components/custom_component_props_struct_without_default.rs");

fn main() {
    rsx! { <CustomComponentPropsStructWithoutDefault /> };
    rsx! { <CustomComponentPropsStructWithoutDefault key="key" />};
}
