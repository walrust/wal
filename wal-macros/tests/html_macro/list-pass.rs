fn main() {
    html! { <></> };
    html! {
        <>
            <div></div>
            <div></div>
        </>
    };
    html! { <key="value"></> };
    let children = vec![html! {<div></div>}, html! {<div></div>}];
    html! { <>{ children }</> };
}
