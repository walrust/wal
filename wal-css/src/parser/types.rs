use std::{collections::HashMap, fmt::format};

use super::parsing_functions::parse_stylesheet;

#[derive(Debug, PartialEq)]
pub enum Instruction<'a> {
    ComplexSelector(Vec<Selector<'a>>),
    SpecialInstruction {
        command: &'a str,
        parameters: &'a str,
    },
}
impl<'a> Instruction<'a> {
    pub fn gen_mapping(&self, prefix: &str) -> Vec<(String, String)> {
        if let Instruction::ComplexSelector(selectors) = self {
            return selectors
                .iter()
                .filter_map(|s| s.gen_mapping(prefix))
                .collect();
        }
        vec![]
    }
    pub fn gen_css(&self, prefix: &str) -> String {
        match self {
            Instruction::ComplexSelector(selectors) => selectors
                .iter()
                .map(|s| s.gen_css(prefix))
                .collect::<Vec<String>>()
                .join(", "),
            Instruction::SpecialInstruction {
                command,
                parameters,
            } => format!("{command}{parameters}").to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Selector<'a> {
    Id(&'a str),
    Class(&'a str),
    Element(&'a str),
}
impl<'a> Selector<'a> {
    pub fn gen_mapping(&self, prefix: &str) -> Option<(String, String)> {
        match self {
            Selector::Id(id) => Some((id.to_string(), format!("{}{}", prefix, id))),
            Selector::Class(class) => Some((class.to_string(), format!("{}{}", prefix, class))),
            Selector::Element(_) => None,
        }
    }
    pub fn gen_css(&self, prefix: &str) -> String {
        match self {
            Selector::Id(id) => format!("#{}{}", prefix, id).to_owned(),
            Selector::Class(class) => format!(".{}{}", prefix, class).to_owned(),
            Selector::Element(element) => element.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Section<'a> {
    WithBody {
        instruction: Instruction<'a>,
        body: Body<'a>,
    },
    WithoutBody(Instruction<'a>),
}
impl<'a> Section<'a> {
    pub fn gen_mapping(&self, prefix: &str) -> Vec<(String, String)> {
        if let Section::WithBody { instruction, body } = self {
            let mut mapping = instruction.gen_mapping(prefix);

            if let Body::ParsedBody(stylesheet) = body {
                mapping.extend(stylesheet.gen_mapping(prefix));
            }
            return mapping;
        }
        vec![]
    }
    pub fn gen_css(&self, prefix: &str) -> String {
        match self {
            Section::WithBody { instruction, body } => format!(
                "{} {{ {} }}",
                instruction.gen_css(prefix),
                body.gen_css(prefix)
            )
            .to_owned(),
            Section::WithoutBody(instruction) => {
                format!("{};", instruction.gen_css(prefix)).to_owned()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Body<'a> {
    ParsedBody(Stylesheet<'a>),
    LiteralBody(&'a str),
}
impl<'a> Body<'a> {
    pub fn gen_css(&self, prefix: &str) -> String {
        match self {
            Body::LiteralBody(body_str) => body_str.to_string(),
            Body::ParsedBody(stylesheet) => stylesheet.gen_css(prefix),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Stylesheet<'a> {
    sections: Vec<Section<'a>>,
}
impl<'a> Stylesheet<'a> {
    pub fn new(sections: Vec<Section<'a>>) -> Self {
        Stylesheet { sections }
    }
    pub fn gen_mapping(&self, prefix: &str) -> Vec<(String, String)> {
        self.sections
            .iter()
            .flat_map(|s| s.gen_mapping(prefix))
            .collect::<Vec<(String, String)>>()
    }
    pub fn gen_css(&self, prefix: &str) -> String {
        self.sections
            .iter()
            .map(|s| s.gen_css(prefix))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::types::{Body, Instruction, Section, Stylesheet};

    use super::Selector;

    #[test]
    fn class_selector_gens_correct_mapping() {
        let selector = Selector::Class("class");
        let prefix = "test-";
        let expected = Some(("class".to_owned(), "test-class".to_owned()));

        assert_eq!(expected, selector.gen_mapping(prefix))
    }
    #[test]
    fn id_selector_gens_correct_mapping() {
        let selector = Selector::Id("id");
        let prefix = "test-";
        let expected = Some(("id".to_owned(), "test-id".to_owned()));

        assert_eq!(expected, selector.gen_mapping(prefix))
    }
    #[test]
    fn element_selector_does_not_gen_mapping() {
        let selector = Selector::Element("body");
        let prefix = "test-";
        let expected = None;

        assert_eq!(expected, selector.gen_mapping(prefix))
    }
    #[test]
    fn instruction_complex_selector_gens_correct_mapping() {
        let instruction = Instruction::ComplexSelector(vec![
            Selector::Class("class"),
            Selector::Element("body"),
            Selector::Id("id"),
        ]);
        let prefix = "test-";
        let expected = vec![
            ("class".to_owned(), "test-class".to_owned()),
            ("id".to_owned(), "test-id".to_owned()),
        ];

        assert_eq!(expected, instruction.gen_mapping(prefix))
    }
    #[test]
    fn instruction_special_insruction_gens_no_mapping() {
        let instruction = Instruction::SpecialInstruction {
            command: "namespace",
            parameters: " svg url('http://www.w3.org/2000/svg')",
        };
        let prefix = "test-";
        let expected: Vec<(String, String)> = vec![];

        assert_eq!(expected, instruction.gen_mapping(prefix))
    }
    #[test]
    fn section_with_literal_body_gens_correct_mapping() {
        let section = Section::WithBody {
            instruction: Instruction::ComplexSelector(vec![Selector::Class("class")]),
            body: Body::LiteralBody(" color: red; "),
        };
        let prefix = "test-";
        let expected = vec![("class".to_owned(), "test-class".to_owned())];

        assert_eq!(expected, section.gen_mapping(prefix))
    }
    #[test]
    fn section_with_parsed_body_gens_correct_mapping() {
        let section = Section::WithBody {
            instruction: Instruction::SpecialInstruction {
                command: "media",
                parameters: " (hover: hover) ",
            },
            body: Body::ParsedBody(Stylesheet::new(vec![Section::WithBody {
                instruction: Instruction::ComplexSelector(vec![Selector::Class("class")]),
                body: Body::LiteralBody(" color: green; "),
            }])),
        };
        let prefix = "test-";
        let expected = vec![("class".to_owned(), "test-class".to_owned())];

        assert_eq!(expected, section.gen_mapping(prefix))
    }
    #[test]
    fn section_without_body_gens_no_mapping() {
        let section = Section::WithoutBody(Instruction::SpecialInstruction {
            command: "namespace",
            parameters: " svg url('http://www.w3.org/2000/svg')",
        });
        let prefix = "test-";
        let expected: Vec<(String, String)> = vec![];

        assert_eq!(expected, section.gen_mapping(prefix))
    }
    #[test]
    fn stylesheet_gens_correct_mapping() {
        let stylesheet = Stylesheet::new(vec![
            Section::WithBody {
                instruction: Instruction::SpecialInstruction {
                    command: "media",
                    parameters: " (hover: hover) ",
                },
                body: Body::ParsedBody(Stylesheet::new(vec![Section::WithBody {
                    instruction: Instruction::ComplexSelector(vec![Selector::Class("class1")]),
                    body: Body::LiteralBody(" color: green; "),
                }])),
            },
            Section::WithBody {
                instruction: Instruction::ComplexSelector(vec![Selector::Class("class2")]),
                body: Body::LiteralBody(" color: red; "),
            },
            Section::WithBody {
                instruction: Instruction::ComplexSelector(vec![Selector::Id("id1")]),
                body: Body::LiteralBody(" color: green; "),
            },
        ]);
        let prefix = "test-";
        let expected = vec![
            ("class1".to_owned(), "test-class1".to_owned()),
            ("class2".to_owned(), "test-class2".to_owned()),
            ("id1".to_owned(), "test-id1".to_owned()),
        ];

        assert_eq!(expected, stylesheet.gen_mapping(prefix))
    }
    #[test]
    fn class_selector_gens_correct_css() {
        let selector = Selector::Class("class");
        let prefix = "test-";
        let expected = ".test-class".to_owned();

        assert_eq!(expected, selector.gen_css(prefix))
    }
    #[test]
    fn id_selector_gens_correct_css() {
        let selector = Selector::Id("id");
        let prefix = "test-";
        let expected = "#test-id".to_owned();

        assert_eq!(expected, selector.gen_css(prefix))
    }
    #[test]
    fn element_selector_gens_correct_css() {
        let selector = Selector::Element("body");
        let prefix = "test-";
        let expected = "body";

        assert_eq!(expected, selector.gen_css(prefix))
    }
    #[test]
    fn instruction_complex_selector_gens_correct_css() {
        let instruction = Instruction::ComplexSelector(vec![
            Selector::Class("class"),
            Selector::Element("body"),
            Selector::Id("id"),
        ]);
        let prefix = "test-";
        let expected = ".test-class, body, #test-id".to_owned();

        assert_eq!(expected, instruction.gen_css(prefix))
    }
    #[test]
    fn instruction_special_insruction_gens_correct_css() {
        let instruction = Instruction::SpecialInstruction {
            command: "namespace",
            parameters: " svg url('http://www.w3.org/2000/svg')",
        };
        let prefix = "test-";
        let expected = "namespace svg url('http://www.w3.org/2000/svg')".to_owned();

        assert_eq!(expected, instruction.gen_css(prefix))
    }
}
