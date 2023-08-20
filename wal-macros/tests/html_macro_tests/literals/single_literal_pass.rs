use wal_macros::html;

fn main() {
    html! {};
    html! { "" };
    html! { "Hello world!" };
    html! { 'a' };
    html! { 15 };
    html! { 15.0 };
}
