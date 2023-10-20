use wal::{
    component::{component_node::ComponentBehavior, Component},
    virtual_dom::VNode,
};
use wal_macros::html;

include!("../utils/custom_components/custom_component_props_i32.rs");

fn main() {
    html! { <CustomComponentPropsI32 attr = 5 /> };
    html! { <CustomComponentPropsI32 props = 1 attr = 5 /> };
    html! { <CustomComponentPropsI32 key = "key" attr = 5 /> };
    html! { <CustomComponentPropsI32 props = 1 attr = 5 key="key" /> };
    html! { <CustomComponentPropsI32 props = 0 props = 1 /> };
    html! { <CustomComponentPropsI32 props = 0 key = "key" props = 1 /> };
    html! { <CustomComponentPropsI32 key = "key1" key = "key2" /> };
    html! { <CustomComponentPropsI32 key = "key1" props = 0 key = "key2" /> };
    html! { <CustomComponentPropsI32 props /> };
    html! { <CustomComponentPropsI32 props = /> };
    html! { <CustomComponentPropsI32 props = key = "key" /> };
    html! { <CustomComponentPropsI32 key /> };
    html! { <CustomComponentPropsI32 key = /> };
    html! { <CustomComponentPropsI32 key = props = 1 /> };
}
