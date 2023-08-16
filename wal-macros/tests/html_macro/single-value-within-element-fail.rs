use wal_macros::html;

fn main() {
    html! { <div>()</div> };
    html! { <div>{ }</div> };
    html! { <div> invalid_reference </div> };

    html! { <div> String::from("Hello world!") </div> };
    let val = "Hello world!";
    html! { <div> val </div> };
    let t = TestStruct { field: 15 };
    html! { <div> t </div> };
    html! { <div> TestStruct { field: 15 } </div> };
}
