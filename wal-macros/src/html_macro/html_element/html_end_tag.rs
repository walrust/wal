use syn::{ext::IdentExt, parse::Parse};

pub struct HtmlEndTag {
    lt: syn::token::Lt,
    gt: syn::token::Gt,
}

impl Parse for HtmlEndTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        input.parse::<syn::token::Slash>()?;
        proc_macro2::Ident::parse_any(&input)?;
        let gt = input.parse()?;
        Ok(HtmlEndTag { lt, gt })
    }
}

impl HtmlEndTag {
    pub fn span(&self) -> proc_macro2::Span {
        self.lt.span.join(self.gt.span).unwrap_or(self.lt.span)
    }
}
