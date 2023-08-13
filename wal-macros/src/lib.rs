use proc_macro::TokenStream;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    input
}
