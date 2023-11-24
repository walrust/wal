use syn::parse::{Parse, ParseStream};

use crate::rsx_macro::{
    attributes::{normal_attribute::NormalAttribute, props_attribute::PropsAttribute},
    KEY_ATTR, PROPS_ATTR,
};

pub enum ComponentAttribute {
    Props(PropsAttribute),
    Key(NormalAttribute),
}

impl Parse for ComponentAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let forked_input = input.fork();
        let ident: proc_macro2::Ident = forked_input.parse()?;

        if ident == PROPS_ATTR {
            Ok(ComponentAttribute::Props(input.parse()?))
        } else if ident == KEY_ATTR {
            Ok(ComponentAttribute::Key(input.parse()?))
        } else {
            Err(syn::Error::new(
                ident.span(),
                format!(
                    "Unsupported attribute `{}`. Custom components supports only `{}` and `{}` attributes",
                    ident,
                    PROPS_ATTR,
                    KEY_ATTR
                ),
            ))
        }
    }
}

impl ComponentAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(syn::Ident)
    }
}
