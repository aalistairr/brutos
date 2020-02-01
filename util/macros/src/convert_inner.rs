use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn convert_inner(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let inner_ty = match &data {
        Data::Struct(strukt) => match &strukt.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => &f.unnamed[0].ty,
            _ => panic!("the type must be a tuple struct with one field"),
        },
        _ => panic!("the type must be a tuple struct with one field"),
    };

    let expanded = quote! {
        impl #impl_generics From<#inner_ty> for #ident #ty_generics #where_clause {
            fn from(value: #inner_ty) -> #ident #ty_generics {
                #ident(value)
            }
        }

        impl #impl_generics Into<#inner_ty> for #ident #ty_generics #where_clause {
            fn into(self) -> #inner_ty {
                self.0
            }
        }
    };
    TokenStream::from(expanded)
}
