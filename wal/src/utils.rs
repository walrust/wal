use core::fmt;

use gloo::{
    console::{error, log, warn},
    dialogs::alert,
};

#[cfg(debug_assertions)]
#[inline]
pub fn debug_log(text: impl ToString) {
    log!(text.to_string())
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_log(text: impl ToString) {}

#[cfg(debug_assertions)]
#[inline]
pub fn debug_alert(text: &str) {
    alert(text)
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_alert(text: impl ToString) {}

#[cfg(debug_assertions)]
#[inline]
pub fn debug_warn(text: &str) {
    warn!(text)
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_alert(text: impl ToString) {}


pub trait WasmUtils<T> {
    fn wasm_expect(self, msg: &str) -> T;
    fn wasm_unwrap(self) -> T;
}

impl<T> WasmUtils<T> for Option<T> {
    #[inline]
    fn wasm_expect(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                error!(msg);
                panic!()
            }
        }
    }

    #[inline]
    fn wasm_unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => {
                error!("called `Option::unwrap()` on a `None` value");
                panic!()
            }
        }
    }
}

impl<T, E: fmt::Debug> WasmUtils<T> for Result<T, E> {
    #[inline]
    fn wasm_expect(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                error!(format!("{}: {:?}", msg, e));
                panic!()
            }
        }
    }

    #[inline]
    fn wasm_unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                error!(format!("called `Result::unwrap()` on an `Err` value: {:?}", &e));
                panic!()
            },
        }
    }
}
