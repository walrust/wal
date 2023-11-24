use wal::virtual_dom::{VList, VNode, VText};
use wal_macros::rsx;

include!("../utils/wrap_in_list.rs");

fn main() {
    let rsx = rsx! {
        if false { "hello1" }
        else if let Some(..) = None::<i32> { "hello2" }
        else if 1 == 1 && 1 != 1 { "hello3" }
        else if let Some(val) = Some("hello4") { val }
        else { "hello5" }
    };
    assert_eq!(rsx, wrap_in_list(VNode::Text(VText::new("hello4"))));
}
