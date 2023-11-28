use pages::{
    click_immediate_reload::click_immediate_reload_father::FatherImmediateReloadComponent,
    for_example::father_for::FatherForComponent,
    if_example::conditional_component::ConditionComponent, menu::MenuComponent, click_not_immediate_reload::click_not_immediate_reload_father::NotImmediateReloadFatherComponent,
};
use wal_routing::router::builder::RouterBuilder;

mod pages;

fn main() {
    RouterBuilder::default()
        .add_page::<MenuComponent>("/")
        .add_page::<FatherImmediateReloadComponent>("/immediate_reload")
        .add_page::<FatherForComponent>("/for_example")
        .add_page::<ConditionComponent>("/if_example")
        .add_page::<NotImmediateReloadFatherComponent>("/not_immediate_reload")
        .build()
        .start();
}
