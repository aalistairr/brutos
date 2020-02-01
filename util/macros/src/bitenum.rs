use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Ident, Meta, Path, Token, Type, Visibility,
};

pub fn bitenum(input: TokenStream) -> TokenStream {
    let DeriveInput {
        vis,
        ident,
        generics,
        data,
        attrs,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let enumm = match data {
        Data::Enum(x) => x,
        _ => panic!("expected an enum"),
    };

    let (mut variant_name, mut variant_discriminant) = (Vec::new(), Vec::new());
    for variant in enumm.variants {
        assert!(
            variant.fields == Fields::Unit,
            "variants cannot have fields"
        );
        let (_, discriminant) = variant
            .discriminant
            .expect("each variant must have a discriminant");
        variant_name.push(variant.ident);
        variant_discriminant.push(discriminant);
    }

    let repr = attrs
        .iter()
        .filter(|attr| {
            attr.path.segments.len() == 1 && attr.path.get_ident() == Some(&format_ident!("repr"))
        })
        .next()
        .map(
            |attr| match attr.parse_meta().expect("failed to parse repr attribute") {
                Meta::Path(p) => p,
                _ => panic!("failed to parse repr attribute"),
            },
        )
        .unwrap_or_else(|| {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push(syn::PathSegment {
                ident: format_ident!("usize"),
                arguments: syn::PathArguments::None,
            });
            Path {
                leading_colon: None,
                segments,
            }
        });

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #vis const fn from_repr(value: #repr) -> Option<Self> {
                match value {
                    #(#variant_discriminant => Some(#ident::#variant_name),)*
                    _ => None,
                }
            }

            #vis const fn into_repr(self) -> #repr {
                self as #repr
            }
        }
    };
    TokenStream::from(expanded)
}

struct BitEnumField {
    vis: Visibility,
    bitfield_ty: Type,
    field_name: Ident,
    bitenum_ty: Type,
}

impl Parse for BitEnumField {
    fn parse(input: ParseStream) -> Result<BitEnumField> {
        let vis = input.parse()?;
        let bitfield_ty = input.parse()?;
        input.parse::<Token![.]>()?;
        let field_name = input.parse()?;
        input.parse::<Token![:]>()?;
        let bitenum_ty = input.parse()?;
        Ok(BitEnumField {
            vis,
            bitfield_ty,
            field_name,
            bitenum_ty,
        })
    }
}

pub fn bitenum_field(input: TokenStream) -> TokenStream {
    let BitEnumField {
        vis,
        bitfield_ty,
        field_name,
        bitenum_ty,
    } = parse_macro_input!(input as BitEnumField);
    let raw_getter_name = format_ident!("{}_raw", field_name);
    let raw_setter_name = format_ident!("set_{}_raw", field_name);
    let getter_name = field_name.clone();
    let setter_name = format_ident!("set_{}", field_name);
    let with_name = format_ident!("with_{}", field_name);

    let expanded = quote! {
        impl #bitfield_ty {
            #vis const fn #getter_name(&self) -> #bitenum_ty {
                match #bitenum_ty::from_repr(self.#raw_getter_name()) {
                    Some(x) => x,
                    _ => panic!(concat!("invalid repr for `", stringify!(#field_name), "`")),
                }
            }

            #vis const fn #setter_name(&mut self, value: #bitenum_ty) {
                self.#raw_setter_name(value.into_repr());
            }

            #vis const fn #with_name(mut self, value: #bitenum_ty) -> Self {
                self.#setter_name(value);
                self
            }
        }
    };
    TokenStream::from(expanded)
}
