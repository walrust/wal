use wal::component::callback::Callback;
use wal_macros::rsx;

fn main() {
    rsx! { <div onclick></div> };
    rsx! { <div onclick=></div> };
    rsx! { <div onclick {Callback::new(|_event: wal::events::MouseEvent)| {}}></div> };
    rsx! {
        <div
            onclick={Callback::new(|_event: wal::events::MouseEvent| {})}
            onclick={Callback::new(|_event: wal::events::MouseEvent| {})}>
        </div>
    };
    rsx! { <div onclick= ondblclick={Callback::new(|_event: wal::events::MouseEvent| {})}></div> };
    rsx! { <div onclick={Callback::new(|_event: wal::events::Event| {})}></div> };
    rsx! { <div onclick={Callback::new(|_event: wal::events::DragEvent| {})}></div> };
    rsx! { <div onclick="value"></div> };
    rsx! { <div onclick={}></div> };
    rsx! { <div onclick={"value"}></div> };
}
