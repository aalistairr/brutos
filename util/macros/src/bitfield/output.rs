use std::ops::Range;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_quote;
use syn::{Expr, Fields, Ident, ItemStruct, Path, Type, Visibility};

use super::{is_uint, self_repr_ty, value_repr_ty};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bitfield {
    pub strukt: ItemStruct,
    pub fields: Vec<Field>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub value_ty: Type,
    pub rels: Vec<Relationship>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Attribute {
    ReadOnly,
    PrivateWrite,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Relationship {
    pub from: Range<Expr>,
    pub to_src: Option<Expr>,
    pub to: Range<Expr>,
}

impl ToTokens for Bitfield {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Bitfield { strukt, fields } = self;

        let strukt_name = &strukt.ident;
        let (impl_generics, ty_generics, where_clause) = strukt.generics.split_for_impl();

        fn mask_f(name: &Ident, repr_ty: &Type) -> TokenStream {
            quote! {
                const fn #name(range: core::ops::Range<u32>) -> #repr_ty {
                    if (range.end - range.start) as usize == core::mem::size_of::<#repr_ty>() * 8 {
                        !0
                    } else {
                        ((1 << (range.end - range.start)) - 1) << range.start
                    }
                }
            }
        }

        fn field_rels<'a>(field: &'a Field) -> Vec<Ident> {
            field
                .rels
                .iter()
                .enumerate()
                .map(|(i, _)| format_ident!("rel_{}", i))
                .collect()
        }

        let mut fields_tokens = quote! {};
        for field in fields {
            let value_ty = &field.value_ty;
            let value_repr_ty = value_repr_ty(value_ty);

            let value_repr_create_mask =
                mask_f(&format_ident!("value_repr_create_mask"), &value_repr_ty);

            let rels = field_rels(field);

            let mut rels_tokens = quote! {};
            for (rel, rel_name) in field.rels.iter().zip(&rels) {
                let (rel_to_start, rel_to_end) = (&rel.to.start, &rel.to.end);
                let (rel_from_start, rel_from_end) = (&rel.from.start, &rel.from.end);
                let self_repr_ty = self_repr_ty(strukt, &rel.to_src);
                let self_bits = quote! { #rel_to_start..#rel_to_end };
                let value_bits = quote! { #rel_from_start..#rel_from_end };

                let self_repr_create_mask =
                    mask_f(&format_ident!("self_repr_create_mask"), &self_repr_ty);

                let self_repr = match &strukt.fields {
                    Fields::Unnamed(_) => match &rel.to_src {
                        Some(i) => quote! { this.0[#i] },
                        None => quote! { this.0 },
                    },
                    Fields::Named(_) => match &rel.to_src {
                        Some(p) => quote! { this.#p },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };

                let assert_inbounds = match &strukt.fields {
                    Fields::Unnamed(fields) => match &fields.unnamed[0].ty {
                        Type::Array(t) => match &rel.to_src {
                            Some(i) => {
                                let len = &t.len;
                                Some(quote! {
                                    pub const _ASSERT_INBOUNDS: () = assert!(#i < #len);
                                })
                            }
                            _ => unreachable!(),
                        },
                        _ => None,
                    },
                    _ => None,
                };

                rels_tokens.extend(quote! {
                    #[allow(non_snake_case)]
                    pub mod #rel_name {
                        use super::*;

                        pub type SelfRepr = #self_repr_ty;

                        #self_repr_create_mask

                        pub const SELF_BITS: core::ops::Range<u32> = #self_bits;
                        pub const SELF_BITS_LEN: usize = (SELF_BITS.end - SELF_BITS.start) as usize;
                        pub const SELF_MASK: SelfRepr = self_repr_create_mask(SELF_BITS);
                        pub const VALUE_BITS: core::ops::Range<u32> = #value_bits;
                        pub const VALUE_BITS_LEN: usize = (VALUE_BITS.end - VALUE_BITS.start) as usize;
                        pub const VALUE_MASK: ValueRepr = value_repr_create_mask(VALUE_BITS);

                        pub const _ASSERT0: () = assert!(SELF_BITS.start <= SELF_BITS.end);
                        pub const _ASSERT1: () = assert!(VALUE_BITS.start <= VALUE_BITS.end);
                        pub const _ASSERT2: () = assert!(SELF_BITS_LEN == VALUE_BITS_LEN);
                        pub const _ASSERT3: () = assert!(SELF_BITS.end as usize <= core::mem::size_of::<SelfRepr>() * 8);
                        pub const _ASSERT4: () = assert!(VALUE_BITS.end as usize <= core::mem::size_of::<ValueRepr>() * 8);
                        #assert_inbounds

                        pub const fn self_repr #impl_generics(this: &#strukt_name #ty_generics) -> SelfRepr #where_clause {
                            #self_repr
                        }

                        pub const fn self_repr_mut #impl_generics(this: &mut #strukt_name #ty_generics) -> &mut SelfRepr #where_clause {
                            &mut #self_repr
                        }
                    }
                });
            }

            let value_into_from_repr = match value_ty {
                Type::Path(p) if is_uint(&p.path) => quote! {
                    pub const fn value_into_repr(value: #value_ty) -> #value_repr_ty {
                        value
                    }
                    pub const fn value_from_repr(value_repr: #value_repr_ty) -> #value_ty {
                        value_repr
                    }
                },
                Type::Path(p) if p.path.is_ident("bool") => quote! {
                    pub const fn value_into_repr(value: #value_ty) -> #value_repr_ty {
                        value as #value_repr_ty
                    }
                    pub const fn value_from_repr(value_repr: #value_repr_ty) -> #value_ty {
                        match value_repr {
                            0 => false,
                            1 => true,
                            _ => panic!("invalid representation for value"),
                        }
                    }
                },
                value_ty => quote! {
                    pub const fn value_into_repr(value: #value_ty) -> #value_repr_ty {
                        <#value_ty>::into_repr(value)
                    }
                    pub const fn value_from_repr(value_repr: #value_repr_ty) -> #value_ty {
                        match <#value_ty>::from_repr(value_repr) {
                            Some(value) => value,
                            None => panic!("invalid representation for value"),
                        }
                    }
                },
            };

            let rels = field_rels(field);
            let field_name = &field.name;
            fields_tokens.extend(quote! {
                #[allow(non_snake_case)]
                pub mod #field_name {
                    use super::*;

                    pub type Value = #value_ty;
                    pub type ValueRepr = #value_repr_ty;

                    #value_into_from_repr

                    #rels_tokens

                    #value_repr_create_mask
                    pub const VALUE_REPR_MASK: #value_repr_ty = {
                        let mut mask = 0;
                        #(mask |= #rels::VALUE_MASK;)*
                        mask
                    };
                }
            });
        }

        let mod_name = format_ident!("__bitfield_{}", strukt.ident);
        tokens.extend(quote! {
            #strukt

            #[allow(non_snake_case)]
            mod #mod_name {
                use super::*;
                #fields_tokens
            }
        });

        for field in fields {
            let value_ty = &field.value_ty;
            let rels = field_rels(field);

            let mut ro = false;
            let mut private_write = false;
            for attr in &field.attrs {
                match attr {
                    Attribute::ReadOnly => ro = true,
                    Attribute::PrivateWrite => private_write = true,
                }
            }
            let emit_setter = !ro || private_write;

            let field_name = &field.name;
            let field_mod: Path = parse_quote! { #mod_name::#field_name };

            let get_name = match value_ty {
                Type::Path(p) if p.path.is_ident("bool") => format_ident!("is_{}", field.name),
                _ => field.name.clone(),
            };
            let get = Get {
                vis: &field.vis,
                get_name,
                field_mod: &field_mod,
                rel: &*rels,
            };
            let set = match emit_setter {
                false => None,
                true => Some(Set {
                    vis: match private_write {
                        false => &field.vis,
                        true => &Visibility::Inherited,
                    },
                    set_name: format_ident!("set_{}", field.name),
                    with_name: format_ident!("with_{}", field.name),
                    field_mod: &field_mod,
                    rel: &*rels,
                }),
            };

            tokens.extend(quote! {
                impl #impl_generics #strukt_name #ty_generics #where_clause {
                    #get
                    #set
                }
            });
        }
    }
}

pub struct Get<'a> {
    vis: &'a Visibility,
    get_name: Ident,
    field_mod: &'a Path,
    rel: &'a [Ident],
}

impl<'a> ToTokens for Get<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Get {
            vis,
            get_name,
            field_mod,
            rel,
        } = self;
        tokens.extend(quote! {
            #vis const fn #get_name(&self) -> #field_mod::Value {
                let mut value_repr: #field_mod::ValueRepr = 0;
                #(value_repr |= (((#field_mod::#rel::self_repr(self) & #field_mod::#rel::SELF_MASK) >> #field_mod::#rel::SELF_BITS.start) as #field_mod::ValueRepr) << #field_mod::#rel::VALUE_BITS.start;)*
                #field_mod::value_from_repr(value_repr)
            }
        });
    }
}

pub struct Set<'a> {
    vis: &'a Visibility,
    set_name: Ident,
    with_name: Ident,
    field_mod: &'a Path,
    rel: &'a [Ident],
}

impl<'a> ToTokens for Set<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Set {
            vis,
            set_name,
            with_name,
            field_mod,
            rel,
        } = self;
        tokens.extend(quote! {
            #vis const fn #set_name(&mut self, value: #field_mod::Value) {
                let value_repr: #field_mod::ValueRepr = #field_mod::value_into_repr(value);
                assert!(value_repr & !#field_mod::VALUE_REPR_MASK == 0);
                #(*#field_mod::#rel::self_repr_mut(self) =
                    (#field_mod::#rel::self_repr(self) & !#field_mod::#rel::SELF_MASK)
                  | ((((value_repr & #field_mod::#rel::VALUE_MASK) >> #field_mod::#rel::VALUE_BITS.start) as #field_mod::#rel::SelfRepr) << #field_mod::#rel::SELF_BITS.start);
                )*
            }

            #vis const fn #with_name(mut self, value: #field_mod::Value) -> Self {
                self.#set_name(value);
                self
            }
        });
    }
}
