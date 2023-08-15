use wal_macros::html;

fn main() {
    html! { <div>()</div> };
    html! { <div>{ }</div> };
    html! { <div> invalid_reference </div> };
}
