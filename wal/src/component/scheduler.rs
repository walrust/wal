use std::{any::Any, cell::RefCell, collections::BinaryHeap, rc::Weak};

use wasm_bindgen_futures::spawn_local;

use crate::utils::{any_utils::raw_memory_compare, debug};

use super::component_node::AnyComponentNode;

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

    // Tests for message eq

    #[wasm_bindgen_test]
    fn update_messages_from_the_same_component_and_the_same_message_should_be_equal() {
        // Arrange
        let component = TestComponent;
        let ancestor = get_body();
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let message = 0;
        let update_message1 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(message),
            any_component_node: weak_component_node.clone(),
        });
        let update_message2 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(message),
            any_component_node: weak_component_node,
        });

        // Act & Assert
        assert_eq!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn update_messages_from_the_same_component_but_with_different_message_should_not_be_equal() {
        // Arrange
        let component = TestComponent;
        let ancestor = get_body();
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let update_message1 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(0),
            any_component_node: weak_component_node.clone(),
        });
        let update_message2 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(1),
            any_component_node: weak_component_node,
        });

        // Act & Assert
        assert_ne!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn update_messages_from_different_component_type_should_not_be_equal() {
        // Arrange
        let ancestor = get_body();
        let message = 0;

        let component1 = TestComponent;
        let component_node1 = AnyComponentNode::new_root(component1, ancestor.clone());
        let weak_component_node1 = Rc::downgrade(&component_node1);
        let update_message1 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(message),
            any_component_node: weak_component_node1,
        });

        let component2 = TestComponent2;
        let component_node2 = AnyComponentNode::new_root(component2, ancestor);
        let weak_component_node2 = Rc::downgrade(&component_node2);
        let update_message2 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(message),
            any_component_node: weak_component_node2,
        });

        // Act & Assert
        assert_ne!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn update_messages_from_the_same_component_type_but_different_instance_and_the_same_message_should_not_be_equal(
    ) {
        // Arrange
        let ancestor = get_body();
        let message = 0;

        let component1 = TestComponent;
        let component_node1 = AnyComponentNode::new_root(component1, ancestor.clone());
        let weak_component_node1 = Rc::downgrade(&component_node1);
        let update_message1 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(message),
            any_component_node: weak_component_node1,
        });

        let component2 = TestComponent;
        let component_node2 = AnyComponentNode::new_root(component2, ancestor);
        let weak_component_node2 = Rc::downgrade(&component_node2);
        let update_message2 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(message),
            any_component_node: weak_component_node2,
        });

        // Act & Assert
        assert_ne!(update_message1, update_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_the_same_component_should_be_equal() {
        // Arrange
        let depth = 0;
        let component = TestComponent;
        let ancestor = get_body();
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message1 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth,
        });
        let rerender_message2 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node,
            depth,
        });

        // Act & Assert
        assert_eq!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_the_same_component_with_different_depth_should_not_be_equal() {
        // Arrange
        let component = TestComponent;
        let ancestor = get_body();
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message1 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth: 0,
        });
        let rerender_message2 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node,
            depth: 1,
        });

        // Act & Assert
        assert_ne!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_the_same_component_type_but_different_instance_should_not_be_equal() {
        // Arrange
        let depth = 0;
        let ancestor = get_body();

        let component1 = TestComponent;
        let component_node1 = AnyComponentNode::new_root(component1, ancestor.clone());
        let weak_component_node1 = Rc::downgrade(&component_node1);
        let rerender_message1 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node1.clone(),
            depth,
        });

        let component2 = TestComponent;
        let component_node2 = AnyComponentNode::new_root(component2, ancestor);
        let weak_component_node2 = Rc::downgrade(&component_node2);
        let rerender_message2 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node2.clone(),
            depth,
        });

        // Act & Assert
        assert_ne!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_from_different_components_should_not_be_equal() {
        // Arrange
        let depth = 0;
        let ancestor = get_body();

        let component1 = TestComponent;
        let component_node1 = AnyComponentNode::new_root(component1, ancestor.clone());
        let weak_component_node1 = Rc::downgrade(&component_node1);
        let rerender_message1 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node1.clone(),
            depth,
        });

        let component2 = TestComponent2;
        let component_node2 = AnyComponentNode::new_root(component2, ancestor);
        let weak_component_node2 = Rc::downgrade(&component_node2);
        let rerender_message2 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node2.clone(),
            depth,
        });

        // Act & Assert
        assert_ne!(rerender_message1, rerender_message2);
    }

    #[wasm_bindgen_test]
    fn rerender_message_and_update_message_should_not_be_equal() {
        // Arrange
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth: 0,
        });
        let update_message = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(0),
            any_component_node: weak_component_node,
        });

        // Act & Assert
        assert_ne!(rerender_message, update_message);
    }

    // Tests for message cmp

    #[wasm_bindgen_test]
    fn update_message_should_be_greater_than_rerender_message() {
        // Arrange
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth: 0,
        });
        let update_message = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(0),
            any_component_node: weak_component_node,
        });

        // Act & Assert
        assert_eq!(
            update_message.cmp(&rerender_message),
            std::cmp::Ordering::Greater
        );
    }

    #[wasm_bindgen_test]
    fn rerender_message_should_be_less_than_rerender_message() {
        // Arrange
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth: 0,
        });
        let update_message = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(0),
            any_component_node: weak_component_node,
        });

        // Act & Assert
        assert_eq!(
            rerender_message.cmp(&update_message),
            std::cmp::Ordering::Less
        );
    }

    #[wasm_bindgen_test]
    fn update_message_should_be_equal_update_message() {
        // Arrange
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let update_message1 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(0),
            any_component_node: weak_component_node.clone(),
        });
        let update_message2 = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(1),
            any_component_node: weak_component_node,
        });

        // Act & Assert
        assert_eq!(
            update_message1.cmp(&update_message2),
            std::cmp::Ordering::Equal
        );
    }

    #[wasm_bindgen_test]
    fn rerender_message_with_smaller_depth_should_be_greater_than_rerender_message_with_bigger_depth(
    ) {
        // Arrange
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message1 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth: 0,
        });
        let rerender_message2 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node,
            depth: 1,
        });

        // Act & Assert
        assert_eq!(
            rerender_message1.cmp(&rerender_message2),
            std::cmp::Ordering::Greater
        );
    }

    #[wasm_bindgen_test]
    fn rerender_message_with_bigger_depth_should_be_less_than_rerender_message_with_smaller_depth()
    {
        // Arrange
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message1 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth: 0,
        });
        let rerender_message2 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node,
            depth: 1,
        });

        // Act & Assert
        assert_eq!(
            rerender_message2.cmp(&rerender_message1),
            std::cmp::Ordering::Less
        );
    }

    #[wasm_bindgen_test]
    fn rerender_messages_with_the_same_depth_should_be_equal() {
        // Arrange
        let ancestor = get_body();
        let depth = 0;
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let rerender_message1 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth,
        });
        let rerender_message2 = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node,
            depth,
        });

        // Act & Assert
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
        let ancestor = get_body();
        let component = UpdateReturnsTrueComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);

        let update_message = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(()),
            any_component_node: weak_component_node.clone(),
        });

        let expected_rerender_message = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth: 0,
        });

        // Act
        update_message.handle();

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 1);
            let rerender_message = scheduler.messages.iter().next().unwrap();
            assert_eq!(rerender_message, &expected_rerender_message);
        });
    }

    #[wasm_bindgen_test]
    fn handle_update_message_when_update_returns_false_should_not_add_rerender_message_to_schedulers_queue(
    ) {
        // Arrange
        clear_scheduler();
        let ancestor = get_body();
        let component = UpdateReturnsFalseComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);

        let update_message = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(()),
            any_component_node: weak_component_node.clone(),
        });

        // Act
        update_message.handle();

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 0);
        });
    }

    #[wasm_bindgen_test]
    fn handle_update_message_with_outdated_weak_reference_should_not_add_rerender_message_to_schedulers_queue(
    ) {
        // Arrange
        clear_scheduler();
        let ancestor = get_body();
        let component = UpdateReturnsTrueComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);

        let update_message = SchedulerMessage::Update(UpdateMessage {
            message: Box::new(()),
            any_component_node: weak_component_node.clone(),
        });

        // Act
        drop(component_node);
        update_message.handle();

        // Assert
        SCHEDULER_INSTANCE.with(|scheduler| {
            let scheduler = scheduler.borrow();
            assert_eq!(scheduler.messages.len(), 0);
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
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);
        let message = Box::new(0);

        let expected_update_message = SchedulerMessage::Update(UpdateMessage {
            message: message.clone(),
            any_component_node: weak_component_node.clone(),
        });

        // Act
        Scheduler::add_update_message(message, weak_component_node);

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
        let ancestor = get_body();
        let component = TestComponent;
        let component_node = AnyComponentNode::new_root(component, ancestor);
        let weak_component_node = Rc::downgrade(&component_node);

        let expected_rerender_message = SchedulerMessage::Rerender(RerenderMessage {
            any_component_node: weak_component_node.clone(),
            depth,
        });

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
