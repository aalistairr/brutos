use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    parse_macro_input, GenericArgument, Generics, Ident, PathArguments, Token, Type, Visibility,
};

struct Selector {
    vis: Visibility,
    name: Ident,
    generics: Generics,
    pointer: Type,
    field: Ident,
}

impl Parse for Selector {
    fn parse(input: ParseStream) -> Result<Self> {
        let vis = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;
        input.parse::<Token![:]>()?;
        let pointer = input.parse()?;
        input.parse::<Token![=>]>()?;
        let field = input.parse()?;
        Ok(Selector {
            vis,
            name,
            generics,
            pointer,
            field,
        })
    }
}

/// Generates a type that implements `Sel` for use with [`LinkedList`](brutos_util::linked_list::LinkedList).
///
/// # Examples
/// ```
/// use brutos_util::linked_list::Node;
///
/// pub struct Foo {
///     node: Node<FooSel>,
/// }
/// brutos_util_macros::selector!(pub FooSel: Box<Foo> => node);
/// ```
///
/// References can also be used:
/// ```
/// use brutos_util::linked_list::Node;
///
/// struct Bar<'a, T> {
///     node: Node<BarSel<'a>>,
///     data: T,
/// }
/// brutos_util_macros::selector!(BarSel<'a, T: 'a + Default>: &'a Bar<'a, T> => node);
/// ```
#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    let Selector {
        vis,
        name,
        generics,
        pointer,
        field,
    } = parse_macro_input!(input as Selector);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let t = match &pointer {
        Type::Reference(x) => match &*x.elem {
            Type::Path(x) => &x.path,
            _ => panic!("Invalid type"),
        },
        Type::Path(x) => match &x.path.segments.last().expect("Invalid type").arguments {
            PathArguments::AngleBracketed(x) => match x
                .args
                .iter()
                .filter(|arg| {
                    if let GenericArgument::Type(_) = arg {
                        true
                    } else {
                        false
                    }
                })
                .next()
                .expect("Invalid type")
            {
                GenericArgument::Type(Type::Path(x)) => &x.path,
                _ => panic!("Invalid type"),
            },
            _ => panic!("Invalid type"),
        },
        _ => panic!("Invalid type"),
    };
    let mut t_path = t.clone();
    t_path.segments.last_mut().expect("Invalid type").arguments = PathArguments::None;

    let expanded = quote! {
        #vis struct #name #impl_generics {
            _marker: core::marker::PhantomData<#pointer>,
        }

        unsafe impl #impl_generics brutos_util::linked_list::Sel for #name #ty_generics #where_clause {
            type Ptr = #pointer;
            type Immovable = <#pointer as brutos_util::pointer::Pointer>::Immovable;
            type Raw = <#pointer as brutos_util::pointer::Pointer>::Raw;
            type Target = <#pointer as core::ops::Deref>::Target;

            fn node_offset() -> usize {
                #[allow(clippy::unneeded_field_pattern, unused_variables)]
                let #t_path { #field, .. }: #t;
                // Create an instance of the container and calculate the offset to its field.
                // Here we're using an uninitialized instance of $parent. We avoid UB
                // by only using raw pointers that point to real (allocated, albeit uninitialized) memory.
                let val = core::mem::MaybeUninit::<#t>::uninit();
                let base_ptr = val.as_ptr();
                #[allow(unused_unsafe)] // for when the macro is used in an unsafe block
                let field_ptr = unsafe { &(*base_ptr).#field as *const brutos_util::linked_list::Node<Self> };
                (field_ptr as usize) - (base_ptr as usize)
            }
        }
    };
    TokenStream::from(expanded)
}
