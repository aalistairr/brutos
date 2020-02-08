use proc_macro::TokenStream;

mod bitenum;
mod bitfield;
mod convert_inner;
mod selector;

#[proc_macro_derive(BitEnum)]
pub fn bitenum(input: TokenStream) -> TokenStream {
    bitenum::bitenum(input)
}

#[proc_macro]
pub fn bitfield(input: TokenStream) -> TokenStream {
    bitfield::bitfield(input)
}

#[proc_macro_derive(BitfieldNew)]
pub fn bitfield_new(input: TokenStream) -> TokenStream {
    bitfield::bitfield_new(input)
}

#[proc_macro_derive(ConvertInner)]
pub fn convert_inner(input: TokenStream) -> TokenStream {
    convert_inner::convert_inner(input)
}

#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    selector::selector(input)
}
