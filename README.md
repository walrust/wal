# Wal

Front-end framework for creating Single Page Applications written in Rust. 

## About

Wal is a Rust library designed to allow the creaction of front-end web applications leveraging Rust and WebAssembly. It consists of three crates:
- `wal-core` - main crate which provides most fuinctionalities: components interface, routing, VDOM calculation, rendering and handling HTML events,
- `wal-rsx` - crate which provides the developer with useful HTML-like syntax called `rsx`,
- `wal-css` - crate which allows usage of local CSS styling with limited scopes.

The library allows creating SPA applications with the use of re-usable interactive components. 

## References
- documentation:
  - [wal-core](https://docs.rs/wal-core/latest/wal_core/)
  - [wal-rsx](https://docs.rs/wal-rsx/latest/wal_rsx/)
  - [wal-css](https://docs.rs/wal-css/latest/wal_css/)
- [tutorial (PL version)](https://github.com/walrust/tutorial-app)
- [project template](https://github.com/walrust/template)

## Used Technologies
- [WebAssembly](https://webassembly.org/)
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
- [Trunk](https://github.com/trunk-rs/trunk)
