use wal_macros::rsx;

include!("../utils/non_display_struct.rs");

fn main() {
    rsx! { <div attr></div> };
    rsx! { <div attr=></div> };
    rsx! { <div attr "value"></div> };
    rsx! { <div attr="key1" attr="key2"></div> };
    rsx! { <div attr1= attr2="key2"></div> };
    rsx! { <div attr={NonDisplayStruct}></div> };
}
