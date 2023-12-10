use wal_rsx::rsx;

fn main() {
    rsx! { <div> };
    rsx! { <div> "Hello world!" };
    rsx! { <div> { String::from("Hello world") } };
    rsx! { <div><div></div> };
}
