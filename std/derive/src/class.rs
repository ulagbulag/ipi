use proc_macro2::TokenStream;
use quote::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream},
    DataEnum, DataUnion, FieldsNamed, FieldsUnnamed, GenericParam, Generics, Ident, LifetimeDef,
    Lit, LitStr, Meta, MetaNameValue, PathArguments, Type,
};

pub fn expand_derive_serialize(input: syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let input_span = input.__span();
    let syn::DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = input;

    // be ready for parsing attributes
    let mut doc = None;
    let mut doc_native = vec![];
    for attr in attrs {
        if attr.path.is_ident("doc") {
            if let Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            }) = attr.parse_meta().map_err(|e| vec![e])?
            {
                if !path.is_ident("doc") {
                    return Err(vec![syn::Error::new(
                        attr.__span(),
                        format!("duplicate doc attribute `{}`", quote! { #attr },),
                    )]);
                }
                doc_native.push(lit);
            }
        } else if attr.path.is_ident("class") {
            struct Attributes {
                attrs_cls: Vec<Attribute>,
            }

            struct Attribute {
                name: Ident,
                value: Lit,
            }

            impl Parse for Attributes {
                fn parse(input: ParseStream) -> syn::Result<Self> {
                    Ok(Self {
                        attrs_cls: {
                            let mut result: Vec<Attribute> = vec![];
                            loop {
                                result.push(input.parse()?);
                                if input.peek(Token![,]) {
                                    input.parse::<Token![,]>()?;
                                    continue;
                                } else {
                                    break result;
                                }
                            }
                        },
                    })
                }
            }

            impl Parse for Attribute {
                fn parse(input: ParseStream) -> syn::Result<Self> {
                    let name = input.parse()?;
                    let _eq_token: Token![=] = input.parse()?;
                    let value = input.parse()?;
                    Ok(Self { name, value })
                }
            }

            let args: Attributes = attr.parse_args().map_err(|e| vec![e])?;
            for attr in args.attrs_cls {
                fn update_attr_value(
                    attr: Attribute,
                    var: &mut Option<Lit>,
                ) -> Result<(), Vec<syn::Error>> {
                    let Attribute { name, value, .. } = attr;

                    if var.replace(value).is_some() {
                        return Err(vec![syn::Error::new(
                            name.span(),
                            format!("duplicated class attribute `{}`", quote! { #name },),
                        )]);
                    }
                    Ok(())
                }

                let name = &attr.name;
                if name == "doc" {
                    update_attr_value(attr, &mut doc)?;
                } else {
                    return Err(vec![syn::Error::new(
                        name.span(),
                        format!("unknown class attribute `{}`", quote! { #name },),
                    )]);
                }
            }
        }
    }

    fn parse_attr(attr: Option<Lit>, attr_native: Option<Vec<LitStr>>) -> TokenStream {
        attr.map(|e| quote! { Some(#e) })
            .or_else(|| match &attr_native {
                Some(attr) if !attr.is_empty() => {
                    let attr = attr
                        .iter()
                        .map(|e| e.value())
                        .collect::<Vec<_>>()
                        .join("\n");
                    Some(quote! { Some(#attr) })
                }
                _ => None,
            })
            .unwrap_or_else(|| quote!(::core::option::Option::<&'static str>::None))
    }

    // parse attributes
    let doc = parse_attr(doc, Some(doc_native));

    match data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(FieldsNamed { named: fields, .. }) => {
                let ident_mod = Ident::new(&format!("__{}", &ident), ident.__span());

                // Add a bound `T: Class` to every type parameter T.
                let generics_for_class = add_trait_bounds(generics);
                let generics_for_object = add_object_bounds(generics_for_class.clone());
                let (impl_generics_for_class, ty_generics, where_clause_for_class) = generics_for_class.split_for_impl();
                let (impl_generics_for_object, _, where_clause_for_object) = generics_for_object.split_for_impl();

                let fields  = fields.iter().filter_map(|f| {
                    let ident = f.ident.as_ref()?;
                    let mut ty = f.ty.clone();
                    attach_colon2_on_class(&mut ty);
                    Some((ident, ty))
                });

                // parse children
                let children = fields.clone().map(|(_, ty)| {
                    quote! { #ty::__class_template() }
                });

                // parse children converted from object data
                let children_from_object_data = fields.clone().map(|(ident, _)| {
                    quote! {
                        #ident: ::ipi::traits::Object::<__S>::__try_from_object_data(
                            __children.get(stringify!(#ident))?,
                        )?
                    }
                });

                // parse children converting to object data
                let children_to_object_data = fields.clone().map(|(ident, _)| {
                    quote! {
                        stringify!(#ident).into(),
                        ::ipi::traits::Object::<__S>::__to_object_data(&self.#ident),
                    }
                });

                // parse cursor children
                let cursor_children: Vec<_> = fields.clone().map(|(ident, _)| {
                    Some(quote! { self.clone().#ident() })
                }).collect();

                // parse cursor methods
                let cursor_methods = fields.clone().map(|(ident, ty)| {
                    quote! {
                        pub fn #ident(self) -> <#ty as ::ipi::Class>::Cursor {
                            let data = self.0.push(
                                stringify!(#ident).into(),
                                <#ty as ::ipi::Class>::__class_value_ty(),
                                <#ty as ::ipi::Class>::__class_default_attention(),
                                <#ty as ::ipi::Class>::__class_default_confidence(),
                            );
                            <#ty as ::ipi::Class>::Cursor::__from_data(data)
                        }
                    }
                });

                // implement the trait
                Ok(quote! {
                    #[allow(non_snake_case)]
                    mod #ident_mod {
                        use super::*;

                        // impl #impl_generics_for_class ::ipi::Class for #ident #ty_generics #where_clause_for_class {
                        //     type Cursor = self::cursors::Cursor<'static>;

                        //     fn __class_name() -> ::core::borrow::Cow<'static, str> {
                        //         stringify!(#ident).into()
                        //     }

                        //     fn __class_doc() -> Option<::ipi::core::value::Text<'static>> {
                        //         #doc.map(::ipi::schema::Text::with_en_us)
                        //     }

                        //     fn __class_value_ty() -> ::ipi::schema::ClassType {
                        //         ::ipi::schema::ClassType::Primitive(
                        //             // note: the object itself has no value.
                        //             ::ipi::schema::ClassTypePrimitive::None,
                        //         )
                        //     }

                        //     fn __class_children() -> Option<Vec<::ipi::schema::ClassTemplate<'static>>> {
                        //         Some(vec![#(
                        //             #children,
                        //         )*])
                        //     }

                        //     fn cursor() -> Self::Cursor {
                        //         Self::Cursor::default()
                        //     }
                        // }

                        // impl #impl_generics_for_object ::ipi::traits::Object<'__object, __S> for #ident #ty_generics #where_clause_for_object
                        // where
                        //     __S: ::ipi::schema::StorageTypes,
                        // {
                        //     fn __try_from_object_data(object: &::ipi::schema::ObjectData<'__object, __S>) -> Option<Self>
                        //     {
                        //         let __children = object.children.as_ref()?;
                        //         Some(Self {
                        //             #(
                        //                 #children_from_object_data,
                        //             )*
                        //         })
                        //     }

                        //     fn __to_object_data(&self) -> ::ipi::schema::ObjectData<'__object, __S>
                        //     {
                        //         ::ipi::schema::ObjectData {
                        //             _storage_types: Default::default(),
                        //             metadata: Self::__class_metadata(),
                        //             attention: <Self as ::ipi::traits::Object<'__object, __S>>::__default_attention(),
                        //             confidence: <Self as ::ipi::traits::Object<'__object, __S>>::__default_confidence(),
                        //             value: ::ipi::schema::ObjectValue::Primitive(
                        //                 ::ipi::schema::ObjectValuePrimitive::None,
                        //             ),
                        //             children: Some({
                        //                 let mut __children = ::std::collections::HashMap::default();
                        //                 #(
                        //                     __children.insert(#children_to_object_data);
                        //                 )*
                        //                 __children
                        //             })
                        //         }
                        //     }
                        // }

                        // mod cursors {
                        //     use ::ipi::schema::CursorData;
                        //     use ::ipi::traits::{IntoCursorData, ToCursorData};

                        //     use super::super::*;

                        //     #[derive(Clone, Default)]
                        //     pub struct Cursor<'a>(CursorData<'a>);

                        //     impl<'a> ::core::fmt::Debug for Cursor<'a> {
                        //         fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        //             self.0.fmt(f)
                        //         }
                        //     }

                        //     impl<'a> ::core::fmt::Display for Cursor<'a> {
                        //         fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        //             self.0.fmt(f)
                        //         }
                        //     }

                        //     impl<'a> ToCursorData for Cursor<'a> {
                        //         type Value = ();

                        //         fn __class_value_ty(&self) -> ::ipi::schema::ClassType {
                        //             <#ident as ::ipi::Class>::__class_value_ty()
                        //         }

                        //         fn __as_cursor_data(&self) -> &CursorData<'_> {
                        //             &self.0
                        //         }
                        //     }

                        //     impl IntoCursorData for Cursor<'static> {
                        //         fn __from_data(parent: CursorData<'static>) -> Self {
                        //             Self(parent)
                        //         }

                        //         fn __into_cursor_data(self) -> CursorData<'static> {
                        //             self.0
                        //         }

                        //         fn __get_all(&self) -> Vec<CursorData<'static>> {
                        //             let mut result = vec![self.0.clone()];
                        //             #( result.append(&mut #cursor_children.__get_all()); )*
                        //             result
                        //         }

                        //         fn __get_children(&self) -> Vec<CursorData<'static>> {
                        //             vec![#(
                        //                 #cursor_children.__into_cursor_data(),
                        //             )*]
                        //         }
                        //     }

                        //     impl #impl_generics_for_class Cursor<'static> {
                        //         #(
                        //             #cursor_methods
                        //         )*
                        //     }
                        // }
                    }
                })
            }
            syn::Fields::Unnamed(FieldsUnnamed { .. }) => {
                 Err(vec![syn::Error::new(
                    input_span,
                    format!(
                        "Cannot define the class \"{}\": Structs with unnamed fields are not supported yet.",
                        quote! {#ident},
                    ),
                )])
            }
            syn::Fields::Unit => {
                 Err(vec![syn::Error::new(
                    input_span,
                    format!(
                        "Cannot define the class \"{}\": Structs without fields are not supported yet.",
                        quote! {#ident},
                    ),
                )])
            }
        },
        syn::Data::Enum(DataEnum { .. }) => {
             Err(vec![syn::Error::new(
                input_span,
                format!(
                    "Cannot define the class \"{}\": Enums are not supported yet",
                    quote! {#ident},
                ),
            )])
        }
        syn::Data::Union(DataUnion {
            fields: FieldsNamed { .. },
            ..
        }) => {
             Err(vec![syn::Error::new(
                input_span,
                format!(
                    "Cannot define the class \"{}\": Unions are not supported yet",
                    quote! {#ident},
                ),
            )])
        }
    }
}

// Add a bound `T: Class` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::ipi::Class));
        }
    }
    generics
}

// Add a object bound `<'__object, __S>`.
fn add_object_bounds(mut generics: Generics) -> Generics {
    // lifetime bounds
    let lifetimes = generics.params.iter_mut().filter_map(|param| match param {
        GenericParam::Lifetime(lifetime_param) => {
            lifetime_param.bounds.push(parse_quote!('__object));
            Some(&lifetime_param.lifetime)
        }
        _ => None,
    });

    let mut lifetime_object: LifetimeDef = parse_quote!('__object);
    lifetimes.for_each(|lifetime| lifetime_object.bounds.push(parse_quote!(#lifetime)));

    generics.params.push(lifetime_object.into());
    generics.params.push(parse_quote!(__S));
    generics
}

// Add `::` on each type segment.
fn attach_colon2_on_class(ty: &mut Type) -> &mut Type {
    if let Type::Path(syn::TypePath { path, .. }) = ty {
        for segment in path.segments.iter_mut() {
            if let PathArguments::AngleBracketed(arguments) = &mut segment.arguments {
                arguments.colon2_token = Some(Token![::](arguments.args.__span()));
            }
        }
    }
    ty
}
