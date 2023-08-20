use wal_macros::html;

fn main() {
    html! { <div attr></> };
    html! { <div attr=></> };
    html! { <div attr "value"></> };
    html! { <div attr="key1" attr="key2"></> };
    html! { <div attr1= attr2="key2"></> };
    // test situations where attribute value is not correct
}
