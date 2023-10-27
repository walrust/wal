use gloo::console::log;
use std::{cell::RefCell, collections::HashSet};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};

use super::Component;

#[wasm_bindgen]
pub async fn fetch_file(path: &str) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(path, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let file_str = JsFuture::from(resp.text()?).await?.as_string().unwrap();
    Ok(file_str)
}

pub struct CssBinder {
    style_string: String,
    bound_components: HashSet<String>,
}

thread_local! {
    pub static BINDER_INSTANCE: RefCell<CssBinder> = RefCell::new(CssBinder::new());
}

impl CssBinder {
    pub fn new() -> CssBinder {
        let mut binder = CssBinder {
            style_string: String::new(),
            bound_components: HashSet::new(),
        };
        binder.reset();
        binder
    }

    pub fn get_style_str(&self) -> &str {
        &self.style_string
    }

    pub fn reset(&mut self) {
        self.style_string.clear();
        self.bound_components.clear();
    }

    pub fn bind_stylesheet<C: Component + 'static>(&mut self) {
        let c_name = C::get_type_name();
        let stylesheet_path = C::stylesheet_path();
        if self.bound_components.contains(&c_name) || stylesheet_path.is_none() {
            return;
        }
        if !self.bound_components.insert(c_name.clone()) {
            log!("error inserting component name: ", &c_name);
            panic!("error inserting component name: {}", &c_name);
        }
        let stylesheet_path = stylesheet_path.unwrap();

        // async part with fetch call
        // async {
        //     let new_css = fetch_file(&stylesheet_path).await.unwrap_or_else(|_|  {
        //         log!("error fetching file: ", &stylesheet_path);
        //         panic!("error opening file: {}", &stylesheet_path);
        //     });

        //     self.style_string.push_str(&Self::apply_binding(new_css, &c_name));
        // };

        let _handle = spawn_local(async {
            let new_css = fetch_file(&stylesheet_path).await.unwrap_or_else(|_| {
                log!("error fetching file: ", stylesheet_path);
                panic!("error opening file: {}", stylesheet_path);
            });

            self.style_string
                .push_str(&Self::apply_binding(new_css, &c_name));
        });
    }

    fn apply_binding(file_str: String, component_id: &str) -> String {
        let mut bound_css = String::new();
        bound_css.push('.');
        bound_css.push_str(component_id);
        bound_css.push_str(" {\n");
        bound_css.push_str(&file_str);
        bound_css.push('}');
        bound_css
    }

    pub fn bind_stylesheet_using_static_instance<C: Component + 'static>() {
        BINDER_INSTANCE.with(|binder| {
            let mut binder = binder.borrow_mut();
            binder.bind_stylesheet::<C>();
        });
    }

    pub fn reset_static_instance() {
        BINDER_INSTANCE.with(|binder| {
            let mut binder = binder.borrow_mut();
            binder.reset();
        });
    }

    pub fn get_style_str_from_static_instance() -> String {
        return BINDER_INSTANCE.with(|binder| {
            let binder = binder.borrow_mut();
            String::from(binder.get_style_str())
        });
    }
}
