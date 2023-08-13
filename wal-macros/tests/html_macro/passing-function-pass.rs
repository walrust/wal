fn main() {
    let node = || html! { "Hello world!" };
    html! { node() };
    html! { <div>{ node() }</div> };
    let node = || html! {<div>{ "Hello world!" }</div>};
    html! { node() };
    html! { <div>{ node() }</div> };
}
