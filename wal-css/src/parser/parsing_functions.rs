use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_till1, take_until};
use nom::character::complete::{multispace0, multispace1};
use nom::error::{Error, ErrorKind, ParseError};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{combinator::map, sequence::pair, Err, IResult};

use super::types::*;

/// parses css string into Stylesheet object
pub fn parse_stylesheet(i: &str) -> IResult<&str, Stylesheet> {
    map(separated_list0(multispace1, p_section), Stylesheet::new)(i)
}

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
        Instruction::ComplexSelector,
    )(i)
}

fn p_special_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        pair(
            pair(tag("@"), take_until(" ")),      // ex @media
            take_till(is_instruction_terminator), // ex. not all and (hover: hover)
        ),
        |((_, command), parameters)| Instruction::SpecialInstruction {
            command,
            parameters,
        },
    )(i)
}

fn p_instruction(i: &str) -> IResult<&str, Instruction> {
    map(alt((p_complex_selector, p_special_instruction)), |s| s)(i)
}

fn p_body(i: &str) -> IResult<&str, &str> {
    map(
        delimited(tag("{"), p_until_unbalanced('{', '}'), tag("}")),
        |body| body,
    )(i)
}

fn p_body_section(i: &str) -> IResult<&str, Section> {
    let (remainig_input, (instruction, body_str)) =
        map(separated_pair(p_instruction, multispace0, p_body), |r| r)(i)?;

    // parse body recursively if needed
    if let Instruction::SpecialInstruction { command, .. } = instruction {
        if needs_nested_parsing(command) {
            let parsed_body = parse_stylesheet(body_str.trim())?.1;
            return Ok((
                remainig_input,
                Section::WithBody {
                    instruction,
                    body: Body::ParsedBody(parsed_body),
                },
            ));
        }
    }
    // if not return section with literal body
    Ok((
        remainig_input,
        Section::WithBody {
            instruction,
            body: Body::LiteralBody(body_str),
        },
    ))
}

fn p_bodyless_section(i: &str) -> IResult<&str, Section> {
    map(pair(p_instruction, tag(";")), |(instr, _)| {
        Section::WithoutBody(instr)
    })(i)
}

fn p_section(i: &str) -> IResult<&str, Section> {
    map(alt((p_body_section, p_bodyless_section)), |s| s)(i)
}
/// used to parse self nested expression delimited with brackets
pub fn p_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                // Openieng bracket
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                // Closing bracket
                c if c == closing_bracket => {
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}

fn is_ident_terminator(c: char) -> bool {
    let terminators = " \t\n\r.#,$@%^&*(){}[]<>";
    terminators.contains(c)
}

fn is_instruction_terminator(c: char) -> bool {
    let terminators = ";{}";
    terminators.contains(c)
}

fn needs_nested_parsing(command: &str) -> bool {
    let commands_to_parse = ["media", "scope", "supports", "document"];
    commands_to_parse.contains(&command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_class_properly() {
        let (_, class) = p_class(".class1").unwrap();
        assert_eq!(class, Selector::Class("class1"))
    }
    #[test]
    fn parses_only_class_part() {
        let (rest, class) = p_class(".class1 rest-part").unwrap();
        assert_eq!(rest, " rest-part");
        assert_eq!(class, Selector::Class("class1"))
    }
    #[test]
    fn parses_class_till_separator() {
        let (rest, class) = p_class(".class1#rest-part").unwrap();
        assert_eq!(rest, "#rest-part");
        assert_eq!(class, Selector::Class("class1"))
    }
    #[test]
    fn parses_selector_class() {
        let (rest, class) = p_selector(".class").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Class("class"))
    }
    #[test]
    fn parses_selector_id() {
        let (rest, class) = p_selector("#id").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Id("id"))
    }
    #[test]
    fn parses_selector_element() {
        let (rest, class) = p_selector("body").unwrap();
        assert_eq!(rest, "");
        assert_eq!(class, Selector::Element("body"))
    }
    #[test]
    fn parses_complex_selector() {
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
    #[test]
    fn parses_special_complex_instruction() {
        let (rest, spec_instr) = p_special_instruction("@media (hover: hover) { }").unwrap();
        assert_eq!(rest, "{ }");
        assert_eq!(
            spec_instr,
            Instruction::SpecialInstruction {
                command: "media",
                parameters: " (hover: hover) "
            }
        )
    }
    #[test]
    fn parses_special_simple_instruction() {
        let (rest, spec_instr) =
            p_special_instruction("@namespace svg url('http://www.w3.org/2000/svg')").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            spec_instr,
            Instruction::SpecialInstruction {
                command: "namespace",
                parameters: " svg url('http://www.w3.org/2000/svg')"
            }
        )
    }
    #[test]
    fn parses_unnested_body() {
        let (rest, body) = p_body("{this is my body}").unwrap();
        assert_eq!(rest, "");
        assert_eq!(body, "this is my body")
    }
    #[test]
    fn parses_nested_body() {
        let (rest, body) = p_body("{ color: red; &:hover { color: green } }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(body, " color: red; &:hover { color: green } ")
    }
    #[test]
    fn parses_nested_body_leaving_rest() {
        let (rest, body) = p_body("{ color: red; &:hover { color: green; } } }").unwrap();
        assert_eq!(rest, " }");
        assert_eq!(body, " color: red; &:hover { color: green; } ")
    }
    #[test]
    fn parses_selector_section_with_body() {
        let (rest, section) = p_body_section(".class { color: green; }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            section,
            Section::WithBody {
                instruction: Instruction::ComplexSelector(vec![Selector::Class("class")]),
                body: Body::LiteralBody(" color: green; ")
            }
        )
    }
    #[test]
    fn parses_special_section_with_body() {
        let (rest, section) =
            p_body_section("@media (hover: hover) { .class { color: green; } }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            section,
            Section::WithBody {
                instruction: Instruction::SpecialInstruction {
                    command: "media",
                    parameters: " (hover: hover) "
                },
                body: Body::ParsedBody(Stylesheet::new(vec![Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Class("class")]),
                    body: Body::LiteralBody(" color: green; ")
                }]))
            }
        )
    }
    #[test]
    fn parses_special_section_without_body() {
        let (rest, section) =
            p_bodyless_section("@namespace svg url('http://www.w3.org/2000/svg');").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            section,
            Section::WithoutBody(Instruction::SpecialInstruction {
                command: "namespace",
                parameters: " svg url('http://www.w3.org/2000/svg')"
            })
        )
    }
    #[test]
    fn parses_section() {
        let (rest, section) = p_section(".class { color: green; }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            section,
            Section::WithBody {
                instruction: Instruction::ComplexSelector(vec![Selector::Class("class")]),
                body: Body::LiteralBody(" color: green; "),
            }
        )
    }
    #[test]
    fn parses_basic_stylesheet() {
        let (rest, stylesheet) =
            parse_stylesheet(".class1 { color: red; } .class2 { color: green; }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            stylesheet,
            Stylesheet::new(vec![
                Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Class("class1")]),
                    body: Body::LiteralBody(" color: red; "),
                },
                Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Class("class2")]),
                    body: Body::LiteralBody(" color: green; "),
                },
            ])
        )
    }
    #[test]
    fn parses_stylesheet_with_mixed_instructions() {
        let (rest, stylesheet) =
            parse_stylesheet("@namespace svg url('http://www.w3.org/2000/svg'); .class1 { color: red; } #id1 { color: green; }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            stylesheet,
            Stylesheet::new(vec![
                Section::WithoutBody(Instruction::SpecialInstruction {
                    command: "namespace",
                    parameters: " svg url('http://www.w3.org/2000/svg')"
                }),
                Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Class("class1")]),
                    body: Body::LiteralBody(" color: red; "),
                },
                Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Id("id1")]),
                    body: Body::LiteralBody(" color: green; "),
                },
            ])
        )
    }
    #[test]
    fn parses_stylesheet_with_nested_instructions() {
        let (rest, stylesheet) =
            parse_stylesheet("@media (hover: hover) { .class1 { color: green; } } .class1 { color: red; } #id1 { color: green; }").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            stylesheet,
            Stylesheet::new(vec![
                Section::WithBody {
                    instruction: Instruction::SpecialInstruction {
                        command: "media",
                        parameters: " (hover: hover) "
                    },
                    body: Body::ParsedBody(Stylesheet::new(vec![Section::WithBody {
                        instruction: Instruction::ComplexSelector(vec![Selector::Class("class1")]),
                        body: Body::LiteralBody(" color: green; ")
                    }]))
                },
                Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Class("class1")]),
                    body: Body::LiteralBody(" color: red; "),
                },
                Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Id("id1")]),
                    body: Body::LiteralBody(" color: green; "),
                },
            ])
        )
    }
}
