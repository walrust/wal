use super::html_root::HtmlRoot;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

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
        let condition = &self.html_if.condition;
        let body = &self.html_if.body;
        let if_tokens = quote_spanned! {self.html_if.if_token.span() =>
            if #condition {
                #body
            }
        };

        let else_ifs_tokens: Vec<proc_macro2::TokenStream> = self
            .else_ifs
            .iter()
            .map(|else_if| {
                let condition = &else_if.condition;
                let body = &else_if.body;

                quote_spanned! {else_if.if_token.span() =>
                    else if #condition {
                        #body
                    }
                }
            })
            .collect();

        let else_tokens = match &self.html_else {
            Some(html_else) => {
                let body = &html_else.body;
                quote_spanned! { html_else.else_token.span() => else { #body } }
            }
            None => {
                let default_else_body = &HtmlRoot::Empty;
                quote! { else { #default_else_body } }
            }
        };

        tokens.extend(quote_spanned! {self.html_if.if_token.span() =>
            #if_tokens
            #(#else_ifs_tokens)*
            #else_tokens
        });
    }
}

struct HtmlIf {
    if_token: syn::token::If,
    condition: syn::Expr,
    body: HtmlRoot,
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let if_token = input.parse::<syn::token::If>()?;
        let condition = parse_condition(input)?;
        let body = parse_body(input)?;

        Ok(HtmlIf {
            if_token,
            condition,
            body,
        })
    }
}

struct HtmlElseIf {
    if_token: syn::token::If,
    condition: syn::Expr,
    body: HtmlRoot,
}

impl Parse for HtmlElseIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::token::Else>()?;
        let if_token = input.parse::<syn::token::If>()?;
        let condition = parse_condition(input)?;
        let body = parse_body(input)?;

        Ok(HtmlElseIf {
            if_token,
            condition,
            body,
        })
    }
}

impl HtmlElseIf {
    fn peek(input: ParseStream) -> bool {
        input.peek(syn::token::Else) && input.peek2(syn::token::If)
    }
}

struct HtmlElse {
    else_token: syn::token::Else,
    body: HtmlRoot,
}

impl Parse for HtmlElse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let else_token = input.parse::<syn::token::Else>()?;
        let body = parse_body(input)?;

        Ok(HtmlElse { else_token, body })
    }
}

fn parse_condition(input: ParseStream) -> syn::Result<syn::Expr> {
    let condition = syn::Expr::parse_without_eager_brace(input)?;

    match &condition {
        syn::Expr::Block(syn::ExprBlock { block, .. }) if block.stmts.is_empty() => {
            Err(syn::Error::new_spanned(
                &condition,
                "Expected condition for `if` expression, found empty block",
            ))
        }
        _ => Ok(condition),
    }
}

fn parse_body(input: ParseStream) -> syn::Result<HtmlRoot> {
    let body;
    syn::braced!(body in input);
    Ok(body.parse()?)
}
