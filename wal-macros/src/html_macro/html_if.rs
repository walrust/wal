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
        let html_if = &self.html_if;
        let else_ifs = &self.else_ifs;
        let else_tokens = match &self.html_else {
            Some(html_else) => html_else.into_token_stream(),
            None => {
                let default_else_body = &HtmlRoot::Empty;
                quote! { else { #default_else_body } }
            }
        };

        tokens.extend(quote_spanned! {self.html_if.if_token.span() =>
            #html_if
            #(#else_ifs)*
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

impl ToTokens for HtmlIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let condition = &self.condition;
        let body = &self.body;
        tokens.extend(quote_spanned! {self.if_token.span() =>
            if #condition {
                #body
            }
        });
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

impl ToTokens for HtmlElseIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let condition = &self.condition;
        let body = &self.body;

        tokens.extend(quote_spanned! {self.if_token.span() =>
            else if #condition {
                #body
            }
        });
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

impl ToTokens for HtmlElse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let body = &self.body;
        tokens.extend(quote_spanned! { self.else_token.span() => else { #body } });
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
