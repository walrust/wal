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

    // pub fn span(&self) -> proc_macro2::Span {
    //     self.to_spanned().span()
    // }

    // fn to_spanned(&self) -> Box<dyn ToTokens> {
    //     match self {
    //         ComponentAttribute::Props(props) => Box::new(props.to_spanned()),
    //         ComponentAttribute::Key(key) => Box::new(key.to_spanned()),
    //     }
    // }
}

// impl ToTokens for HtmlComponentAttributeValue {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         match self {
//             HtmlComponentAttributeValue::Literal(lit) => lit.to_tokens(tokens),
//             HtmlComponentAttributeValue::StructExpression(expr_struct) => {
//                 expr_struct.to_tokens(tokens)
//             }
//             HtmlComponentAttributeValue::ExpressionBlock(expr_block) => {
//                 expr_block.to_tokens(tokens)
//             }
//         }
//     }
// }
