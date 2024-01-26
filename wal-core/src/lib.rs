//! This crate provides the core functionalities of wal library, which are essential to build a web application.
//! This includes component, events, routing and virtual dom.

/// Module `component` provides the `Component` trait, which is the core of wal library. It enables for creating components, which are the building blocks of a web application.
pub mod component;
/// Module `events` provides the event functions and types, which is used to handle events in a web application.
pub mod events;
/// Module `router` provides the `PageRenderer` and `Router` structs, which are used to run application and handle routing.
pub mod router;
pub(crate) mod utils;
/// Module `virtual_dom` provides functions and types related to VDOM. VDOM is later translated into real DOM, that is rendered in the browser.
pub mod virtual_dom;
