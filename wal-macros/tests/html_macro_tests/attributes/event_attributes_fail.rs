use wal::component::callback::Callback;
use wal_macros::html;

fn main() {
    html! { <div onclick></div> };
    html! { <div onclick=></div> };
    html! { <div onclick {Callback::new(|_event: wal::events::MouseEvent)| {}}></div> };
    html! {
        <div
            onclick={Callback::new(|_event: wal::events::MouseEvent| {})}
            onclick={Callback::new(|_event: wal::events::MouseEvent| {})}>
        </div>
    };
    html! { <div onclick= ondblclick={Callback::new(|_event: wal::events::MouseEvent| {})}></div> };
    html! { <div onclick={Callback::new(|_event: wal::events::Event| {})}></div> };
    html! { <div onclick={Callback::new(|_event: wal::events::DragEvent| {})}></div> };
    html! { <div onclick="value"></div> };
    html! { <div onclick={}></div> };
    html! { <div onclick={"value"}></div> };
}
