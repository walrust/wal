use std::{rc::Rc, cell::RefCell};

use gloo::{utils::document, timers::callback::Interval};
use wal_vdom::virtual_dom::{VNode, VElement, VText, mount};
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};

fn log<T>(text: &T)
where
    T: serde::ser::Serialize + ?Sized,
{
    web_sys::console::log_1(&serde_wasm_bindgen::to_value(text).unwrap());
}

fn log_node(node: &Node) {
    web_sys::console::log_1(&JsValue::from(node));
}

// TODO: Finish the goddamn VDOM: https://www.youtube.com/watch?v=85gJMUEcnkc
// to watch: https://www.youtube.com/watch?v=l2Tu0NqH0qU

fn create_elem(count: i32) -> VNode {
    let velement = VElement {
        tag_name: "div".into(),
        attr: [
            ("id".to_string(), "app".to_string()),
            ("dataCount".to_string(), count.to_string()),
        ]
        .into(),
        children: vec![
            VNode::Text {
                virt: VText::new(count.to_string()),
                concrete: None,
            },
            VNode::Element {
                virt: VElement {
                    tag_name: "img".to_string(),
                    attr: [(
                        "src".to_string(),
                        "https://media.giphy.com/media/xUPGGL6TieAUk10oNO/giphy.gif".to_string(),
                    )]
                    .into(),
                    children: vec![],
                },
                concrete: None,
            },
        ],
    };
    log(&velement);

    VNode::Element { virt: velement, concrete: None }
}

#[wasm_bindgen(start)]
fn start() {
    web_sys::console::log_1(&"WALRUST TIME".into());

    let mut count = 0;
    let root = document().get_element_by_id("app").unwrap();
    let mut curr = create_elem(count);
    curr.patch(None, &root);
    let mut curr = Some(curr);
    web_sys::console::log_1(&format!("{:#?}", curr).into());
    let int = Interval::new(1000, move || {
        count += 1;
        let mut now = create_elem(count);
        now.patch(curr.take(), &root);
        curr = Some(now);
    });
    int.forget();
}
