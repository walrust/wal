use wal_macros::html;

include!("../utils/non_display_struct.rs");

fn main() {
    html! { <div attr></div> };
    html! { <div attr=></div> };
    html! { <div attr "value"></div> };
    html! { <div attr="key1" attr="key2"></div> };
    html! { <div attr1= attr2="key2"></div> };
    html! { <div attr={NonDisplayStruct}></div> };
}
