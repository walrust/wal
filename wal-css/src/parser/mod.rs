use std::collections::HashMap;

use self::{cleaners::clear_css, types::*};

mod cleaners;
mod parsing_functions;
mod types;

// // generates css with prefixed selectors and stylesheet selector mapping
// pub fn parse_css(input: &str, prefix: &str) -> (String, HashMap<String, String>) {
//     // clear css form comments and collapse whitespaces
//     let cleared_input = clear_css(input);

//     // map(parser, f)
// }
