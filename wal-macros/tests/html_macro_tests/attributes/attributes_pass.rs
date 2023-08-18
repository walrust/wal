use wal_macros::html;

fn main() {
    html! { <div key="value" /> };
    html! { <div key="value" other_attr1="val" other_attr2="val"></div> };
    html! { <div key="value" other_attr1="val" other_attr2="val"/> };
    html! { <input key="value" other_attr1="val" other_attr2="val"/> };
    html! { <key="value"></> };
    html! { <div attr={1 + 2}></div> };
    html! { <div attr={let x = 5; x}></div> };
    html! { <input key={"value"} other_attr1="val" other_attr2={"val"}/> };
}
