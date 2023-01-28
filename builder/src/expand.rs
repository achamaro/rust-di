#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, GenericArgument, Ident,
    Lit, Meta, MetaNameValue, NestedMeta, PathArguments, PathSegment, Type, Visibility,
};
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = match ::syn::parse_macro_input::parse::<DeriveInput>(input) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    let ident = input.ident;
    let vis = input.vis;
    let builder_name = match ::quote::__private::IdentFragmentAdapter(&ident) {
        arg => {
            ::quote::__private::mk_ident(
                &{
                    let res = ::alloc::fmt::format(format_args!("{0}Builder", arg));
                    res
                },
                ::quote::__private::Option::None.or(arg.span()),
            )
        }
    };
    let fields = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => fields,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("no unnamed fields are allowed"),
                    )
                }
            }
        }
        _ => {
            ::core::panicking::panic_fmt(
                format_args!("this macro can be applied only to structaa"),
            )
        }
    };
    let builder_struct = build_builder_struct(&fields, &builder_name, &vis);
    let builder_impl = build_builder_impl(&fields, &builder_name, &ident);
    let struct_impl = build_struct_impl(&fields, &builder_name, &ident);
    let expand = {
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::ToTokens::to_tokens(&builder_struct, &mut _s);
        ::quote::ToTokens::to_tokens(&builder_impl, &mut _s);
        ::quote::ToTokens::to_tokens(&struct_impl, &mut _s);
        _s
    };
    proc_macro::TokenStream::from(expand)
}
fn build_builder_struct(
    fields: &FieldsNamed,
    builder_name: &Ident,
    visibility: &Visibility,
) -> proc_macro2::TokenStream {
    let struct_fields = fields
        .named
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref();
            let ty = unwrap_option(&field.ty);
            if is_vector(&ty) {
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::ToTokens::to_tokens(&ty, &mut _s);
                    _s
                }
            } else {
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Option");
                    ::quote::__private::push_lt(&mut _s);
                    ::quote::ToTokens::to_tokens(&ty, &mut _s);
                    ::quote::__private::push_gt(&mut _s);
                    _s
                }
            }
        });
    {
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::ToTokens::to_tokens(&visibility, &mut _s);
        ::quote::__private::push_ident(&mut _s, "struct");
        ::quote::ToTokens::to_tokens(&builder_name, &mut _s);
        ::quote::__private::push_group(
            &mut _s,
            ::quote::__private::Delimiter::Brace,
            {
                let mut _s = ::quote::__private::TokenStream::new();
                {
                    use ::quote::__private::ext::*;
                    let mut _i = 0usize;
                    let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                    #[allow(unused_mut)]
                    let (mut struct_fields, i) = struct_fields.quote_into_iter();
                    let has_iter = has_iter | i;
                    let _: ::quote::__private::HasIterator = has_iter;
                    while true {
                        let struct_fields = match struct_fields.next() {
                            Some(_x) => ::quote::__private::RepInterp(_x),
                            None => break,
                        };
                        if _i > 0 {
                            ::quote::__private::push_comma(&mut _s);
                        }
                        _i += 1;
                        ::quote::ToTokens::to_tokens(&struct_fields, &mut _s);
                    }
                }
                _s
            },
        );
        _s
    }
}
fn build_builder_impl(
    fields: &FieldsNamed,
    builder_name: &Ident,
    struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let checks = fields
        .named
        .iter()
        .filter(|field| !is_option(&field.ty))
        .filter(|field| !is_vector(&field.ty))
        .map(|field| {
            let ident = field.ident.as_ref();
            let err = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "Required field \'{0}\' is missing", ident.unwrap().to_string()
                    ),
                );
                res
            };
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "if");
                ::quote::__private::push_ident(&mut _s, "self");
                ::quote::__private::push_dot(&mut _s);
                ::quote::ToTokens::to_tokens(&ident, &mut _s);
                ::quote::__private::push_dot(&mut _s);
                ::quote::__private::push_ident(&mut _s, "is_none");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    ::quote::__private::TokenStream::new(),
                );
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "return");
                        ::quote::__private::push_ident(&mut _s, "Err");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::ToTokens::to_tokens(&err, &mut _s);
                                ::quote::__private::push_dot(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "into");
                                ::quote::__private::push_group(
                                    &mut _s,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    ::quote::__private::TokenStream::new(),
                                );
                                _s
                            },
                        );
                        ::quote::__private::push_semi(&mut _s);
                        _s
                    },
                );
                _s
            }
        });
    let setters = fields
        .named
        .iter()
        .map(|field| {
            let ident_each_name = field
                .attrs
                .first()
                .map(|attr| match attr.parse_meta() {
                    Ok(Meta::List(list)) => {
                        match list.nested.first() {
                            Some(
                                NestedMeta::Meta(
                                    Meta::NameValue(
                                        MetaNameValue {
                                            path: _,
                                            eq_token: _,
                                            lit: Lit::Str(ref str),
                                        },
                                    ),
                                ),
                            ) => Some(str.value()),
                            _ => None,
                        }
                    }
                    _ => None,
                })
                .flatten();
            let ident = field.ident.as_ref();
            let ty = unwrap_option(&field.ty);
            match ident_each_name {
                Some(name) => {
                    let ty_each = unwrap_vector(ty).unwrap();
                    let ident_each = Ident::new(name.as_str(), Span::call_site());
                    if ident.unwrap().to_string() == name {
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "pub");
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ::quote::ToTokens::to_tokens(&ident_each, &mut _s);
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_and(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "mut");
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident_each, &mut _s);
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ty_each, &mut _s);
                                    _s
                                },
                            );
                            ::quote::__private::push_rarrow(&mut _s);
                            ::quote::__private::push_and(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "mut");
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_dot(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_dot(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "push");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::ToTokens::to_tokens(&ident_each, &mut _s);
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_semi(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    _s
                                },
                            );
                            _s
                        }
                    } else {
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "pub");
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ::quote::ToTokens::to_tokens(&ident, &mut _s);
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_and(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "mut");
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ty, &mut _s);
                                    _s
                                },
                            );
                            ::quote::__private::push_rarrow(&mut _s);
                            ::quote::__private::push_and(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "mut");
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_dot(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_eq(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_semi(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    _s
                                },
                            );
                            ::quote::__private::push_ident(&mut _s, "pub");
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ::quote::ToTokens::to_tokens(&ident_each, &mut _s);
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_and(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "mut");
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident_each, &mut _s);
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ty_each, &mut _s);
                                    _s
                                },
                            );
                            ::quote::__private::push_rarrow(&mut _s);
                            ::quote::__private::push_and(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "mut");
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_dot(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_dot(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "push");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::ToTokens::to_tokens(&ident_each, &mut _s);
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_semi(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    _s
                                },
                            );
                            _s
                        }
                    }
                }
                None => {
                    if is_vector(&ty) {
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "pub");
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ::quote::ToTokens::to_tokens(&ident, &mut _s);
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_and(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "mut");
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ty, &mut _s);
                                    _s
                                },
                            );
                            ::quote::__private::push_rarrow(&mut _s);
                            ::quote::__private::push_and(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "mut");
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_dot(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_eq(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_semi(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    _s
                                },
                            );
                            _s
                        }
                    } else {
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "pub");
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ::quote::ToTokens::to_tokens(&ident, &mut _s);
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_and(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "mut");
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ty, &mut _s);
                                    _s
                                },
                            );
                            ::quote::__private::push_rarrow(&mut _s);
                            ::quote::__private::push_and(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "mut");
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ::quote::__private::push_dot(&mut _s);
                                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                    ::quote::__private::push_eq(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Some");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_semi(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    _s
                                },
                            );
                            _s
                        }
                    }
                }
            }
        });
    let struct_fields = fields
        .named
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref();
            if is_option(&field.ty) || is_vector(&field.ty) {
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "self");
                    ::quote::__private::push_dot(&mut _s);
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_dot(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "clone");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        ::quote::__private::TokenStream::new(),
                    );
                    _s
                }
            } else {
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "self");
                    ::quote::__private::push_dot(&mut _s);
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_dot(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "clone");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        ::quote::__private::TokenStream::new(),
                    );
                    ::quote::__private::push_dot(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "unwrap");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        ::quote::__private::TokenStream::new(),
                    );
                    _s
                }
            }
        });
    {
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::__private::push_ident(&mut _s, "impl");
        ::quote::ToTokens::to_tokens(&builder_name, &mut _s);
        ::quote::__private::push_group(
            &mut _s,
            ::quote::__private::Delimiter::Brace,
            {
                let mut _s = ::quote::__private::TokenStream::new();
                {
                    use ::quote::__private::ext::*;
                    let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                    #[allow(unused_mut)]
                    let (mut setters, i) = setters.quote_into_iter();
                    let has_iter = has_iter | i;
                    let _: ::quote::__private::HasIterator = has_iter;
                    while true {
                        let setters = match setters.next() {
                            Some(_x) => ::quote::__private::RepInterp(_x),
                            None => break,
                        };
                        ::quote::ToTokens::to_tokens(&setters, &mut _s);
                    }
                }
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "fn");
                ::quote::__private::push_ident(&mut _s, "build");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_and(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "mut");
                        ::quote::__private::push_ident(&mut _s, "self");
                        _s
                    },
                );
                ::quote::__private::push_rarrow(&mut _s);
                ::quote::__private::push_ident(&mut _s, "Result");
                ::quote::__private::push_lt(&mut _s);
                ::quote::ToTokens::to_tokens(&struct_name, &mut _s);
                ::quote::__private::push_comma(&mut _s);
                ::quote::__private::push_ident(&mut _s, "Box");
                ::quote::__private::push_lt(&mut _s);
                ::quote::__private::push_ident(&mut _s, "dyn");
                ::quote::__private::push_ident(&mut _s, "std");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "error");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "Error");
                ::quote::__private::push_shr(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut checks, i) = checks.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let checks = match checks.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                ::quote::ToTokens::to_tokens(&checks, &mut _s);
                            }
                        }
                        ::quote::__private::push_ident(&mut _s, "Ok");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::ToTokens::to_tokens(&struct_name, &mut _s);
                                ::quote::__private::push_group(
                                    &mut _s,
                                    ::quote::__private::Delimiter::Brace,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        {
                                            use ::quote::__private::ext::*;
                                            let mut _i = 0usize;
                                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                            #[allow(unused_mut)]
                                            let (mut struct_fields, i) = struct_fields
                                                .quote_into_iter();
                                            let has_iter = has_iter | i;
                                            let _: ::quote::__private::HasIterator = has_iter;
                                            while true {
                                                let struct_fields = match struct_fields.next() {
                                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                                    None => break,
                                                };
                                                if _i > 0 {
                                                    ::quote::__private::push_comma(&mut _s);
                                                }
                                                _i += 1;
                                                ::quote::ToTokens::to_tokens(&struct_fields, &mut _s);
                                            }
                                        }
                                        _s
                                    },
                                );
                                _s
                            },
                        );
                        _s
                    },
                );
                _s
            },
        );
        _s
    }
}
fn build_struct_impl(
    fields: &FieldsNamed,
    builder_name: &Ident,
    struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let field_defaults = fields
        .named
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref();
            let ty = &field.ty;
            if is_vector(&ty) {
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Vec");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "new");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        ::quote::__private::TokenStream::new(),
                    );
                    _s
                }
            } else {
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "None");
                    _s
                }
            }
        });
    {
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::__private::push_ident(&mut _s, "impl");
        ::quote::ToTokens::to_tokens(&struct_name, &mut _s);
        ::quote::__private::push_group(
            &mut _s,
            ::quote::__private::Delimiter::Brace,
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "fn");
                ::quote::__private::push_ident(&mut _s, "builder");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    ::quote::__private::TokenStream::new(),
                );
                ::quote::__private::push_rarrow(&mut _s);
                ::quote::ToTokens::to_tokens(&builder_name, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::ToTokens::to_tokens(&builder_name, &mut _s);
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                {
                                    use ::quote::__private::ext::*;
                                    let mut _i = 0usize;
                                    let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                    #[allow(unused_mut)]
                                    let (mut field_defaults, i) = field_defaults
                                        .quote_into_iter();
                                    let has_iter = has_iter | i;
                                    let _: ::quote::__private::HasIterator = has_iter;
                                    while true {
                                        let field_defaults = match field_defaults.next() {
                                            Some(_x) => ::quote::__private::RepInterp(_x),
                                            None => break,
                                        };
                                        if _i > 0 {
                                            ::quote::__private::push_comma(&mut _s);
                                        }
                                        _i += 1;
                                        ::quote::ToTokens::to_tokens(&field_defaults, &mut _s);
                                    }
                                }
                                _s
                            },
                        );
                        _s
                    },
                );
                _s
            },
        );
        _s
    }
}
fn get_last_path_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(path) => path.path.segments.last(),
        _ => None,
    }
}
fn is_option(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Option",
        None => false,
    }
}
fn is_vector(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Vec",
        _ => false,
    }
}
fn unwrap_option(ty: &Type) -> &Type {
    unwrap_generic_type(ty).unwrap_or(ty)
}
fn unwrap_vector(ty: &Type) -> Option<&Type> {
    if !is_vector(ty) {
        return None;
    }
    return unwrap_generic_type(ty);
}
fn unwrap_generic_type(ty: &Type) -> Option<&Type> {
    if let Some(seg) = get_last_path_segment(ty) {
        if let PathArguments::AngleBracketed(ref args) = seg.arguments {
            if let Some(&GenericArgument::Type(ref ty)) = args.args.first() {
                return Some(ty);
            }
        }
    }
    return None;
}
const _: () = {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    #[used]
    #[allow(deprecated)]
    static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[
        proc_macro::bridge::client::ProcMacro::custom_derive(
            "Builder",
            &["builder"],
            derive,
        ),
    ];
};
