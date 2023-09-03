use wal_macros::html;

fn main() {
    html! { if };
    html! { if true };
    html! { if {} };
    html! { if 1 {} };
    let x = 1;
    html! { if x {} };
    html! { if true {} else };
    html! { if true {} else if };
    html! { if true {} else if true };
    html! { if true {} else if {} };
    html! { if true {} else if true {} else };
    html! { if true { {} } };
    html! { if true { () } };
    html! { if true { invalid_reference } };
}
