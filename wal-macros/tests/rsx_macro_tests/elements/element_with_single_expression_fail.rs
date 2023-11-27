use wal_macros::rsx;

include!("../utils/non_display_struct.rs");

fn main() {
    rsx! { <div>()</div> };
    rsx! { <div>{ }</div> };
    rsx! { <div> { invalid_reference } </div> };
    rsx! { <div> String::from("Hello world!") </div> };
    let val = "Hello world!";
    rsx! { <div> val </div> };
    let t = NonDisplayStruct;
    rsx! { <div> { t } </div> };
    rsx! { <div> { NonDisplayStruct } </div> };
    let node = || NonDisplayStruct;
    rsx! { <div> { node() } </div> };
}
