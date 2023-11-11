use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::multispace0;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{combinator::map, sequence::pair, IResult};

use super::types::*;

fn p_id(i: &str) -> IResult<&str, Selector> {
    map(
        pair(tag("#"), take_till1(is_ident_terminator)),
        |(_, ident)| Selector::Id(ident),
    )(i)
}

fn p_class(i: &str) -> IResult<&str, Selector> {
    map(
        pair(tag("."), take_till1(is_ident_terminator)),
        |(_, ident)| Selector::Class(ident),
    )(i)
}

fn p_element(i: &str) -> IResult<&str, Selector> {
    map(take_till1(is_ident_terminator), |ident| {
        Selector::Element(ident)
    })(i)
}

fn p_selector(i: &str) -> IResult<&str, Selector> {
    map(alt((p_class, p_id, p_element)), |s| s)(i)
}

fn p_complex_selector(i: &str) -> IResult<&str, Instruction> {
    map(
        separated_list1(tuple((multispace0, tag(","), multispace0)), p_selector),
        |s| Instruction::ComplexSelector(s),
    )(i)
}

// fn p_special_instruction(i: &str) -> IResult<&str, Instruction> {
//     map(parser, f)(i)
// }

fn is_ident_terminator(c: char) -> bool {
    let terminators = " \t\n\r.#,$@%^&*(){}[]<>";
    terminators.contains(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_class_properly() {
        let (_, class) = p_class(".class1").unwrap();
        assert_eq!(class, Selector::Class("class1"))
    }
    #[test]
    fn gets_only_class_part() {
        let (rest, class) = p_class(".class1 rest-part").unwrap();
        assert_eq!(rest, " rest-part");
        assert_eq!(class, Selector::Class("class1"))
    }
    #[test]
    fn gets_class_till_separator() {
        let (rest, class) = p_class(".class1#rest-part").unwrap();
        assert_eq!(rest, "#rest-part");
        assert_eq!(class, Selector::Class("class1"))
    }

    #[test]
    fn gets_selector_class() {
        let (rest, class) = p_selector(".class").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Class("class"))
    }
    #[test]
    fn gets_selector_id() {
        let (rest, class) = p_selector("#id").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Id("id"))
    }
    #[test]
    fn gets_selector_element() {
        let (rest, class) = p_selector("body").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Element("body"))
    }
    #[test]
    fn gets_complex_selector() {
        let (rest, complex_selector) = p_complex_selector(".class, body , #id").unwrap();
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
