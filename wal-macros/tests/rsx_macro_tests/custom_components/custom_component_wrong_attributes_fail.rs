use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};
use wal_macros::rsx;

include!("../utils/custom_components/custom_component_props_i32.rs");

fn main() {
    rsx! { <CustomComponentPropsI32 attr = 5 /> };
    rsx! { <CustomComponentPropsI32 props = 1 attr = 5 /> };
    rsx! { <CustomComponentPropsI32 key = "key" attr = 5 /> };
    rsx! { <CustomComponentPropsI32 props = 1 attr = 5 key="key" /> };
    rsx! { <CustomComponentPropsI32 props = 0 props = 1 /> };
    rsx! { <CustomComponentPropsI32 props = 0 key = "key" props = 1 /> };
    rsx! { <CustomComponentPropsI32 key = "key1" key = "key2" /> };
    rsx! { <CustomComponentPropsI32 key = "key1" props = 0 key = "key2" /> };
    rsx! { <CustomComponentPropsI32 props /> };
    rsx! { <CustomComponentPropsI32 props = /> };
    rsx! { <CustomComponentPropsI32 props = key = "key" /> };
    rsx! { <CustomComponentPropsI32 key /> };
    rsx! { <CustomComponentPropsI32 key = /> };
    rsx! { <CustomComponentPropsI32 key = props = 1 /> };
}
