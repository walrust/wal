use wal_rsx::rsx;

include!("../utils/non_display_struct.rs");

fn main() {
    rsx! { <> };
    rsx! { <><> };
    rsx! { <><div></> };
    rsx! { <></div></> };
    rsx! { </> };
    rsx! { <key="key"></key> };
    rsx! { <key></> };
    rsx! { <key=></> };
    rsx! { <key="key1" key="key2"></> };
    rsx! { <other_attr="attr value"></> };
    rsx! { <key="key" other_attr="attr value"></> };
    rsx! { <other_attr="attr value" key="key"></> };
    rsx! { <other_attr1="attr value" other_attr2="attr val"></> };
    rsx! { <key={NonDisplayStruct}></> };
}
