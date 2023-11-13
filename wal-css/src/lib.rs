pub mod css;
pub mod css_manager;
pub mod id_generator;
pub mod parser;

#[macro_export]
macro_rules! css_stylesheet {
    ($filepath: expr) => {
        $crate::css_manager::CssManager::new().attach_css(include_str!($filepath))
    };
}
