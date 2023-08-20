use wal_macros::html;

fn main() {
    html! { </div> };
    html! { </span> };
    html! { </> };
    html! { <div></span> };
    html! { <div><span></div></span></div> };
}
