use std::collections::HashMap;

use self::{cleaners::clear_css, parsing_functions::parse_stylesheet, types::*};

mod cleaners;
mod parsing_functions;
mod types;

// generates css with prefixed selectors and stylesheet selector mapping
// pub fn generate_css_mapping(input: &str, prefix: &str) -> HashMap<String, String> {
//     // clear css form comments and collapse whitespaces
//     let cleared_input = clear_css(input);
//     let (_, stylesheet) = parse_stylesheet(&cleared_input).unwrap();

// }
