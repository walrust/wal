use wal_core::component::callback::Callback;
use wal_rsx::rsx;

fn main() {
    rsx! { <div onclick></div> };
    rsx! { <div onclick=></div> };
    rsx! { <div onclick {Callback::new(|_event: wal_core::events::MouseEvent)| {}}></div> };
    rsx! {
        <div
            onclick={Callback::new(|_event: wal_core::events::MouseEvent| {})}
            onclick={Callback::new(|_event: wal_core::events::MouseEvent| {})}>
        </div>
    };
    rsx! { <div onclick= ondblclick={Callback::new(|_event: wal_core::events::MouseEvent| {})}></div> };
    rsx! { <div onclick={Callback::new(|_event: wal_core::events::Event| {})}></div> };
    rsx! { <div onclick={Callback::new(|_event: wal_core::events::DragEvent| {})}></div> };
    rsx! { <div onclick="value"></div> };
    rsx! { <div onclick={}></div> };
    rsx! { <div onclick={"value"}></div> };
}
