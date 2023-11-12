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
    WithoutBody(Instruction<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Stylesheet<'a> {
    sections: Vec<Section<'a>>,
}

impl<'a> Stylesheet<'a> {
    pub fn new(sections: Vec<Section<'a>>) -> Self {
        Stylesheet { sections }
    }
}
