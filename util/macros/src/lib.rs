use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    braced, parse_macro_input, BinOp, Data, DeriveInput, Error, Expr, ExprBinary, ExprLit,
    ExprParen, ExprRepeat, Fields, GenericArgument, Generics, Ident, ItemStruct, Lit, LitInt,
    PathArguments, Token, Type, Visibility,
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

struct BitfieldMacroInput {
    strukt: ItemStruct,
    fields: Vec<Field>,
}

impl Parse for BitfieldMacroInput {
    fn parse(input: ParseStream) -> Result<BitfieldMacroInput> {
        let strukt = input.parse()?;
        let mut fields = Vec::new();
        while !input.is_empty() {
            fields.push(input.parse()?);
        }
        Ok(BitfieldMacroInput { strukt, fields })
    }
}

struct Field {
    vis: Visibility,
    name: Ident,
    ty: Type,
    value: Value,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Field> {
        let vis = input.parse()?;
        let field_token = input.parse::<Ident>()?;
        if field_token != "field" {
            return Err(Error::new(field_token.span(), "expected `field`"));
        }

        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse()?;

        let value = if input.parse::<Token![=>]>().is_ok() {
            let mut concatenation = Vec::new();
            loop {
                concatenation.push(input.parse()?);
                if input.parse::<Token![~]>().is_ok() {
                    continue;
                } else if input.parse::<Token![;]>().is_ok() {
                    break;
                } else {
                    return Err(Error::new(input.span(), "expected `~` or `;`"));
                }
            }
            Value::Concatenated(concatenation)
        } else {
            let content;
            braced!(content in input);
            let mut map = Vec::new();
            while !content.is_empty() {
                let dst = content.parse()?;
                content.parse::<Token![=>]>()?;
                let src = content.parse()?;
                map.push((dst, src));

                if content.parse::<Token![,]>().is_ok() || content.is_empty() {
                    continue;
                } else {
                    return Err(Error::new(input.span(), "expected `,` or `}`"));
                }
            }
            Value::Mapped(map)
        };

        Ok(Field {
            vis,
            name,
            ty,
            value,
        })
    }
}

enum Value {
    Concatenated(Vec<Bits>),
    Mapped(Vec<(Bits, Bits)>),
}

#[derive(Clone)]
struct Bits {
    array_index: Option<Expr>,
    bits_index: BitsIndex,
}

impl Parse for Bits {
    fn parse(input: ParseStream) -> Result<Bits> {
        fn extract_index(x: Expr) -> Result<BitsIndex> {
            match x {
                Expr::Range(x) => Ok(BitsIndex::Range(
                    *x.from
                        .ok_or(Error::new(Span::call_site(), "invalid range type"))?,
                    *x.to
                        .ok_or(Error::new(Span::call_site(), "invalid range type"))?,
                )),
                x => Ok(BitsIndex::One(x)),
            }
        }

        match input.parse::<Expr>()? {
            Expr::Index(x) => Ok(Bits {
                array_index: Some(*x.expr),
                bits_index: extract_index(*x.index)?,
            }),
            x => Ok(Bits {
                array_index: None,
                bits_index: extract_index(x)?,
            }),
        }
    }
}

#[derive(Clone)]
enum BitsIndex {
    One(Expr),
    Range(Expr, Expr),
}

struct ArrayIndex(Option<Expr>);

struct BoolField(Field, ItemStruct);
struct UintField(Field, ItemStruct);

struct AssertInBounds(Ident, Expr, Expr);
struct ValueIsZeroFn(Type);

impl ToTokens for ArrayIndex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(ref x) = self.0 {
            tokens.extend(quote! { [#x] });
        }
    }
}

impl ToTokens for AssertInBounds {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let AssertInBounds(name, i, len) = self;
        tokens.extend(quote! { const #name: () = assert!(#i < #len); });
    }
}

impl ToTokens for ValueIsZeroFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let t = &self.0;
        match t {
            Type::Array(ta) => {
                let len = &ta.len;
                tokens.extend(quote! {
                    const fn value_is_zero(value: #t) -> bool {
                        let mut i = 0;
                        while i < #len {
                            if value[i] != 0 {
                                return false;
                            }
                        }
                        true
                    }
                });
            }
            t => tokens.extend(quote! {
                const fn value_is_zero(value: #t) -> bool {
                    value == 0
                }
            }),
        }
    }
}

impl ToTokens for BoolField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let vis = &self.0.vis;
        let getter_name = format_ident!("is_{}", self.0.name);
        let setter_name = format_ident!("set_{}", self.0.name);
        let with_name = format_ident!("with_{}", self.0.name);

        let (array_index, bit_index) = match &self.0.value {
            Value::Concatenated(c) if c.len() == 1 => (
                ArrayIndex(c[0].array_index.clone()),
                match &c[0].bits_index {
                    BitsIndex::One(bit_index) => bit_index.clone(),
                    _ => unreachable!(),
                },
            ),
            _ => unreachable!(),
        };

        let (self_nt, self_assert_inbounds) = match &self.1.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => match &f.unnamed[0].ty {
                Type::Array(t) => (
                    (*t.elem).clone(),
                    array_index.0.as_ref().map(|i| {
                        AssertInBounds(format_ident!("_ASSERT_INBOUNDS"), i.clone(), t.len.clone())
                    }),
                ),
                t => (t.clone(), None),
            },
            _ => panic!("the bitfield must be a tuple struct with one field"),
        };

        tokens.extend(quote! {
            #vis const fn #getter_name(&self) -> bool {
                const _ASSERT: () = assert!(#bit_index < core::mem::size_of::<#self_nt>() * 8);
                #self_assert_inbounds
                (self.0 #array_index >> #bit_index) & 1 == 1
            }

            #vis const fn #setter_name(&mut self, value: bool) {
                self.0 #array_index = (self.0 #array_index & !(1 << #bit_index)) | (if value { 1 } else { 0 } << #bit_index);
            }

            #vis const fn #with_name(mut self, value: bool) -> Self {
                self.#setter_name(value);
                self
            }
        });
    }
}

impl ToTokens for UintField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let vis = &self.0.vis;
        let getter_name = format_ident!("{}", self.0.name);
        let setter_name = format_ident!("set_{}", self.0.name);
        let with_name = format_ident!("with_{}", self.0.name);

        let ty = &self.0.ty;

        let zero = Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Int(LitInt::new("0", Span::call_site())),
        });
        let value_zero = match ty {
            Type::Array(t) => Expr::Repeat(ExprRepeat {
                attrs: Vec::new(),
                bracket_token: syn::token::Bracket {
                    span: Span::call_site(),
                },
                expr: Box::new(zero),
                semi_token: Token![;](Span::call_site()),
                len: Box::new(t.len.clone()),
            }),
            _ => zero,
        };

        let (mut value_array_index, mut value_bits_start, mut value_bits_end) =
            (Vec::new(), Vec::new(), Vec::new());
        let (mut self_array_index, mut self_bits_start, mut self_bits_end) =
            (Vec::new(), Vec::new(), Vec::new());
        let (mut value_assert_inbounds, mut self_assert_inbounds) = (Vec::new(), Vec::new());
        let map = match &self.0.value {
            Value::Mapped(map) => map,
            _ => unreachable!("value should be mapped"),
        };
        let (self_nt, self_t_len) = match &self.1.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => match &f.unnamed[0].ty {
                Type::Array(t) => ((*t.elem).clone(), Some(t.len.clone())),
                t => (t.clone(), None),
            },
            _ => panic!("the bitfield must be a tuple struct with one field"),
        };
        let (value_nt, value_t_len) = match ty {
            Type::Array(t) => ((*t.elem).clone(), Some(t.len.clone())),
            t => (t.clone(), None),
        };
        for (value, selff) in map {
            value_array_index.push(ArrayIndex(value.array_index.clone()));
            self_array_index.push(ArrayIndex(selff.array_index.clone()));
            match &value.bits_index {
                BitsIndex::Range(start, end) => {
                    value_bits_start.push(start.clone());
                    value_bits_end.push(end.clone());
                }
                _ => unreachable!("value bits should be a range"),
            }
            match &selff.bits_index {
                BitsIndex::Range(start, end) => {
                    self_bits_start.push(start.clone());
                    self_bits_end.push(end.clone());
                }
                _ => unreachable!("self bits should be a range"),
            }

            value_assert_inbounds.push(value.array_index.as_ref().map(|i| {
                AssertInBounds(
                    format_ident!("_ASSERT_VALUE_INBOUNDS"),
                    i.clone(),
                    value_t_len.as_ref().unwrap().clone(),
                )
            }));
            self_assert_inbounds.push(selff.array_index.as_ref().map(|i| {
                AssertInBounds(
                    format_ident!("_ASSERT_SELF_INBOUNDS"),
                    i.clone(),
                    self_t_len.as_ref().unwrap().clone(),
                )
            }));
        }

        let value_is_zero_fn = ValueIsZeroFn(ty.clone());

        tokens.extend(quote! {
            #vis const fn #getter_name(&self) -> #ty {
                #({
                    const _ASSERT0: () = assert!(#self_bits_start < #self_bits_end);
                    const _ASSERT1: () = assert!(#value_bits_start < #value_bits_end);
                    const _ASSERT2: () = assert!(#self_bits_end - #self_bits_start == #value_bits_end - #value_bits_start);
                    #self_assert_inbounds
                    #value_assert_inbounds
                })*
                let mut x = #value_zero;
                #(x #value_array_index |=
                    ((((self.0 #self_array_index & Self::__self_mask(#self_bits_start..#self_bits_end)) >> #self_bits_start) as #value_nt)
                    << #value_bits_start);
                )*
                x
            }

            #vis const fn #setter_name(&mut self, value: #ty) {
                const fn __value_mask(r: core::ops::Range<u32>) -> #value_nt {
                    const N_BITS: u32 = (core::mem::size_of::<#value_nt>() * 8) as u32;
                    assert!(r.end <= N_BITS && r.start < N_BITS && r.start <= r.end);
                    if r.end - r.start == N_BITS {
                        !0
                    } else {
                        ((1 << (r.end - r.start)) - 1) << r.start
                    }
                }

                {
                    #value_is_zero_fn
                    let mut tmp = value;
                    #(tmp #value_array_index &= !__value_mask(#value_bits_start..#value_bits_end);)*
                    assert!(value_is_zero(tmp));
                }

                #(self.0 #self_array_index =
                    (self.0 #self_array_index & !Self::__self_mask(#self_bits_start..#self_bits_end))
                        | ((((value #value_array_index & __value_mask(#value_bits_start..#value_bits_end))
                            >> #value_bits_start) as #self_nt) << #self_bits_start);
                )*
            }

            #vis const fn #with_name(mut self, value: #ty) -> Self {
                self.#setter_name(value);
                self
            }
        });
    }
}

fn parens(x: Expr) -> Expr {
    Expr::Paren(ExprParen {
        attrs: Vec::new(),
        paren_token: syn::token::Paren(Span::call_site()),
        expr: Box::new(x),
    })
}

#[proc_macro]
pub fn bitfield(input: TokenStream) -> TokenStream {
    let BitfieldMacroInput { strukt, fields } = parse_macro_input!(input as BitfieldMacroInput);
    let strukt_name = &strukt.ident;
    let (impl_generics, ty_generics, where_clause) = strukt.generics.split_for_impl();

    let (mut bools, mut uints, mut arrays) = (Vec::new(), Vec::new(), Vec::new());
    for mut field in fields {
        match field.ty {
            Type::Array(_) => match field.value {
                Value::Mapped(_) => arrays.push(UintField(field, strukt.clone())),
                _ => panic!("array fields must be mapped"),
            },
            Type::Path(ref t)
                if t.path.segments.len() == 1 && t.path.segments[0].ident == "bool" =>
            {
                match &field.value {
                    Value::Concatenated(concatenation) if concatenation.len() == 1 => {
                        match concatenation[0].bits_index {
                            BitsIndex::One(_) => bools.push(BoolField(field, strukt.clone())),
                            _ => panic!("bool fields must point to one bit"),
                        }
                    }
                    _ => panic!("bool fields must point to one bit"),
                }
            }
            _ => {
                match field.value {
                    Value::Mapped(ref mut map) => {
                        fn make_one_into_range(bits_index: &mut BitsIndex) {
                            match bits_index {
                                BitsIndex::One(bit) => {
                                    *bits_index = BitsIndex::Range(
                                        bit.clone(),
                                        parens(Expr::Binary(ExprBinary {
                                            attrs: Vec::new(),
                                            left: Box::new(bit.clone()),
                                            op: BinOp::Add(Token![+](Span::call_site())),
                                            right: Box::new(Expr::Lit(ExprLit {
                                                attrs: Vec::new(),
                                                lit: Lit::Int(LitInt::new("1", Span::call_site())),
                                            })),
                                        })),
                                    )
                                }
                                BitsIndex::Range(_, _) => (),
                            }
                        }
                        for (value, selff) in map {
                            make_one_into_range(&mut value.bits_index);
                            make_one_into_range(&mut selff.bits_index);
                        }
                    }
                    Value::Concatenated(concatenation) => {
                        let mut map = Vec::new();
                        let mut prev_end = Expr::Lit(ExprLit {
                            attrs: Vec::new(),
                            lit: Lit::Int(LitInt::new("0", Span::call_site())),
                        });
                        for bits in concatenation {
                            let next_end = parens(Expr::Binary(ExprBinary {
                                attrs: Vec::new(),
                                left: Box::new(prev_end.clone()),
                                op: BinOp::Add(Token![+](Span::call_site())),
                                right: Box::new(match &bits.bits_index {
                                    BitsIndex::One(_) => Expr::Lit(ExprLit {
                                        attrs: Vec::new(),
                                        lit: Lit::Int(LitInt::new("1", Span::call_site())),
                                    }),
                                    BitsIndex::Range(start, end) => {
                                        parens(Expr::Binary(ExprBinary {
                                            attrs: Vec::new(),
                                            left: Box::new(end.clone()),
                                            op: BinOp::Sub(Token![-](Span::call_site())),
                                            right: Box::new(start.clone()),
                                        }))
                                    }
                                }),
                            }));
                            map.push((
                                Bits {
                                    array_index: None,
                                    bits_index: BitsIndex::Range(
                                        prev_end.clone(),
                                        next_end.clone(),
                                    ),
                                },
                                bits.clone(),
                            ));
                            prev_end = next_end;
                        }
                        field.value = Value::Mapped(map);
                    }
                }
                uints.push(UintField(field, strukt.clone()));
            }
        }
    }

    let self_nt = match &strukt.fields {
        Fields::Unnamed(f) if f.unnamed.len() == 1 => match &f.unnamed[0].ty {
            Type::Array(t) => (*t.elem).clone(),
            t => t.clone(),
        },
        _ => panic!("the bitfield must be a tuple struct with one field"),
    };

    let expanded = quote! {
        #strukt

        impl #impl_generics #strukt_name #ty_generics #where_clause {
            const fn __self_mask(r: core::ops::Range<u32>) -> #self_nt {
                const N_BITS: u32 = (core::mem::size_of::<#self_nt>() * 8) as u32;
                assert!(r.end <= N_BITS && r.start < N_BITS && r.start <= r.end);
                if r.end - r.start == N_BITS {
                    !0
                } else {
                    ((1 << (r.end - r.start)) - 1) << r.start
                }
            }

            #(#bools)*
            #(#uints)*
            #(#arrays)*
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ConvertInner)]
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
