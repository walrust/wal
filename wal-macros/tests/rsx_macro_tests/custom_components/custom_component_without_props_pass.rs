use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::{VComponent, VNode},
};
use wal_macros::rsx;

include!("../utils/custom_components/custom_component_props_i32.rs");
include!("../utils/custom_components/custom_component_props_struct_with_default_and_hash.rs");

fn main() {
    custom_component_props_i32();
    custom_component_props_struct_with_default();

    custom_component_props_i32_with_key();
    custom_component_props_struct_with_default_with_key();
}

fn custom_component_props_i32() {
    let html = rsx! { <CustomComponentPropsI32 /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(
            <CustomComponentPropsI32 as Component>::Properties::default(),
            None
        ))
    );
}

fn custom_component_props_struct_with_default() {
    let html = rsx! { <CustomComponentPropsStructWithDefaultAndHash /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<
            CustomComponentPropsStructWithDefaultAndHash,
        >(
            <CustomComponentPropsStructWithDefaultAndHash as Component>::Properties::default(),
            None
        ))
    );
}

fn custom_component_props_i32_with_key() {
    let html = rsx! { <CustomComponentPropsI32 key="key" /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(
            <CustomComponentPropsI32 as Component>::Properties::default(),
            Some("key".to_string())
        ))
    );
}

fn custom_component_props_struct_with_default_with_key() {
    let html = rsx! { <CustomComponentPropsStructWithDefaultAndHash key="key" /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<
            CustomComponentPropsStructWithDefaultAndHash,
        >(
            <CustomComponentPropsStructWithDefaultAndHash as Component>::Properties::default(),
            Some("key".to_string())
        ))
    );
}
