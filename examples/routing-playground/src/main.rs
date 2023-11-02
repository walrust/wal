use wal_routing::router;
use wal::{self, component::{Component, behavior::Behavior, callback::Callback}, utils::debug};
use wal_macros::html;
use web_sys::MouseEvent;

struct NavigationComp;
impl Component for NavigationComp {
    type Message=();
    type Properties=();

    fn new(_props: Self::Properties) -> Self { NavigationComp }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        html! {
            <nav>
                <a href="/" data_link="/">"Homepage"</a>
                <a href="/alt" data_link="/alt">"Alt"</a>
                <a href="/another" data_link="/another">"Another"</a>
            </nav>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool { todo!() }
}

struct RootComp;
#[derive(Hash)]
struct RootProp;
impl Component for RootComp {
    type Message=();
    type Properties=RootProp;
    fn new(_props: Self::Properties) -> Self { RootComp }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        let call = Callback::new(|_event: MouseEvent| {
            debug::alert("Homepage");
        });
        html! {
            <div id="rootcomp">
                <NavigationComp props={()} />
                <button onclick={call}>"Homepage"</button>
            </div>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool { true }
}

struct AltComp;
#[derive(Hash)]
struct AltProp;
impl Component for AltComp {
    type Message=();
    type Properties=AltProp;
    fn new(_props: Self::Properties) -> Self { AltComp }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        let call = Callback::new(|_event: MouseEvent| {
            debug::alert("Alt");
        });
        html! {
            <div id="rootcomp">
                <NavigationComp props={()} />
                <button onclick={call}>"Alt"</button>
            </div>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool { true }
}

struct AnotherComp;
#[derive(Hash)]
struct AnotherProp;
impl Component for AnotherComp {
    type Message=();
    type Properties=AnotherProp;
    fn new(_props: Self::Properties) -> Self { AnotherComp }
    fn view(&self, _behavior: &mut impl Behavior<Self>) -> wal::virtual_dom::VNode {
        let call = Callback::new(|_event: MouseEvent| {
            debug::alert("Another");
        });
        html! {
            <div id="rootcomp">
                <NavigationComp props={()} />
                <button onclick={call}>"Another"</button>
            </div>
        }
    }
    fn update(&mut self, _message: Self::Message) -> bool { true }
}

fn main() {
    // wal::app::start(RootComp);
    router::builder::Builder::new()
        .add_page::<RootComp>("/", RootProp{})
        .add_page::<AltComp>("/alt", AltProp{})
        .add_page::<AnotherComp>("/another", AnotherProp{})
        .build()
        .start();
}
