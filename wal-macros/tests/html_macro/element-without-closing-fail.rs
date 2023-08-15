use wal_macros::html;

fn main() {
    html! { <div> };
    html! { <div> "Hello world!" };
    html! { <div></span> };
    html! { <div><span></div></span> };
    html! { <div><div></div> };
}
