use wal_core::{
    component::{behavior::Behavior, Component},
    virtual_dom::VNode,
};

use wal_css::css::Css;
use wal_css::css_stylesheet;
use wal_rsx::rsx;

thread_local! {
    static CSS: Css = css_stylesheet!("../../styles/click_father.css");
}
use crate::pages::click_immediate_reload::click_immediate_reload_child::{
    ChildImmediateReloadComponent, ChildImmediateReloadProperties,
};

pub(crate) enum FatherImmediateReloadMessages {
    FirstChildClicked,
    SecondChildClicked,
    FirstChildChangeName(String),
    SecondChildChangeName(String),
}

#[derive(Hash)]
pub(crate) struct FatherImmediateReloadProperties;

pub(crate) struct FatherImmediateReloadComponent {
    first_child_count: i32,
    first_child_name: String,
    second_child_count: i32,
    second_child_name: String,
}

impl Component for FatherImmediateReloadComponent {
    type Message = FatherImmediateReloadMessages;
    type Properties = FatherImmediateReloadProperties;

    fn new(_props: Self::Properties) -> Self {
        Self {
            first_child_count: 0,
            first_child_name: "first".to_string(),
            second_child_count: 0,
            second_child_name: "second".to_string(),
        }
    }

    fn view(&self, behavior: &mut impl Behavior<Self>) -> VNode {
        let first_child_click_callback =
            behavior.create_callback(|()| FatherImmediateReloadMessages::FirstChildClicked);
        let second_child_click_callback =
            behavior.create_callback(|()| FatherImmediateReloadMessages::SecondChildClicked);
        let first_child_change_name_callback =
            behavior.create_callback(FatherImmediateReloadMessages::FirstChildChangeName);
        let second_child_change_name_callback =
            behavior.create_callback(FatherImmediateReloadMessages::SecondChildChangeName);

        CSS.with(|css| {
            rsx! {
                <div class={&css["container"]}>
                    <div class={&css["raport-container"]}>
                        { format!("My children got clicked {} times", self.first_child_count + self.second_child_count) }
                    </div>
                    <div class={&css["raport-container"]}>
                        { format!("{} child got clicked {} times", self.first_child_name, self.first_child_count) }
                    </div>
                    <div class={&css["raport-container"]}>
                        { format!("{} child got clicked {} times", self.second_child_name, self.second_child_count) }
                    </div>
                    <ChildImmediateReloadComponent
                    props = ChildImmediateReloadProperties {
                        click: first_child_click_callback,
                        on_change_name: first_child_change_name_callback,
                        name: self.first_child_name.clone()
                    }/>
                    <ChildImmediateReloadComponent
                    props = ChildImmediateReloadProperties {
                        click: second_child_click_callback,
                        on_change_name: second_child_change_name_callback,
                        name: self.second_child_name.clone()
                    }/>
                </div>
            }
        })
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            FatherImmediateReloadMessages::FirstChildClicked => {
                self.first_child_count += 1;
                true
            }
            FatherImmediateReloadMessages::SecondChildClicked => {
                self.second_child_count += 1;
                true
            }
            FatherImmediateReloadMessages::FirstChildChangeName(name) => {
                self.first_child_name = name;
                true
            }
            FatherImmediateReloadMessages::SecondChildChangeName(name) => {
                self.second_child_name = name;
                true
            }
        }
    }
}

impl Default for FatherImmediateReloadComponent {
    fn default() -> Self {
        Self::new(FatherImmediateReloadProperties)
    }
}
