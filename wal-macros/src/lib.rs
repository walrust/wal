use html_macro::html_root::HtmlRoot;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod html_macro;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRoot);
    TokenStream::from(root.into_token_stream())
}
