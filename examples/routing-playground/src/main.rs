use wal::{
    self,
    component::{behavior::Behavior, callback::Callback, root::RootComponent},
    events::MouseEvent,
    utils::debug,
};
use wal_macros::rsx;
use wal_routing::prelude::RouterBuilder;

struct NavigationComp;
impl RootComponent for NavigationComp {
    type Message = ();

    fn new_root() -> Self {
        NavigationComp
    }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        // idea: <route href="/elo">"Homepage"</route> should be treated as <a href="/elo" data_link>"Homepage"</a>
        rsx! {
            <nav>
                <a href="/" data_link="/">"Homepage"</a><br/>
                <a href="/alt" data_link="/alt">"Alt"</a><br/>
                <a href="/another" data_link="/another">"Another"</a>
            </nav>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool {
        todo!()
    }
}

struct RootComp;
#[allow(clippy::unused_unit)]
impl RootComponent for RootComp {
    type Message = ();
    fn new_root() -> Self {
        RootComp
    }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        let call = Callback::new(|_event: MouseEvent| {
            debug::alert("Homepage");
        });
        rsx! {
            <div id="rootcomp">
                <NavigationComp props={()} />
                <button onclick={call}>"Homepage"</button>
            </div>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}

struct AltComp;
#[allow(clippy::unused_unit)]
impl RootComponent for AltComp {
    type Message = ();
    fn new_root() -> Self {
        AltComp
    }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        let call = Callback::new(|_event: MouseEvent| {
            debug::alert("Alt");
        });
        rsx! {
            <div id="rootcomp">
                <NavigationComp props={()} />
                <button onclick={call}>"Alt"</button>
            </div>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}

struct AnotherComp;
#[allow(clippy::unused_unit)]
impl RootComponent for AnotherComp {
    type Message = ();
    fn new_root() -> Self {
        AnotherComp
    }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        let call = Callback::new(|_event: MouseEvent| {
            debug::alert("Another");
        });
        rsx! {
            <div id="rootcomp">
                <NavigationComp props={()} />
                <button onclick={call}>"Another"</button>
            </div>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool {
        true
    }
}

fn main() {
    RouterBuilder::default()
        .add_page::<RootComp>("/")
        .add_page::<AltComp>("/alt")
        .add_page::<AnotherComp>("/another")
        .build()
        .start();
}
