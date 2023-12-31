use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::root::Root;

pub(crate) struct IfExpression {
    rsx_if: If,
    else_ifs: Vec<ElseIf>,
    rsx_else: Option<Else>,
}

impl Parse for IfExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let rsx_if = input.parse()?;

        let mut else_ifs = Vec::new();
        while ElseIf::peek(input) {
            else_ifs.push(input.parse()?);
        }

        let rsx_else = if input.peek(syn::token::Else) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(IfExpression {
            rsx_if,
            else_ifs,
            rsx_else,
        })
    }
}

impl ToTokens for IfExpression {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let IfExpression {
            rsx_if,
            else_ifs,
            rsx_else,
        } = self;

        let else_tokens = match rsx_else {
            Some(rsx_else) => rsx_else.into_token_stream(),
            None => {
                let default_else_body = &Root::Empty;
                quote! { else { #default_else_body } }
            }
        };

        tokens.extend(quote_spanned! {rsx_if.if_token.span() =>
            #rsx_if
            #(#else_ifs)*
            #else_tokens
        });
    }
}

impl IfExpression {
    fn parse_condition(input: ParseStream) -> syn::Result<syn::Expr> {
        let condition = syn::Expr::parse_without_eager_brace(input)?;

        if let syn::Expr::Block(syn::ExprBlock { block, .. }) = &condition {
            if block.stmts.is_empty() {
                return Err(syn::Error::new_spanned(
                    &condition,
                    "Expected condition for `if` expression, found an empty block",
                ));
            }
        }

        Ok(condition)
    }
}

struct If {
    if_token: syn::token::If,
    condition: syn::Expr,
    body: IfBody,
}

impl Parse for If {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(If {
            if_token: input.parse::<syn::token::If>()?,
            condition: IfExpression::parse_condition(input)?,
            body: input.parse()?,
        })
    }
}

impl ToTokens for If {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let If {
            condition, body, ..
        } = self;

        tokens.extend(quote_spanned! {self.error_span() =>
            if #condition {
                #body
            }
        });
    }
}

impl If {
    fn error_span(&self) -> proc_macro2::Span {
        self.error_spanned().span()
    }

    fn error_spanned(&self) -> impl ToTokens {
        let If { body, if_token, .. } = self;
        quote!(#if_token #body)
    }
}

struct ElseIf {
    else_token: syn::token::Else,
    condition: syn::Expr,
    body: IfBody,
}

impl Parse for ElseIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let else_token = input.parse::<syn::token::Else>()?;
        input.parse::<syn::token::If>()?;

        Ok(ElseIf {
            else_token,
            condition: IfExpression::parse_condition(input)?,
            body: input.parse()?,
        })
    }
}

impl ElseIf {
    fn peek(input: ParseStream) -> bool {
        input.peek(syn::token::Else) && input.peek2(syn::token::If)
    }
}

impl ToTokens for ElseIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ElseIf {
            condition, body, ..
        } = self;

        tokens.extend(quote_spanned! {self.error_span() =>
            else if #condition {
                #body
            }
        });
    }
}

impl ElseIf {
    fn error_span(&self) -> proc_macro2::Span {
        self.error_spanned().span()
    }

    fn error_spanned(&self) -> impl ToTokens {
        let ElseIf {
            body, else_token, ..
        } = self;
        quote!(#else_token #body)
    }
}

struct Else {
    else_token: syn::token::Else,
    body: IfBody,
}

impl Parse for Else {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Else {
            else_token: input.parse::<syn::token::Else>()?,
            body: input.parse()?,
        })
    }
}

impl ToTokens for Else {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let body = &self.body;
        tokens.extend(quote_spanned! { self.error_span() =>
            else {
                #body
            }
        });
    }
}

impl Else {
    fn error_span(&self) -> proc_macro2::Span {
        self.error_spanned().span()
    }

    fn error_spanned(&self) -> impl ToTokens {
        let Else { body, else_token } = self;
        quote!(#else_token #body)
    }
}

struct IfBody(Root);

impl Parse for IfBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body_input;
        syn::braced!(body_input in input);
        Ok(IfBody(body_input.parse()?))
    }
}

impl ToTokens for IfBody {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let IfBody(body) = self;

        tokens.extend(quote! {
            #body
        });
    }
}
