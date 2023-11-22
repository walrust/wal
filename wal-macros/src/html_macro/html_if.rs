use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::html_root::HtmlRoot;

pub struct HtmlIfExpression {
    html_if: HtmlIf,
    else_ifs: Vec<HtmlElseIf>,
    html_else: Option<HtmlElse>,
}

impl Parse for HtmlIfExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let html_if = input.parse()?;

        let mut else_ifs = Vec::new();
        while HtmlElseIf::peek(input) {
            else_ifs.push(input.parse()?);
        }

        let html_else = if input.peek(syn::token::Else) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(HtmlIfExpression {
            html_if,
            else_ifs,
            html_else,
        })
    }
}

impl ToTokens for HtmlIfExpression {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlIfExpression {
            html_if,
            else_ifs,
            html_else,
        } = self;

        let else_tokens = match html_else {
            Some(html_else) => html_else.into_token_stream(),
            None => {
                let default_else_body = &HtmlRoot::Empty;
                quote! { else { #default_else_body } }
            }
        };

        tokens.extend(quote_spanned! {html_if.if_token.span() =>
            #html_if
            #(#else_ifs)*
            #else_tokens
        });
    }
}

impl HtmlIfExpression {
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

struct HtmlIf {
    if_token: syn::token::If,
    condition: syn::Expr,
    body: HtmlIfBody,
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(HtmlIf {
            if_token: input.parse::<syn::token::If>()?,
            condition: HtmlIfExpression::parse_condition(input)?,
            body: input.parse()?,
        })
    }
}

impl ToTokens for HtmlIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlIf {
            condition,
            body,
            if_token,
        } = self;

        let spanned = quote!(#if_token #body);

        tokens.extend(quote_spanned! {spanned.span() =>
            if #condition {
                #body
            }
        });
    }
}

struct HtmlElseIf {
    else_token: syn::token::Else,
    condition: syn::Expr,
    body: HtmlIfBody,
}

impl Parse for HtmlElseIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let else_token = input.parse::<syn::token::Else>()?;
        input.parse::<syn::token::If>()?;

        Ok(HtmlElseIf {
            else_token,
            condition: HtmlIfExpression::parse_condition(input)?,
            body: input.parse()?,
        })
    }
}

impl HtmlElseIf {
    fn peek(input: ParseStream) -> bool {
        input.peek(syn::token::Else) && input.peek2(syn::token::If)
    }
}

impl ToTokens for HtmlElseIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlElseIf {
            condition,
            body,
            else_token,
        } = self;

        let spanned = quote!(#else_token #body);

        tokens.extend(quote_spanned! {spanned.span() =>
            else if #condition {
                #body
            }
        });
    }
}

struct HtmlElse {
    else_token: syn::token::Else,
    body: HtmlIfBody,
}

impl Parse for HtmlElse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(HtmlElse {
            else_token: input.parse::<syn::token::Else>()?,
            body: input.parse()?,
        })
    }
}

impl ToTokens for HtmlElse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlElse { body, else_token } = self;
        let spanned = quote!(#else_token #body);
        tokens.extend(quote_spanned! { spanned.span() =>
            else {
                #body
            }
        });
    }
}

struct HtmlIfBody(HtmlRoot);

impl Parse for HtmlIfBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body_input;
        syn::braced!(body_input in input);
        Ok(HtmlIfBody(body_input.parse()?))
    }
}

impl ToTokens for HtmlIfBody {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlIfBody(body) = self;

        tokens.extend(quote! {
            if let ::wal::virtual_dom::VNode::List(_) = #body {
                #body
            } else {
                ::wal::virtual_dom::VNode::List (
                    ::wal::virtual_dom::VList::new(vec![#body], None)
                )
            }
        });
    }
}
