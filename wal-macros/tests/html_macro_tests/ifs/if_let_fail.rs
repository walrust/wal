use wal_macros::html;

fn main() {
    html! { if let };
    html! { if let {} };
    html! { if let true {}};
    html! { if let {} {} };
}