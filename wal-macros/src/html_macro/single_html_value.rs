use syn::parse::{Parse, ParseStream};

pub enum SingleHtmlValue {
    Lit(syn::Lit),
    Expr(syn::Expr),
}

impl Parse for SingleHtmlValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Lit) {
            let lit = input.parse()?;

            let error_message = match lit {
                syn::Lit::ByteStr(_) => Some("Byte string literals are not supported"),
                syn::Lit::Byte(_) => Some("Byte literals are not supported"),
                syn::Lit::Verbatim(_) => Some("Unsupported literal format encountered"),
                _ => None,
            };

            if let Some(msg) = error_message {
                return Err(syn::Error::new_spanned(lit, msg));
            };

            Ok(Self::Lit(lit))
        } else {
            Ok(Self::Expr(input.parse()?))
        }
    }
}
