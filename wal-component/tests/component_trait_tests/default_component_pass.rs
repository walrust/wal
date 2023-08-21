use wal_component::Component;

struct TestComponent {
    pub state: (),
}

impl Component for TestComponent {
    fn create() -> Self {
        TestComponent { state: () }
    }
}

fn main() {
    let mut c = TestComponent::create();
    // default implementations
    let _vnode = c.view();
    let _rerender = c.update();
}
