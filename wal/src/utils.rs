
#[cfg(debug_assertions)]
pub mod debug {
    use gloo::{
        console::{log, warn}, dialogs,
    };

    #[inline]
    pub fn log(text: impl ToString) {
        log!(text.to_string())
    }

    #[inline]
    pub fn alert(text: impl ToString) {
        dialogs::alert(text.to_string().as_str())
    }

    #[inline]
    pub fn warn(text: impl ToString) {
        warn!(text.to_string().as_str())
    }
}

#[cfg(not(debug_assertions))]
pub mod debug {
    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn log(_text: impl ToString) {}

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn alert(_text: impl ToString) {}

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn warn(_text: impl ToString) {}
}
