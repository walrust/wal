use std::collections::HashMap;
use wal_core::virtual_dom::{VElement, VList, VNode, VText};
use wal_rsx::rsx;

fn main() {
    if_true();
    if_false();
    if_true_else_if();
    if_false_else_if_true();
    if_false_else_if_false();
    if_multiple_else_ifs();
    if_true_else();
    if_false_else();
    if_multiple_else_ifs_else();
    if_false_multiple_else_ifs_false_else();
    if_in_element();
    if_with_complex_condition();
}

fn if_true() {
    let rsx = rsx! {
        if true {
            "hello"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello")));
}

fn if_false() {
    let rsx = rsx! {
        if false {
            "hello"
        }
    };
    assert_eq!(rsx, VNode::List(VList::new_empty(None)));
}

fn if_true_else_if() {
    let rsx = rsx! {
        if true {
            "hello"
        } else if true {
            "hello2"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello")));
}

fn if_false_else_if_true() {
    let rsx = rsx! {
        if false {
            "hello"
        } else if true {
            "hello2"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello2")));
}

fn if_false_else_if_false() {
    let rsx = rsx! {
        if false {
            "hello"
        } else if false {
            "hello2"
        }
    };
    assert_eq!(rsx, VNode::List(VList::new_empty(None)));
}

fn if_multiple_else_ifs() {
    let rsx = rsx! {
        if false {
            "hello"
        } else if false {
            "hello2"
        } else if true {
            "hello3"
        } else if false {
            "hello4"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello3")));
}

fn if_true_else() {
    let rsx = rsx! {
        if true {
            "hello"
        } else {
            "hello2"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello")));
}

fn if_false_else() {
    let rsx = rsx! {
        if false {
            "hello"
        } else {
            "hello2"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello2")));
}

fn if_multiple_else_ifs_else() {
    let rsx = rsx! {
        if false {
            "hello"
        } else if false {
            "hello2"
        } else if true {
            "hello3"
        } else {
            "hello4"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello3")));
}

fn if_false_multiple_else_ifs_false_else() {
    let rsx = rsx! {
        if false {
            "hello"
        } else if false {
            "hello2"
        } else if false {
            "hello3"
        } else {
            "hello4"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello4")));
}

fn if_in_element() {
    let rsx = rsx! {
        <div>
            if true {
                "hello"
            }
        </div>
    };
    assert_eq!(
        rsx,
        VNode::Element(VElement::new(
            String::from("div"),
            HashMap::new(),
            Vec::new(),
            None,
            vec![VNode::Text(VText::new("hello"))],
        ))
    );
}

fn if_with_complex_condition() {
    let f = false;
    let rsx = rsx! {
        if 1 == 1 && f {
            "hello"
        } else if false || !f {
            "hello2"
        }
    };
    assert_eq!(rsx, VNode::Text(VText::new("hello2")));
}
