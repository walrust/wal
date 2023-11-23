use std::{any::Any, cell::RefCell, collections::BinaryHeap, rc::Weak};

use wasm_bindgen_futures::spawn_local;

use crate::utils::{any_utils::raw_memory_compare, debug};

use super::node::AnyComponentNode;

#[derive(Debug)]
enum SchedulerMessage {
    Update(UpdateMessage),
    Rerender(RerenderMessage),
}

impl SchedulerMessage {
    fn handle(self) {
        match self {
            SchedulerMessage::Update(msg) => msg.handle(),
            SchedulerMessage::Rerender(msg) => msg.handle(),
        }
    }
}

#[derive(Debug)]
struct UpdateMessage {
    message: Box<dyn Any>,
    any_component_node: Weak<RefCell<AnyComponentNode>>,
}

impl UpdateMessage {
    fn handle(self) {
        if let Some(any_component_node) = self.any_component_node.upgrade() {
            let to_rerender = any_component_node.borrow_mut().update(self.message);
            if to_rerender {
                Scheduler::add_rerender_message(
                    self.any_component_node,
                    any_component_node.borrow().depth,
                );
            }
        } else {
            debug::log("Weak reference to AnyComponentNode is not attached to AnyComponentNode");
        }
    }
}

#[derive(Debug)]
struct RerenderMessage {
    any_component_node: Weak<RefCell<AnyComponentNode>>,
    depth: u32,
}

impl RerenderMessage {
    fn handle(self) {
        if let Some(any_component_node) = self.any_component_node.upgrade() {
            any_component_node.borrow_mut().view_and_patch();
        } else {
            debug::log("Weak reference to AnyComponentNode is not attached to AnyComponentNode");
        }
    }
}

impl PartialEq for SchedulerMessage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Update(s_msg), Self::Update(o_msg)) => {
                Weak::ptr_eq(&s_msg.any_component_node, &o_msg.any_component_node)
                    && raw_memory_compare(&s_msg.message, &o_msg.message)
            }
            (Self::Rerender(s_msg), Self::Rerender(o_msg)) => {
                Weak::ptr_eq(&s_msg.any_component_node, &o_msg.any_component_node)
                    && s_msg.depth == o_msg.depth
            }
            _ => false,
        }
    }
}

impl Eq for SchedulerMessage {}

impl PartialOrd for SchedulerMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SchedulerMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Update(_), Self::Rerender(_)) => std::cmp::Ordering::Greater,
            (Self::Rerender(_), Self::Update(_)) => std::cmp::Ordering::Less,
            (Self::Update(_), Self::Update(_)) => std::cmp::Ordering::Equal,
            (Self::Rerender(s_msg), Self::Rerender(o_msg)) => {
                s_msg.depth.cmp(&o_msg.depth).reverse()
            }
        }
    }
}

thread_local! {
    pub static SCHEDULER_INSTANCE: RefCell<Scheduler> = RefCell::new(Scheduler::new());
}

pub struct Scheduler {
    messages: BinaryHeap<SchedulerMessage>,
    is_handle_messages_scheduled: bool,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            messages: BinaryHeap::new(),
            is_handle_messages_scheduled: false,
        }
    }

    fn schedule_handle_messages(&mut self) {
        if !self.is_handle_messages_scheduled {
            self.is_handle_messages_scheduled = true;
            spawn_local(async {
                Scheduler::handle_messages();
            });
        }
    }

    fn handle_messages() {
        let scheduler_messages: Vec<SchedulerMessage> = SCHEDULER_INSTANCE.with(|scheduler| {
            let mut scheduler = scheduler.borrow_mut();
            let messages = scheduler.messages.drain().collect();
            scheduler.is_handle_messages_scheduled = false;
            messages
        });

        for scheduler_message in scheduler_messages {
            scheduler_message.handle();
        }
    }

    pub fn add_update_message(
        message: Box<dyn Any>,
        any_component_node: Weak<RefCell<AnyComponentNode>>,
    ) {
        let message = SchedulerMessage::Update(UpdateMessage {
            message,
            any_component_node,
        });
        Self::add_message(message);
    }

    pub fn add_rerender_message(any_component_node: Weak<RefCell<AnyComponentNode>>, depth: u32) {
        let message = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node,
            depth,
        });
        Self::add_message(message);
    }

    fn add_message(message: SchedulerMessage) {
        SCHEDULER_INSTANCE.with(|scheduler| {
            let mut scheduler = scheduler.borrow_mut();
            scheduler.messages.push(message);
            scheduler.schedule_handle_messages();
        });
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::component::Component;

    use super::*;
    use crate::virtual_dom::*;
    use std::rc::Rc;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    struct TestComponent;
    impl Component for TestComponent {
        type Message = i8;
        type Properties = ();

        fn new(_props: Self::Properties) -> Self {
            TestComponent
        }

        fn view(&self, _behavior: &mut impl crate::component::behavior::Behavior<Self>) -> VNode {
            VNode::List(VList::new_empty(None))
        }

        fn update(&mut self, _message: Self::Message) -> bool {
            unimplemented!();
        }
    }

    struct TestComponent2;
    impl Component for TestComponent2 {
        type Message = i8;
        type Properties = ();

        fn new(_props: Self::Properties) -> Self {
            TestComponent2
        }

        fn view(&self, _behavior: &mut impl crate::component::behavior::Behavior<Self>) -> VNode {
            VNode::List(VList::new_empty(None))
        }

        fn update(&mut self, _message: Self::Message) -> bool {
            unimplemented!();
        }
    }

    struct UpdateReturnsTrueComponent;

    impl Component for UpdateReturnsTrueComponent {
        type Message = ();
        type Properties = ();

        fn new(_props: Self::Properties) -> Self {
            UpdateReturnsTrueComponent
        }

        fn view(&self, _behavior: &mut impl crate::component::behavior::Behavior<Self>) -> VNode {
            VNode::List(VList::new_empty(None))
        }

        fn update(&mut self, _message: Self::Message) -> bool {
            true
        }
    }

    struct UpdateReturnsFalseComponent;

    impl Component for UpdateReturnsFalseComponent {
        type Message = ();
        type Properties = ();

        fn new(_props: Self::Properties) -> Self {
            UpdateReturnsFalseComponent
        }

        fn view(&self, _behavior: &mut impl crate::component::behavior::Behavior<Self>) -> VNode {
            VNode::List(VList::new_empty(None))
        }

        fn update(&mut self, _message: Self::Message) -> bool {
            false
        }
    }

    fn get_body() -> web_sys::Node {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .into()
    }

    fn clear_scheduler() {
        SCHEDULER_INSTANCE.with(|scheduler| {
            let mut scheduler = scheduler.borrow_mut();
            scheduler.messages.clear();
            scheduler.is_handle_messages_scheduled = false;
        });
    }

    fn create_any_component_node<T: Component + 'static>(
        props: T::Properties,
    ) -> Rc<RefCell<AnyComponentNode>> {
        let ancestor = get_body();
        let component = T::new(props);
        AnyComponentNode::new_root(component, ancestor)
    }

    fn create_update_message<T: Component>(
        message: T::Message,
        any_component_node: &Rc<RefCell<AnyComponentNode>>,
    ) -> SchedulerMessage {
        let weak_component_node = Rc::downgrade(any_component_node);
        SchedulerMessage::Update(UpdateMessage {
            message: Box::new(message),
            any_component_node: weak_component_node,
        })
    }

    fn create_rerender_message(
        any_component_node: &Rc<RefCell<AnyComponentNode>>,
        depth: u32,
    ) -> SchedulerMessage {
        let weak_component_node = Rc::downgrade(any_component_node);
        SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node,
            depth,
        })
    }

    // Tests for message eq

    #[wasm_bindgen_test]
    fn update_messages_from_the_same_component_and_the_same_message_should_be_equal() {
        // Arrange
        let component_node = create_any_component_node::<TestComponent>(());
        let message = 0;
        let update_message1 = create_update_message::<TestComponent>(message, &component_node);
        let update_message2 = create_update_message::<TestComponent>(message, &component_node);

        // Act & Assert
        assert_eq!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn update_messages_from_the_same_component_but_with_different_message_should_not_be_equal() {
        // Arrange
        let component_node = create_any_component_node::<TestComponent>(());
        let update_message1 = create_update_message::<TestComponent>(0, &component_node);
        let update_message2 = create_update_message::<TestComponent>(1, &component_node);

        // Act & Assert
        assert_ne!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn update_messages_from_different_component_type_should_not_be_equal() {
        // Arrange
        let message = 0;

        let component_node1 = create_any_component_node::<TestComponent>(());
        let update_message1 = create_update_message::<TestComponent>(message, &component_node1);

        let component_node2 = create_any_component_node::<TestComponent2>(());
        let update_message2 = create_update_message::<TestComponent2>(message, &component_node2);

        // Act & Assert
        assert_ne!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn update_messages_from_the_same_component_type_but_different_instance_and_the_same_message_should_not_be_equal(
    ) {
        // Arrange
        let message = 0;

        let component_node1 = create_any_component_node::<TestComponent>(());
        let update_message1 = create_update_message::<TestComponent>(message, &component_node1);

        let component_node2 = create_any_component_node::<TestComponent>(());
        let update_message2 = create_update_message::<TestComponent>(message, &component_node2);

        // Act & Assert
        assert_ne!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_the_same_component_and_the_same_depth_should_be_equal() {
        // Arrange
        let depth = 0;

        let component_node = create_any_component_node::<TestComponent>(());
        let rerender_message1 = create_rerender_message(&component_node, depth);
        let rerender_message2 = create_rerender_message(&component_node, depth);

        // Act & Assert
        assert_eq!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_the_same_component_with_different_depth_should_not_be_equal() {
        // Arrange
        let component_node = create_any_component_node::<TestComponent>(());
        let rerender_message1 = create_rerender_message(&component_node, 0);
        let rerender_message2 = create_rerender_message(&component_node, 1);

        // Act & Assert
        assert_ne!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_the_same_component_type_but_different_instance_should_not_be_equal() {
        // Arrange
        let depth = 0;

        let component_node1 = create_any_component_node::<TestComponent>(());
        let rerender_message1 = create_rerender_message(&component_node1, depth);

        let component_node2 = create_any_component_node::<TestComponent>(());
        let rerender_message2 = create_rerender_message(&component_node2, depth);

        // Act & Assert
        assert_ne!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_different_components_should_not_be_equal() {
        // Arrange
        let depth = 0;

        let component_node1 = create_any_component_node::<TestComponent>(());
        let rerender_message1 = create_rerender_message(&component_node1, depth);

        let component_node2 = create_any_component_node::<TestComponent2>(());
        let rerender_message2 = create_rerender_message(&component_node2, depth);

        // Act & Assert
        assert_ne!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_and_update_message_should_not_be_equal() {
        // Arrange
        let component_node = create_any_component_node::<TestComponent>(());
        let update_message = create_update_message::<TestComponent>(0, &component_node);
        let rerender_message = create_rerender_message(&component_node, 0);

        // Act & Assert
        assert_ne!(rerender_message, update_message);
    }

    // Tests for message cmp

    #[wasm_bindgen_test]
    fn update_message_should_be_greater_than_rerender_message() {
        // Arrange
        let component_node = create_any_component_node::<TestComponent>(());
        let update_message = create_update_message::<TestComponent>(0, &component_node);
        let rerender_message = create_rerender_message(&component_node, 0);

        // Act & Assert
        assert_eq!(
            update_message.cmp(&rerender_message),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            rerender_message.cmp(&update_message),
            std::cmp::Ordering::Less
        );
    }

    #[wasm_bindgen_test]
    fn update_message_should_be_equal_update_message() {
        // Arrange
        let component_node = create_any_component_node::<TestComponent>(());
        let update_message1 = create_update_message::<TestComponent>(0, &component_node);
        let update_message2 = create_update_message::<TestComponent>(1, &component_node);

        // Act & Assert
        assert_eq!(
            update_message1.cmp(&update_message2),
            std::cmp::Ordering::Equal
        );

        assert_eq!(
            update_message2.cmp(&update_message1),
            std::cmp::Ordering::Equal
        );
    }

    #[wasm_bindgen_test]
    fn rerender_message_with_smaller_depth_should_be_greater_than_rerender_message_with_bigger_depth(
    ) {
        // Arrange
        let component_node = create_any_component_node::<TestComponent>(());
        let rerender_message1 = create_rerender_message(&component_node, 0);
        let rerender_message2 = create_rerender_message(&component_node, 1);

        // Act & Assert
        assert_eq!(
            rerender_message1.cmp(&rerender_message2),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            rerender_message2.cmp(&rerender_message1),
            std::cmp::Ordering::Less
        );
    }

    #[wasm_bindgen_test]
    fn rerender_messages_with_the_same_depth_should_be_equal() {
        // Arrange
        let depth = 0;
        let component_node = create_any_component_node::<TestComponent>(());
        let rerender_message1 = create_rerender_message(&component_node, depth);
        let rerender_message2 = create_rerender_message(&component_node, depth);

        // Act & Assert
        assert_eq!(
            rerender_message1.cmp(&rerender_message2),
            std::cmp::Ordering::Equal
        );

        assert_eq!(
            rerender_message2.cmp(&rerender_message1),
            std::cmp::Ordering::Equal
        );
    }

    // Tests update message handle

    #[wasm_bindgen_test]
    fn handle_update_message_when_update_returns_true_should_add_rerender_message_to_schedulers_queue(
    ) {
        // Arrange
        clear_scheduler();
        let component_node = create_any_component_node::<UpdateReturnsTrueComponent>(());
        let update_message =
            create_update_message::<UpdateReturnsTrueComponent>((), &component_node);
        let expected_rerender_message = create_rerender_message(&component_node, 0);

        // Act
        update_message.handle();

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 1);
            assert!(scheduler.is_handle_messages_scheduled);
            let rerender_message = scheduler.messages.iter().next().unwrap();
            assert_eq!(rerender_message, &expected_rerender_message);
        });
    }

    #[wasm_bindgen_test]
    fn handle_update_message_when_update_returns_false_should_not_add_rerender_message_to_schedulers_queue(
    ) {
        // Arrange
        clear_scheduler();
        let component_node = create_any_component_node::<UpdateReturnsFalseComponent>(());
        let update_message =
            create_update_message::<UpdateReturnsFalseComponent>((), &component_node);

        // Act
        update_message.handle();

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 0);
            assert!(!scheduler.is_handle_messages_scheduled);
        });
    }

    #[wasm_bindgen_test]
    fn handle_update_message_with_outdated_weak_reference_should_not_add_rerender_message_to_schedulers_queue(
    ) {
        // Arrange
        clear_scheduler();
        let component_node = create_any_component_node::<UpdateReturnsTrueComponent>(());
        let update_message =
            create_update_message::<UpdateReturnsTrueComponent>((), &component_node);

        // Act
        drop(component_node);
        update_message.handle();

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 0);
            assert!(!scheduler.is_handle_messages_scheduled);
        });
    }

    // Tests schedule_handle_messages

    #[wasm_bindgen_test]
    fn schedule_handle_messages_should_schedule_handle_messages() {
        // Act
        clear_scheduler();
        SCHEDULER_INSTANCE.with(|scheduler| {
            scheduler.borrow_mut().schedule_handle_messages();
        });

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            assert!(scheduler.borrow().is_handle_messages_scheduled);
        });
    }

    // Tests add messages

    #[wasm_bindgen_test]
    fn add_update_message_should_add_update_message_to_schedulers_queue() {
        // Arrange
        clear_scheduler();
        let component_node = create_any_component_node::<TestComponent>(());
        let message = 0;
        let expected_update_message =
            create_update_message::<TestComponent>(message, &component_node);
        let message_boxed = Box::new(message);
        let weak_component_node = Rc::downgrade(&component_node);

        // Act
        Scheduler::add_update_message(message_boxed, weak_component_node);

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 1);
            assert!(scheduler.is_handle_messages_scheduled);
            let update_message = scheduler.messages.iter().next().unwrap();
            assert_eq!(update_message, &expected_update_message);
        });
    }

    #[wasm_bindgen_test]
    fn add_rerender_message_should_add_rerender_message_to_schedulers_queue() {
        // Arrange
        clear_scheduler();
        let depth = 0;
        let component_node = create_any_component_node::<TestComponent>(());
        let expected_rerender_message = create_rerender_message(&component_node, depth);
        let weak_component_node = Rc::downgrade(&component_node);

        // Act
        Scheduler::add_rerender_message(weak_component_node, depth);

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 1);
            assert!(scheduler.is_handle_messages_scheduled);
            let rerender_message = scheduler.messages.iter().next().unwrap();
            assert_eq!(rerender_message, &expected_rerender_message);
        });
    }
}
