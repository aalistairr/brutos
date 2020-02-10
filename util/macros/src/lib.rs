use proc_macro::TokenStream;

mod convert_inner;
mod selector;

#[proc_macro_derive(ConvertInner)]
pub fn convert_inner(input: TokenStream) -> TokenStream {
    convert_inner::convert_inner(input)
}

#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    selector::selector(input)
}
