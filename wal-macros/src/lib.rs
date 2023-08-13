use proc_macro::TokenStream;

mod html_macro;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    input
}
