use wal_macros::html;
use wal_vdom::virtual_dom::{VNode, VText};

fn main() {
    let html = html! {
        if false { "hello1" }
        else if let Some(..) = None::<i32> { "hello2" }
        else if 1 == 1 && 1 != 1 { "hello3" }
        else if let Some(val) = Some("hello4") { val }
        else { "hello5" }
    };
    assert_eq!(html, VNode::Text(VText::new("hello4")));
}
