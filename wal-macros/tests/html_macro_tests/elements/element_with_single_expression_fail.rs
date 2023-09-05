use wal_macros::html;

include!("../utils/non_display_struct.rs");

fn main() {
    html! { <div>()</div> };
    html! { <div>{ }</div> };
    html! { <div> { invalid_reference } </div> };
    html! { <div> String::from("Hello world!") </div> };
    let val = "Hello world!";
    html! { <div> val </div> };
    let t = NonDisplayStruct;
    html! { <div> { t } </div> };
    html! { <div> { NonDisplayStruct } </div> };
    let node = || NonDisplayStruct;
    html! { <div> { node() } </div> };
}
