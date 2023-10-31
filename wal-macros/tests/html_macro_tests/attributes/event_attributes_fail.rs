use wal::component::callback::Callback;
use wal_macros::html;

fn main() {
    html! { <div onclick></div> };
    html! { <div onclick=></div> };
    html! { <div onclick {Callback::new(|_event: web_sys::MouseEvent)| {}}></div> };
    html! {
        <div
            onclick={Callback::new(|_event: web_sys::MouseEvent| {})}
            onclick={Callback::new(|_event: web_sys::MouseEvent| {})}>
        </div>
    };
    html! { <div onclick= ondblclick={Callback::new(|_event: web_sys::MouseEvent| {})}></div> };
    html! { <div onclick={Callback::new(|_event: web_sys::Event| {})}></div> };
    html! { <div onclick={Callback::new(|_event: web_sys::DragEvent| {})}></div> };
    html! { <div onclick="value"></div> };
    html! { <div onclick={}></div> };
    html! { <div onclick={"value"}></div> };
}
