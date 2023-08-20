use super::html_root::HtmlRoot;
use syn::parse::{Parse, ParseStream};

pub struct HtmlIf {
    _condition: syn::Expr,
    _body: HtmlRoot,
    _else_ifs: Vec<HtmlElseIf>,
    _else_body: Option<HtmlRoot>,
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::token::If>()?;
        let condition = parse_condition(input)?;
        let body = parse_body(input)?;

        let mut else_ifs = Vec::new();
        while HtmlElseIf::peek(input) {
            else_ifs.push(input.parse()?);
        }

        let else_body = if input.peek(syn::token::Else) {
            input.parse::<syn::token::Else>()?;
            Some(parse_body(input)?)
        } else {
            None
        };

        Ok(HtmlIf {
            _condition: condition,
            _body: body,
            _else_ifs: else_ifs,
            _else_body: else_body,
        })
    }
}

struct HtmlElseIf {
    _condition: syn::Expr,
    _body: HtmlRoot,
}

impl Parse for HtmlElseIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::token::Else>()?;
        input.parse::<syn::token::If>()?;
        let condition = parse_condition(input)?;
        let body = parse_body(input)?;

        Ok(HtmlElseIf {
            _condition: condition,
            _body: body,
        })
    }
}

impl HtmlElseIf {
    fn peek(input: ParseStream) -> bool {
        input.peek(syn::token::Else) && input.peek2(syn::token::If)
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