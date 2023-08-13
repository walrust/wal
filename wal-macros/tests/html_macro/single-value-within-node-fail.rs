fn main() {
    html! { <div>{ "valid value" "invalid value" }</div> };
    let reference = "test";
    html! { <div> reference </div> };
    html! { <div>()</div> };
    html! { <div>5</div> };
    html! { <div>"test"</div> };
    html! { <div>{ () }</div> };
    html! { <div>{ invalid_reference }</div> };
}
