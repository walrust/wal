// TODO
// change fields to private
// i should create this struct in the macro for every event that has MouseEvent as argument
macro_rules! event_creators {
    ($($handler_name:ident ($event_type:ty)),*) => {
        $(
            pub struct $handler_name {
                pub event_type: Cow<'static, str>,
                pub callback: Callback<$event_type>,
            }

            impl EventCreator for $handler_name {
                fn get_event_type(&self) -> Cow<'static, str> {
                    self.event_type.clone()
                }

                fn create_callback(&self) -> Box<dyn FnMut(&Event)> {
                    let callback = self.callback.clone();
                    Box::new(move |event: &Event| {
                        let event = event.clone().unchecked_into();
                        callback.emit(event);
                    })
                }
            }

            impl $handler_name {
                pub fn new(event_type: Cow<'static, str>, callback: Callback<$event_type>) -> Self {
                    Self {
                        event_type,
                        callback,
                    }
                }
            }

            impl EventCreatorType for $event_type {
                type Creator = $handler_name;
            }
        )*
    }
}

macro_rules! unspecialized_event_creators_constructor {
    ($($name:ident),*) => {
        $(
            fn $name(callback: Callback<Event>) -> Box<dyn EventCreator> {
                Box::new(UnspecializedEventCreator {
                    event_type: Cow::from(stringify!($name)[2..].to_string()),
                    callback,
                })
            }
        )*
    };
}

macro_rules! event_creators_constructor {
    ($($event_name:ident ($event_type:ty) ),*) => {
        $(
            fn $event_name(callback: Callback<$event_type>) -> Box<dyn EventCreator> {
                Box::new(<$event_type as EventCreatorType>::Creator::new(
                    Cow::from(stringify!($name)[2..].to_string()),
                    callback
                ))
            }
        )*
    };
}
