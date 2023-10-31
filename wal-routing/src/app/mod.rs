pub mod builder;

use std::collections::HashMap;

use wal::component::node::AnyComponentNode;

pub struct App {
    pub pages: HashMap<&'static str, AnyComponentNode>,
}
