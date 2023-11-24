use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::{VComponent, VNode},
};
use wal_macros::rsx;

include!("../utils/custom_components/custom_component_props_i32.rs");
include!("../utils/custom_components/custom_component_props_struct.rs");
include!("../utils/custom_components/custom_component_props_tuple_struct.rs");

fn main() {
    custom_component_props_i32();
    custom_component_props_i32_proprs_reference();

    custom_component_props_tuple_struct();
    custom_component_props_struct();

    custom_component_props_tuple_struct_with_struct_expression();
    custom_component_props_struct_with_struct_expression();

    custom_component_props_i32_with_key();
    custom_component_props_i32_proprs_reference_with_key();

    custom_component_props_tuple_struct_with_key();
    custom_component_props_struct_with_key();

    custom_component_props_tuple_struct_with_struct_expression_with_key();
    custom_component_props_struct_with_struct_expression_with_key();
}

fn custom_component_props_i32() {
    let html = rsx! { <CustomComponentPropsI32 props = 1 /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(1, None))
    );
}

fn custom_component_props_i32_proprs_reference() {
    let props_val = 1;
    let html = rsx! { <CustomComponentPropsI32 props = {props_val} /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(props_val, None))
    );
}

fn custom_component_props_tuple_struct() {
    let props_val = 1;
    let html =
        rsx! { <CustomComponentPropsTupleStruct props = {PropsTupleStruct::new(props_val)} /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsTupleStruct>(
            PropsTupleStruct::new(props_val),
            None
        ))
    );
}

fn custom_component_props_struct() {
    let props_val = 1;
    let html = rsx! { <CustomComponentPropsStruct props = {PropsStruct::new(props_val)} /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsStruct>(
            PropsStruct::new(props_val),
            None
        ))
    );
}

fn custom_component_props_tuple_struct_with_struct_expression() {
    let props_val = 1;
    let html = rsx! { <CustomComponentPropsTupleStruct props = {PropsTupleStruct(props_val)} /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsTupleStruct>(
            PropsTupleStruct(props_val),
            None
        ))
    );
}

fn custom_component_props_struct_with_struct_expression() {
    let props_val = 1;
    let html = rsx! { <CustomComponentPropsStruct props = PropsStruct{ x: props_val } /> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsStruct>(
            PropsStruct { x: props_val },
            None
        ))
    );
}

fn custom_component_props_i32_with_key() {
    let html = rsx! { <CustomComponentPropsI32 props = 1 key = "key"/> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(
            1,
            Some("key".to_string())
        ))
    );
}

fn custom_component_props_i32_proprs_reference_with_key() {
    let props_val = 1;
    let html = rsx! { <CustomComponentPropsI32 props = {props_val} key = "key"/> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsI32>(
            props_val,
            Some("key".to_string())
        ))
    );
}

fn custom_component_props_tuple_struct_with_key() {
    let props_val = 1;
    let html = rsx! { <CustomComponentPropsTupleStruct props = {PropsTupleStruct::new(props_val)} key = "key"/> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsTupleStruct>(
            PropsTupleStruct::new(props_val),
            Some("key".to_string())
        ))
    );
}

fn custom_component_props_struct_with_key() {
    let props_val = 1;
    let html =
        rsx! { <CustomComponentPropsStruct props = {PropsStruct::new(props_val)} key = "key"/> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsStruct>(
            PropsStruct::new(props_val),
            Some("key".to_string())
        ))
    );
}

fn custom_component_props_tuple_struct_with_struct_expression_with_key() {
    let props_val = 1;
    let html = rsx! { <CustomComponentPropsTupleStruct props = {PropsTupleStruct(props_val)} key = "key"/> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsTupleStruct>(
            PropsTupleStruct(props_val),
            Some("key".to_string())
        ))
    );
}

fn custom_component_props_struct_with_struct_expression_with_key() {
    let props_val = 1;
    let html =
        rsx! { <CustomComponentPropsStruct props = PropsStruct{ x: props_val } key = "key"/> };
    assert_eq!(
        html,
        VNode::Component(VComponent::new::<CustomComponentPropsStruct>(
            PropsStruct { x: props_val },
            Some("key".to_string())
        ))
    );
}
