use wal_macros::html;

include!("../utils/non_display_struct.rs");

fn main() {
    html! { () };
    html! { invalid_reference };
    html! { NonDisplayStruct };
    let node = || NonDisplayStruct;
    html! { node() };
}
