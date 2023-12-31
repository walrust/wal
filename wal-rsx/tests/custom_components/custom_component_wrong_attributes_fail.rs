use wal_core::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};
use wal_rsx::rsx;

include!("../utils/custom_components/custom_component_props_i32.rs");
include!("../utils/non_display_struct.rs");

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
    rsx! { <CustomComponentPropsI32 key = {NonDisplayStruct} /> };
    rsx! { <CustomComponentPropsI32 props = "value" /> };
    rsx! { <CustomComponentPropsI32 props = {"value"} /> };
    rsx! { <CustomComponentPropsI32 props = NonDisplayStruct {} /> };
}
