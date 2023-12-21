use wal_core::virtual_dom::{VNode, VText};
use wal_rsx::rsx;

fn main() {
    let rsx = rsx! {
        if false { "hello1" }
        else if let Some(..) = None::<i32> { "hello2" }
        else if 1 == 1 && 1 != 1 { "hello3" }
        else if let Some(val) = Some("hello4") { val }
        else { "hello5" }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello4")));
}
