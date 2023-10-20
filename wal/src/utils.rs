use gloo::{
    console::{log, warn},
    dialogs::alert,
};

#[cfg(debug_assertions)]
#[inline]
pub fn debug_log(text: impl ToString) {
    log!(text.to_string())
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_log(_text: impl ToString) {}

#[cfg(debug_assertions)]
#[inline]
pub fn debug_alert(text: &str) {
    alert(text)
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_alert(_text: impl ToString) {}

#[cfg(debug_assertions)]
#[inline]
pub fn debug_warn(text: &str) {
    warn!(text)
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_warn(_text: impl ToString) {}
