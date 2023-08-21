use wal_vdom::virtual_dom::VNode;

pub trait Component {
    fn create() -> Self;

    // NOTE: default implementation just for testing - change later
    fn view(&mut self) -> VNode {
        VNode::Text {
            vtext: wal_vdom::virtual_dom::VText {
                text: String::from("empty component"),
            },
        }
    }

    // NOTE: maybe resign form default implementation alltogether
    fn update(&mut self) -> bool {
        true
    }
}
