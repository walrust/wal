//! This crate provides CSS manager for global styling that can be used with
//! [wal-core](../wal_core/index.html) crate along with [css_stylesheet] macro.
pub mod css;

pub mod css_manager;

mod id_generator;
mod parser;

/// css_stylesheet macro allows to attach new css stylesheets to the application directly from CSS files.
///
/// To attach CSS file as a stylesheet, call the macro providing the relative path to the file.
/// It will return the [Css](./css/struct.Css.html) object, which can be used to reference the stylesheet selectors
/// inside the [rsx macro](../wal_rsx/macro.rsx.html)
///  
/// # Example usage
/// ```
/// use wal_css::css:Css;
/// use wal_css::css_stylesheet;
///
/// thread_local! {
///     static CSS: Css = css_stylesheet!("path-to-css-file");
/// }
/// // ...
/// CSS.with(|css| {
///     rsx! {
///         <div class={format!("{} {}", css["class1"], css["class2"])} />
///     }
/// })
/// ```
///
#[macro_export]
macro_rules! css_stylesheet {
    ($filepath: expr) => {
        $crate::css_manager::CssManager::new().attach_css(include_str!($filepath))
    };
}
