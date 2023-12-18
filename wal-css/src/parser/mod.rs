use std::collections::HashMap;

use self::{cleaners::clear_css, parsing_functions::parse_stylesheet};

mod cleaners;
mod parsing_functions;
mod types;

/// generates css with prefixed selectors and stylesheet selector mapping
pub(crate) fn process_css(input: &str, prefix: &str) -> (String, HashMap<String, String>) {
    let cleared_input = clear_css(input);
    let (_, stylesheet) = parse_stylesheet(&cleared_input).unwrap();

    let mapping: HashMap<String, String> = stylesheet.gen_mapping(prefix).into_iter().collect();
    let updated_css = stylesheet.gen_css(prefix);

    (updated_css, mapping)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::process_css;

    #[test]
    fn process_css_returns_correct_values() {
        let input = " 
        .class { 
            color: red; 
            background-color: yellow; 
        }

        #id { 
            color: green; 
            background-color: white; 
        }
        ";
        let prefix = "test-";

        let mut expected_mapping = HashMap::<String, String>::new();
        expected_mapping.insert("class".to_owned(), "test-class".to_owned());
        expected_mapping.insert("id".to_owned(), "test-id".to_owned());

        let expected_css = ".test-class { color: red; background-color: yellow; } #test-id { color: green; background-color: white; }".to_owned();

        assert_eq!((expected_css, expected_mapping), process_css(input, prefix));
    }
}
