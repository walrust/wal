use wal_macros::html;

fn main() {
    html! { <div> };
    html! { <div> "Hello world!" };
    html! { <div> { String::from("Hello world") } };
    html! { <div><div></div> };
}
