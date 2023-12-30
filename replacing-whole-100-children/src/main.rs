use root_component::RootComponent;
use wal_core::router::builder::RouterBuilder;

mod nested_component;
mod root_component;

fn main() {
    RouterBuilder::default()
        .add_page::<RootComponent>("/")
        .build()
        .start();
}
