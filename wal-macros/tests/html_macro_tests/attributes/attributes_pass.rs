use wal_macros::html;

fn main() {
    html! { <div key="value" /> };
    html! { <div key="value" other_attr1="val" other_attr2="val"></div> };
    html! { <div key="value" other_attr1="val" other_attr2="val"/> };
    html! { <input key="value" other_attr1="val" other_attr2="val"/> };
    html! { <key="value"></> };
}
