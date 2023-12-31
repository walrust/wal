use web_sys::Node;

use super::{VComponent, VElement, VList, VText};

/// VNode is enum representing node in virtual DOM tree.
/// Provides a wrapper over different types of nodes along with concise and convinient API for VDOM manipulation.
#[derive(PartialEq, Debug)]
pub enum VNode {
    /// Represents [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element) in DOM and contains [VElement],
    Element(VElement),
    /// Represents [Text](https://developer.mozilla.org/en-US/docs/Web/API/Text) in DOM and contains [VText],
    Text(VText),
    /// Represents a series of adjacent [virtual nodes](VNode) located at the same depth, contains [VList],
    List(VList),
    /// Represents user-defined custom component, contains [VComponent].
    Component(VComponent),
}

impl VNode {
    pub(crate) fn patch(&mut self, last: Option<VNode>, ancestor: &Node) {
        match self {
            VNode::Element(velement) => velement.patch(last, ancestor),
            VNode::Text(vtext) => vtext.patch(last, ancestor),
            VNode::Component(vcomp) => vcomp.patch(last, ancestor),
            VNode::List(vlist) => vlist.patch(last, ancestor),
        };
    }

    pub(crate) fn erase(&self) {
        match self {
            VNode::Element(v) => v.erase(),
            VNode::Text(v) => v.erase(),
            VNode::List(v) => v.erase(),
            VNode::Component(v) => v.erase(),
        }
    }

    pub(crate) fn set_depth(&mut self, depth: u32) {
        match self {
            VNode::Component(vcomp) => vcomp.set_depth(depth),
            VNode::List(vlist) => vlist.set_depth(depth),
            VNode::Element(velem) => velem.set_depth(depth),
            VNode::Text(_) => {}
        }
    }
}

impl From<VElement> for VNode {
    fn from(velement: VElement) -> Self {
        Self::Element(velement)
    }
}

impl From<VComponent> for VNode {
    fn from(vcomp: VComponent) -> Self {
        Self::Component(vcomp)
    }
}

impl From<VText> for VNode {
    fn from(vtext: VText) -> Self {
        Self::Text(vtext)
    }
}

impl From<VList> for VNode {
    fn from(vlist: VList) -> Self {
        Self::List(vlist)
    }
}

impl<T: ToString> From<T> for VNode {
    fn from(t: T) -> Self {
        Self::Text(VText::new(t))
    }
}

impl<T: Into<VNode>> FromIterator<T> for VNode {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::List(VList::new(iter.into_iter().map(Into::into).collect(), None))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        component::{behavior::Behavior, Component},
        virtual_dom::{VComponent, VElement, VList, VText},
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::VNode;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn from_to_string() {
        let target = String::from("tmp");
        assert_eq!(
            VNode::Text(VText {
                text: "tmp".to_string(),
                dom: None
            }),
            target.into()
        );
    }

    #[wasm_bindgen_test]
    fn from_vec_string() {
        let target = vec![String::from("tmp")];
        assert_eq!(
            VNode::List(VList::new(vec![VText::new("tmp").into()], None)),
            VNode::from_iter(target)
        );
    }

    #[wasm_bindgen_test]
    fn from_vec_elements() {
        let target = vec![VElement::new(
            "div".to_string(),
            [].into(),
            [].into(),
            None,
            [].into(),
        )];
        assert_eq!(
            VNode::List(VList::new(
                vec![VNode::Element(VElement::new(
                    "div".to_string(),
                    [].into(),
                    [].into(),
                    None,
                    [].into()
                ))],
                None
            )),
            VNode::from_iter(target)
        );
    }

    #[wasm_bindgen_test]
    fn from_vec_lists() {
        let target = vec![VList::new(vec![], None)];
        assert_eq!(
            VNode::List(VList::new(
                vec![VNode::List(VList::new(vec![], None,))],
                None
            )),
            VNode::from_iter(target)
        );
    }

    struct Comp;
    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn new(_props: Self::Properties) -> Self {
            Comp
        }
        fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
            VText::new("Test").into()
        }
        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    #[wasm_bindgen_test]
    fn from_vec_comp() {
        let target = vec![VComponent::new::<Comp>((), None)];
        assert_eq!(
            VNode::List(VList::new(
                vec![VNode::Component(VComponent::new::<Comp>((), None))],
                None
            )),
            VNode::from_iter(target)
        );
    }
}
