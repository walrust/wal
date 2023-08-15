use syn::{ext::IdentExt, parse::Parse};

pub struct HtmlEndTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    gt: syn::token::Gt,
}

impl Parse for HtmlEndTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        let name = proc_macro2::Ident::parse_any(&input)?;
        let gt = input.parse()?;
        Ok(HtmlEndTag { lt, name, gt })
    }
}

impl HtmlEndTag {
    pub fn span(&self) -> proc_macro2::Span {
        self.lt.span.join(self.gt.span).unwrap_or(self.lt.span)
    }
}
