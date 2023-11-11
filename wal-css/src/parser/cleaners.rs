use std::{cell::RefCell, rc::Rc};

use regex::Regex;

thread_local! {
    static COMMENTS_REGEX: Rc<RefCell<Regex>> =
        Rc::new(RefCell::new(Regex::new(r"/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/").unwrap()));
    static MULTIPLE_NEWLINES_REGEX: Rc<RefCell<Regex>> =
        Rc::new(RefCell::new(Regex::new(r"[ \t\r\n]+").unwrap()));
}

pub fn remove_comments(input: &str) -> String {
    COMMENTS_REGEX.with(|rgx| {
        rgx.as_ref()
            .borrow_mut()
            .replace_all(input, " ")
            .into_owned()
    })
}

pub fn collapse_whitespaces(input: &str) -> String {
    MULTIPLE_NEWLINES_REGEX.with(|rgx| {
        rgx.as_ref()
            .borrow_mut()
            .replace_all(input, " ")
            .into_owned()
    })
}

pub fn clear_css(input: &str) -> String {
    collapse_whitespaces(&remove_comments(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collapses_multiple_spaces() {
        assert_eq!(" abba ", collapse_whitespaces("  abba   "))
    }
    #[test]
    fn collapses_multiple_newlines() {
        assert_eq!(" abba ", collapse_whitespaces("\n\nabba\n\n\n"))
    }
    #[test]
    fn collapses_multiple_tabs() {
        assert_eq!(" abba ", collapse_whitespaces("\t\tabba\t"))
    }
    #[test]
    fn collapses_multiple_returns() {
        assert_eq!(" abba ", collapse_whitespaces("\r\rabba\r"))
    }
    #[test]
    fn collapses_mixed_whitespaces() {
        assert_eq!(
            " abba ",
            collapse_whitespaces(" \t \n\rabba \t    \n\n\r \r")
        )
    }

    #[test]
    fn removes_single_line_comments() {
        assert_eq!(" \nabba", remove_comments("/*this is a comment*/\nabba"))
    }
    #[test]
    fn removes_muti_line_comments() {
        assert_eq!(
            " \nabba",
            remove_comments("/*this \n is \n a \n comment*/\nabba")
        )
    }
    #[test]
    fn removes_intext_comments() {
        assert_eq!("ab ba", remove_comments("ab/*this is a comment*/ba"))
    }
    #[test]
    fn removes_comments_with_stars_and_slashes() {
        assert_eq!(
            "ab ba",
            remove_comments("ab/***this // is ///* / ** ** / a comment***/ba")
        )
    }

    #[test]
    fn clears_css_properly() {
        assert_eq!(
            " .container { background: yellow; } ",
            clear_css(
                "
            /* this is a comment */
            .container  {
                background: yellow;
                
            }
            "
            )
        )
    }
}
