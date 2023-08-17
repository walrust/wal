use wal_macros::html;

fn main() {
    html! { <> };
    html! { <><> };
    html! { <><div></> };
    html! { <></div></> };
    html! { </> };
    html! { <> invalid_reference </> };
    html! { <key="key"></key> };
    html! { <key=></> };
    html! { <key="key1" key="key2"></> };
    html! { <other_attr="attr value"></> };
}
