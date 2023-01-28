use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, GenericArgument, Ident, Lit, Meta,
    MetaNameValue, NestedMeta, PathArguments, PathSegment, Type, Visibility,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let vis = input.vis;

    let builder_name = format_ident!("{}Builder", ident);

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("no unnamed fields are allowed"),
        },
        _ => panic!("this macro can be applied only to structaa"),
    };

    let builder_struct = build_builder_struct(&fields, &builder_name, &vis);
    let builder_impl = build_builder_impl(&fields, &builder_name, &ident);
    let struct_impl = build_struct_impl(&fields, &builder_name, &ident);

    let expand = quote! {
        #builder_struct
        #builder_impl
        #struct_impl
    };
    proc_macro::TokenStream::from(expand)
}

fn build_builder_struct(
    fields: &FieldsNamed,
    builder_name: &Ident,
    visibility: &Visibility,
) -> proc_macro2::TokenStream {
    let struct_fields = fields.named.iter().map(|field| {
        let ident = field.ident.as_ref();
        let ty = unwrap_option(&field.ty);
        if is_vector(&ty) {
            quote! {
                #ident: #ty
            }
        } else {
            quote! {
                #ident: Option<#ty>
            }
        }
    });

    quote! {
        #visibility struct #builder_name {
            #(#struct_fields),*
        }
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
            let err = format!("Required field '{}' is missing", ident.unwrap().to_string());
            quote! {
                if self.#ident.is_none() {
                    return Err(#err.into());
                }
            }
        });

    let setters = fields.named.iter().map(|field| {
        let ident_each_name = field
            .attrs
            .first()
            .map(|attr| match attr.parse_meta() {
                Ok(Meta::List(list)) => match list.nested.first() {
                    Some(NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        path: _,
                        eq_token: _,
                        lit: Lit::Str(ref str),
                    }))) => Some(str.value()),
                    _ => None,
                },
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
                    quote! {
                        pub fn #ident_each(&mut self, #ident_each:#ty_each) -> &mut Self {
                            self.#ident.push(#ident_each);
                            self
                        }
                    }
                } else {
                    quote! {
                        pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = #ident;
                            self
                        }
                        pub fn #ident_each(&mut self, #ident_each: #ty_each) -> &mut Self {
                            self.#ident.push(#ident_each);
                            self
                        }
                    }
                }
            }
            None => {
                if is_vector(&ty) {
                    quote! {
                        pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = #ident;
                            self
                        }
                    }
                } else {
                    quote! {
                        pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = Some(#ident);
                            self
                        }
                    }
                }
            }
        }
    });

    let struct_fields = fields.named.iter().map(|field| {
        let ident = field.ident.as_ref();
        if is_option(&field.ty) || is_vector(&field.ty) {
            quote! {
                #ident: self.#ident.clone()
            }
        } else {
            quote! {
                #ident: self.#ident.clone().unwrap()
            }
        }
    });

    quote! {
        impl #builder_name {
            #(#setters)*

            pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                #(#checks)*
                Ok(#struct_name {
                    #(#struct_fields),*
                })
            }
        }
    }
}

fn build_struct_impl(
    fields: &FieldsNamed,
    builder_name: &Ident,
    struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let field_defaults = fields.named.iter().map(|field| {
        let ident = field.ident.as_ref();
        let ty = &field.ty;
        if is_vector(&ty) {
            quote! {
                #ident: Vec::new()
            }
        } else {
            quote! {
                #ident: None
            }
        }
    });
    quote! {
        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_defaults),*
                }
            }
        }
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
        _ => false,
    }
}

fn is_vector(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Vec",
        _ => false,
    }
}

fn unwrap_option(ty: &Type) -> &Type {
    if !is_option(ty) {
        return ty;
    }
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
