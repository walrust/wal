use html_macro::HtmlTree;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod html_macro;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let _tree = parse_macro_input!(input as HtmlTree);
    TokenStream::new()
}
