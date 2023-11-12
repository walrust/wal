use std::collections::HashMap;

use super::parsing_functions::parse_stylesheet;

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
// impl<'a> Selector<'a> {
//     pub fn gen_mapping(&self, prefix: &str) -> Option<(String, String)> {
//         match self {
//             Self::Class(class) => {
//                 return Some((class.to_owned(), format!("{}{}", prefix, class).to_owned()))
//             }
//             Self::Id(id) => return Some((id.to_owned(), format!("{}{}", prefix, id).to_owned())),
//             _ => return None,
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub enum Section<'a> {
    WithBody {
        instruction: Instruction<'a>,
        body: Body<'a>,
    },
    WithoutBody(Instruction<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Stylesheet<'a> {
    sections: Vec<Section<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Body<'a> {
    ParsedBody(Stylesheet<'a>),
    LiteralBody(&'a str),
}
impl<'a> Stylesheet<'a> {
    pub fn new(sections: Vec<Section<'a>>) -> Self {
        Stylesheet { sections }
    }

    // pub fn generate_mapping(&self, prefix: &str) -> HashMap<String, String> {
    //     let mut mapping = HashMap::<String, String>::new();
    //     for section in &self.sections {
    //         match section {
    //             // body instruction
    //             Section::WithBody { instruction, body } => match instruction {
    //                 // get mappings from selectors
    //                 Instruction::ComplexSelector(selectors) => {
    //                     // for each selector generate new mapping
    //                     for selector in selectors {
    //                         if let Some((key, val)) = selector.gen_mapping(prefix) {
    //                             mapping.insert(key, val);
    //                         }
    //                     }
    //                 }
    //                 // for some special instructions generate mappings form their body as well
    //                 Instruction::SpecialInstruction { command, .. } => {
    //                     if needs_nested_parsing(command) {
    //                         // parse the body as another stylesheet
    //                         let (_, sub_stylesheet) = parse_stylesheet(body).unwrap();
    //                         let sub_mapping = sub_stylesheet.generate_mapping(prefix);
    //                         // append sub_mapping to the current mapping
    //                         mapping.extend(sub_mapping);
    //                     }
    //                 }
    //             },
    //             // instructions without body don't have an influence on mapping
    //             Section::WithoutBody(_) => (),
    //         }
    //     }
    //     mapping
    // }
}

#[cfg(test)]
mod tests {}
