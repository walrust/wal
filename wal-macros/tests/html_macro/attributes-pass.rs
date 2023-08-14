use wal_macros::html;

fn main() {
    html! { <div key="value" /> };
    html! { <key="value"></> };
}
