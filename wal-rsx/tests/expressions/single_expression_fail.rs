use wal_rsx::rsx;

include!("../utils/non_display_struct.rs");

fn main() {
    rsx! { () };
    rsx! { invalid_reference };
    rsx! { NonDisplayStruct };
    let node = || NonDisplayStruct;
    rsx! { node() };
}
