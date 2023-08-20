use wal_macros::html;

fn main() {
    let node = || 5;
    html! { node() };
    html! { <div> { node() } </div> };

    // it is commented out because html doesnt return anything for now. When it does it can be commented out
    // let node = || html! { "Hello world!" };
    // html! { node() };
    // html! { <div> { node() } </div> };

    // let node = || html! {<div> "Hello world!" </div>};
    // html! { node() };
    // html! { <div> { node() } </div> };
}
