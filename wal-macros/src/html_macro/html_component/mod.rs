use syn::parse::Parse;

use self::html_component_attributes::HtmlComponentAttributes;

mod html_component_attributes;

pub struct HtmlComponent {
    lt: syn::token::Lt,
    ty: syn::Type,
    attributes: HtmlComponentAttributes,
    gt: syn::token::Gt,
}

impl Parse for HtmlComponent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let ty = input.parse()?;
        let attributes = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let gt = input.parse()?;

        Ok(HtmlComponent {
            lt,
            ty,
            attributes,
            gt,
        })
    }
}
