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
pub fn debug_alert(text: impl ToString) {
    alert(text.to_string().as_str())
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_alert(_text: impl ToString) {}

#[cfg(debug_assertions)]
#[inline]
pub fn debug_warn(text: impl ToString) {
    warn!(text.to_string().as_str())
}
#[cfg(not(debug_assertions))]
#[inline]
pub fn debug_warn(_text: impl ToString) {}
