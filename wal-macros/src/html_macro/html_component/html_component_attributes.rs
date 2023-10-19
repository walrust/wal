use syn::parse::Parse;

use crate::html_macro::html_attribute::HtmlAttribute;

pub struct HtmlComponentAttributes {
    pub props: Option<HtmlAttribute>,
    key: Option<HtmlAttribute>,
}

impl Parse for HtmlComponentAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut props = None;
        let mut key = None;
        while HtmlAttribute::peek(input) {
            let attribute = input.parse::<HtmlAttribute>()?;
            if attribute.ident == "props" {
                if props.is_some() {
                    return Err(syn::Error::new(
                        attribute.ident.span(),
                        format!("Duplicate attribute `{}`", attribute.ident),
                    ));
                }
                props = Some(attribute);
            } else if attribute.ident == "key" {
                if key.is_some() {
                    return Err(syn::Error::new(
                        attribute.ident.span(),
                        format!("Duplicate attribute `{}`", attribute.ident),
                    ));
                }
                key = Some(attribute);
            } else {
                return Err(syn::Error::new(
                    attribute.ident.span(),
                    format!(
                        "Unsupported attribute `{}`. Custom components supports only `props` and `key` attributes",
                        attribute.ident
                    ),
                ));
            }
        }
        Ok(HtmlComponentAttributes { props, key })
    }
}
