pub extern crate wal;

use gloo::utils::document;
use wal::virtual_dom::*;
use wasm_bindgen::prelude::*;
use web_sys::Node;

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

#[wasm_bindgen(start)]
fn start() {
    web_sys::console::log_1(&"WALRUST TIME".into());

    let el = VNode {
        tag_name: "div",
        attr: [("id", "app"), ("dataCount", "0")].into(),
        children: vec![VNode {
            tag_name: "img",
            attr: [(
                "src",
                "https://media.giphy.com/media/xUPGGL6TieAUk10oNO/giphy.gif",
            )]
            .into(),
            children: vec![],
        }],
    };
    log(&el);

    let app = match render(el) {
        Ok(val) => val,
        Err(err) => {
            web_sys::console::log_1(&err);
            return;
        }
    };
    log_node(&app);

    let current = document().get_element_by_id("app").unwrap();
    match mount(app, current) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
