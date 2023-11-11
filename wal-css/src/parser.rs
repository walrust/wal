use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::multispace0;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{combinator::map, sequence::pair, IResult};
use std::collections::HashMap;

// pub enum Token<'a> {
//     Id {
//         hash: &'a str,
//         name: &'a str,
//     },
//     Class {
//         dot: &'a str,
//         name: &'a str,
//     },
//     Element {
//         name: &'a str,
//     },
//     Ruleset {
//         opening_brace: &'a str,
//         body: &'a str,
//         closing_brace: &'a str,
//     },
//     Other(&'a str),
// }

#[derive(Debug, PartialEq)]
pub enum Instruction<'a> {
    ComplexSelector(Vec<Selector<'a>>),
    SpecialInstruction {
        command: &'a str,
        parameters: &'a str,
    },
}
#[derive(Debug, PartialEq)]
pub enum Selector<'a> {
    Id(&'a str),
    Class(&'a str),
    Element(&'a str),
}
#[derive(Debug, PartialEq)]
pub enum Section<'a> {
    WithBody {
        instruction: Instruction<'a>,
        body: &'a str,
    },
    WithoutBody(&'a str),
}

/// generates css with prefixed selectors and stylesheet selector mapping
// pub fn generate_css_with_ids(input: &str, prefix: &str) -> (String, HashMap<String, String>) {
//     map(parser, f)
// }

// fn id_prefixer_parser(input: &str) -> IResult<&str,>

fn get_id(i: &str) -> IResult<&str, Selector> {
    map(
        pair(tag("#"), take_till1(is_ident_terminator)),
        |(_, ident)| Selector::Id(ident),
    )(i)
}

fn get_class(i: &str) -> IResult<&str, Selector> {
    map(
        pair(tag("."), take_till1(is_ident_terminator)),
        |(_, ident)| Selector::Class(ident),
    )(i)
}

fn get_element(i: &str) -> IResult<&str, Selector> {
    map(take_till1(is_ident_terminator), |ident| {
        Selector::Element(ident)
    })(i)
}

fn get_selector(i: &str) -> IResult<&str, Selector> {
    map(alt((get_class, get_id, get_element)), |s| s)(i)
}

fn get_complex_selector(i: &str) -> IResult<&str, Instruction> {
    map(
        separated_list1(tuple((multispace0, tag(","), multispace0)), get_selector),
        |s| Instruction::ComplexSelector(s),
    )(i)
}

fn is_ident_terminator(c: char) -> bool {
    let terminators = " \t\n\r.#,$@%^&*(){}[]<>";
    terminators.contains(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_class_properly() {
        let (_, class) = get_class(".class1").unwrap();
        assert_eq!(class, Selector::Class("class1"))
    }
    #[test]
    fn gets_only_class_part() {
        let (rest, class) = get_class(".class1 rest-part").unwrap();
        assert_eq!(rest, " rest-part");
        assert_eq!(class, Selector::Class("class1"))
    }
    #[test]
    fn gets_class_till_separator() {
        let (rest, class) = get_class(".class1#rest-part").unwrap();
        assert_eq!(rest, "#rest-part");
        assert_eq!(class, Selector::Class("class1"))
    }

    #[test]
    fn gets_selector_class() {
        let (rest, class) = get_selector(".class").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Class("class"))
    }
    #[test]
    fn gets_selector_id() {
        let (rest, class) = get_selector("#id").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Id("id"))
    }
    #[test]
    fn gets_selector_element() {
        let (rest, class) = get_selector("body").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Element("body"))
    }
    #[test]
    fn gets_complex_selector() {
        let (rest, complex_selector) = get_complex_selector(".class, body , #id").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            complex_selector,
            Instruction::ComplexSelector(vec![
                Selector::Class("class"),
                Selector::Element("body"),
                Selector::Id("id")
            ])
        )
    }
}
