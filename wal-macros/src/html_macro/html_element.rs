use proc_macro2::Ident;
use syn::{ext::IdentExt, parse::Parse};

pub struct HtmlElement {
    name: Ident,
    //props: ElementProps,
    //children: Vec<HtmlTree>
}

impl Parse for HtmlElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Slash) {
            return match input.parse::<HtmlEndTag>() {
                Ok(html_end_tag) => Err(syn::Error::new(
                    html_end_tag.span(),
                    "This closing tag does not have corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        unimplemented!();
    }
}

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
    fn span(&self) -> proc_macro2::Span {
        self.lt.span.join(self.gt.span).unwrap_or(self.lt.span)
    }
}
