macro_rules! define_events {
    ($($event:ident),*) => {
        $(
            pub struct $event(web_sys::$event);

            impl Deref for $event {
                type Target = web_sys::$event;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl $event {
                #[allow(dead_code)]
                fn new(event: web_sys::$event) -> Self {
                    Self(event)
                }
            }
        )*
    };
}

macro_rules! event_creators {
    ($($handler_name:ident ($event_type:ty)),*) => {
        $(
            struct $handler_name {
                event_type: Cow<'static, str>,
                callback: Callback<$event_type>,
            }

            impl EventCreator for $handler_name {
                fn get_event_type(&self) -> Cow<'static, str> {
                    self.event_type.clone()
                }

                fn create_callback(&self) -> Box<dyn FnMut(&web_sys::Event)> {
                    let callback = self.callback.clone();
                    Box::new(move |event: &web_sys::Event| {
                        let event = <$event_type>::new(event.clone().unchecked_into());
                        callback.emit(event);
                    })
                }
            }

            impl $handler_name {
                fn new(event_type: Cow<'static, str>, callback: Callback<$event_type>) -> Self {
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
    ($($event_name:ident),*) => {
        $(
            #[allow(dead_code)]
            pub fn $event_name(callback: Callback<Event>) -> Box<dyn EventCreator> {
                Box::new(UnspecializedEventCreator {
                    event_type: Cow::from(stringify!($event_name)[2..].to_string()),
                    callback,
                })
            }
        )*
    };
}

macro_rules! event_creators_constructor {
    ($($event_name:ident ($event_type:ty) ),*) => {
        $(
            pub fn $event_name(callback: Callback<$event_type>) -> Box<dyn EventCreator> {
                Box::new(<$event_type as EventCreatorType>::Creator::new(
                    Cow::from(stringify!($event_name)[2..].to_string()),
                    callback
                ))
            }
        )*
    };
}
