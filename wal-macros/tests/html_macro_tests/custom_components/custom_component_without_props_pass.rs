use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::{VComponent, VNode},
};
use wal_macros::html;

include!("../utils/custom_components/custom_component_props_i32.rs");
include!("../utils/custom_components/custom_component_props_struct_with_default_and_hash.rs");

fn main() {
    custom_component_props_i32();
    custom_component_props_struct_with_default();

    custom_component_props_i32_with_key();
    custom_component_props_struct_with_default_with_key();
}

fn custom_component_props_i32() {
    let html = html! { <CustomComponentPropsI32 /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(
            <CustomComponentPropsI32 as Component>::Properties::default()
        ))
    );
}

fn custom_component_props_struct_with_default() {
    let html = html! { <CustomComponentPropsStructWithDefaultAndHash /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<
            CustomComponentPropsStructWithDefaultAndHash,
        >(
            <CustomComponentPropsStructWithDefaultAndHash as Component>::Properties::default()
        ))
    );
}

fn custom_component_props_i32_with_key() {
    let html = html! { <CustomComponentPropsI32 key="key" /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(
            <CustomComponentPropsI32 as Component>::Properties::default()
        ))
    );
}

fn custom_component_props_struct_with_default_with_key() {
    let html = html! { <CustomComponentPropsStructWithDefaultAndHash key="key" /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<
            CustomComponentPropsStructWithDefaultAndHash,
        >(
            <CustomComponentPropsStructWithDefaultAndHash as Component>::Properties::default()
        ))
    );
}
