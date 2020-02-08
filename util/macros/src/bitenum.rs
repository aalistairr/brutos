use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta, Path};

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
        impl #impl_generics brutos_util::ConvertRepr for #ident #ty_generics #where_clause {
            type Repr = #repr;
        }

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
