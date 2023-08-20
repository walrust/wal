pub extern crate wal;

use gloo::utils::document;
use wal::virtual_dom::*;
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
                vtext: VText::new(count.to_string()),
            },
            VNode::Element {
                velement: VElement {
                    tag_name: "img".to_string(),
                    attr: [(
                        "src".to_string(),
                        "https://media.giphy.com/media/xUPGGL6TieAUk10oNO/giphy.gif".to_string(),
                    )]
                    .into(),
                    children: vec![],
                },
            },
        ],
    };
    log(&velement);

    VNode::Element { velement }
}

#[wasm_bindgen(start)]
fn start() {
    web_sys::console::log_1(&"WALRUST TIME".into());

    let mut count = 0;
    let mut current = document().get_element_by_id("app").unwrap();

    let mut el = create_elem(count);
    let mut app = match el.render() {
        Ok(val) => val,
        Err(err) => {
            web_sys::console::log_1(&err);
            return;
        }
    };
    match mount(&app, &current) {
        Ok(_) => (),
        Err(_) => todo!(),
    };
    current = Element::from(JsValue::from(app.clone()));
    let int = gloo::timers::callback::Interval::new(1000, move || {
        count += 1;
        el = create_elem(count);
        app = match el.render() {
            Ok(val) => val,
            Err(err) => {
                web_sys::console::log_1(&err);
                return;
            }
        };
        match mount(&app, &current) {
            Ok(_) => (),
            Err(err) => {
                web_sys::console::log_1(&err);
                return;
            }
        };
        current = Element::from(JsValue::from(app.clone()));
    });
    int.forget();
}
