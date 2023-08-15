use syn::parse::Parse;

pub struct HtmlStartTag {
    lt: syn::token::Lt,
    pub name: proc_macro2::Ident,
    // properties: ElementProperties,
    slash: Option<syn::token::Slash>,
    gt: syn::token::Gt,
}

impl Parse for HtmlStartTag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let name = input.parse()?;
        // here parse properties
        let slash = input.parse().ok();
        let gt = input.parse()?;

        Ok(HtmlStartTag {
            lt,
            name,
            slash,
            gt,
        })
    }
}

impl HtmlStartTag {
    pub fn is_self_closing(&self) -> bool {
        self.slash.is_some()
    }

    pub fn is_void(&self) -> bool {
        self.name == "area"
            || self.name == "base"
            || self.name == "br"
            || self.name == "col"
            || self.name == "embed"
            || self.name == "hr"
            || self.name == "img"
            || self.name == "input"
            || self.name == "link"
            || self.name == "meta"
            || self.name == "param"
            || self.name == "source"
            || self.name == "track"
            || self.name == "wbr"
    }

    pub fn span(&self) -> proc_macro2::Span {
        self.lt.span.join(self.gt.span).unwrap_or(self.lt.span)
    }
}
