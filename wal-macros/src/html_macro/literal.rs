use syn::parse::{Parse, ParseStream};

pub struct Literal(syn::Lit);

impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

        Ok(Self(lit))
    }
}
