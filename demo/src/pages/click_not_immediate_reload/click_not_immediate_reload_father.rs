use crate::pages::click_not_immediate_reload::click_not_immediate_reload_child::{
    ClickNotImmediateReloadChild, ClickNotImmediateReloadChildProperties,
};
use wal::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};

use super::click_not_immediate_reload_child::ToBeUpdatedMessage;
use wal_css::css::Css;
use wal_css::css_stylesheet;
use wal_macros::rsx;

thread_local! {
    static CSS: Css = css_stylesheet!("../../styles/click_father.css");
}
pub(crate) struct NotImmediateReloadFatherComponent {
    children: Vec<Children>,
}

struct Children {
    id: i32,
    count: i32,
    name: String,
}

pub(crate) enum MyMessage {
    ChildToBeUpdated(ToBeUpdatedMessage),
}

impl Component for NotImmediateReloadFatherComponent {
    type Message = MyMessage;
    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        Self {
            children: vec![
                Children {
                    id: 0,
                    count: 0,
                    name: "first".to_string(),
                },
                Children {
                    id: 1,
                    count: 0,
                    name: "second".to_string(),
                },
            ],
        }
    }

    fn view(&self, behavior: &mut impl Behavior<Self>) -> VNode {
        let children_to_be_updated_message_callback =
            behavior.create_callback(MyMessage::ChildToBeUpdated);

        CSS.with(|css| {
            rsx! {
                <div  class={&css["container"]}>
                { self.children_raport() }
                for { self.children.iter().map(|child|
                    rsx! { <ClickNotImmediateReloadChild props = {
                        ClickNotImmediateReloadChildProperties {
                            id: child.id,
                            count: child.count,
                            name: child.name.clone(),
                            click: children_to_be_updated_message_callback.clone()
                        }
                    }/>
                })}
                </div>
            }
        })
    }

    fn update(&mut self, message: Self::Message) -> bool {
        let MyMessage::ChildToBeUpdated(message) = message;

        if let Some(child) = self
            .children
            .iter_mut()
            .find(|child| child.id == message.id)
        {
            let to_be_reloaded = child.name != message.name || child.count != message.count;
            child.name = message.name;
            child.count = message.count;
            return to_be_reloaded;
        }

        false
    }
}

impl NotImmediateReloadFatherComponent {
    fn children_raport(&self) -> VNode {
        let children = &self.children;
        rsx! { for children.iter().map(|child| { Self::child_raport(child) })}
    }

    fn child_raport(child: &Children) -> VNode {
        CSS.with(|css| {
            rsx! {
                <div class={&css["raport-container"]}>
                    { format!("Child with id {}, name {}, got clicked {} times", child.id, child.name, child.count) }
                </div>
            }
        })
    }
}

impl Default for NotImmediateReloadFatherComponent {
    fn default() -> Self {
        Self::new(())
    }
}
