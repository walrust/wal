use gloo::{utils::document, timers::callback::Interval};
use wal::virtual_dom::{VNode, VElement, VText};
use wasm_bindgen::prelude::*;
use web_sys::Node;

fn _log<T>(text: &T)
where
    T: serde::ser::Serialize + ?Sized,
{
    web_sys::console::log_1(&serde_wasm_bindgen::to_value(text).unwrap());
}

fn _log_node(node: &Node) {
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
                VText::new(count.to_string()).into(),
                VElement {                    
                    tag_name: "img".to_string(),
                    attr: [(
                        "src".to_string(),
                        "https://media.giphy.com/media/xUPGGL6TieAUk10oNO/giphy.gif".to_string(),
                    )]
                    .into(),
                    children: vec![],
                }.into(),
        ],
    };
    //log_node(&velement);

    VNode::Element { velement, concrete: None }
}

#[wasm_bindgen(start)]
fn start() {
    web_sys::console::log_1(&"WALRUST TIME".into());

    let mut count = 0;
    let root = document().get_element_by_id("app").unwrap();
    let mut last = create_elem(count);
    last.patch(None, &root);
    let mut last = Some(last);
    web_sys::console::log_1(&format!("{:#?}", last).into());

    let int = Interval::new(1000, move || {
        count += 1;
        let mut curr = create_elem(count);
        curr.patch(last.take(), &root);
        last = Some(curr);
    });
    int.forget();
}
