use syn::parse::{Parse, ParseStream};

use crate::rsx_macro::attributes::{
    normal_attribute::NormalAttribute, props_attribute::PropsAttribute, KEY_ATTR,
};

const PROPS_ATTR: &str = "props";

pub(crate) enum ComponentAttribute {
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
                    "Unsupported attribute `{ident}`. Custom components supports only `{PROPS_ATTR}` and `{KEY_ATTR}` attributes"
                ),
            ))
        }
    }
}

impl ComponentAttribute {
    pub(crate) fn peek(input: ParseStream) -> bool {
        input.peek(syn::Ident)
    }
}
