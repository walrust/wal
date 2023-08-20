use syn::parse::{Parse, ParseStream};

pub struct HtmlLiteral(syn::Lit);

impl Parse for HtmlLiteral {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit = input.parse()?;

        let error_message = match lit {
            syn::Lit::ByteStr(_) => Some("Byte string literals are not supported"),
            syn::Lit::Byte(_) => Some("Byte literals are not supported"),
            syn::Lit::Verbatim(_) => Some("Unsupported literal format encountered"),
            _ => None,
        };

        if let Some(msg) = error_message {
            return Err(syn::Error::new(lit.span(), msg));
        };

        Ok(Self(lit))
    }
}
