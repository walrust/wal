use rsx_macro::root::Root;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod rsx_macro;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as Root);
    TokenStream::from(root.into_token_stream())
}
