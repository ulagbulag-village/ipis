use proc_macro2::TokenStream;
use quote::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream},
    DataEnum, DataUnion, FieldsNamed, FieldsUnnamed, GenericParam, Generics, Ident, Lit, LitStr,
    Meta, MetaNameValue, PathArguments, Type,
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
                // Add a bound `T: Class` to every type parameter T.
                let generics_for_class = add_trait_bounds_for_class(&generics);
                let generics_for_object = add_trait_bounds_for_object(&generics);
                let (impl_generics_for_class, ty_generics, where_clause_for_class) = generics_for_class.split_for_impl();
                let (impl_generics_for_object, _, where_clause_for_object) = generics_for_object.split_for_impl();

                let fields  = fields.iter().filter_map(|f| {
                    let ident = f.ident.as_ref()?;
                    let mut ty = f.ty.clone();
                    attach_colon2_on_class(&mut ty);
                    Some((ident, ty))
                });

                // parse class children
                let class_children = fields.clone().map(|(_, ty)| {
                    quote! { <#ty as ::ipis::class::Class>::__class_metadata() }
                });

                // parse object children
                let to_object_children = fields.clone().map(|(ident, _)| {
                    quote! { ::ipis::object::ToObjectData::__to_object_data(&self.#ident) }
                });

                // parse object value
                let get_object_value = fields.clone().map(|(ident, _)| {
                    quote! { stringify!(#ident) => ::ipis::object::ToObjectData::__get_object_value(&self.#ident, &path[1..]) }
                });

                // parse object data
                let get_object_data = fields.clone().map(|(ident, _)| {
                    quote! { stringify!(#ident) => ::ipis::object::ToObjectData::__get_object_data(&self.#ident, &path[1..]) }
                });

                // parse cursor methods
                let cursor_methods = fields.clone().map(|(ident, ty)| {
                    quote! {
                        pub fn #ident(self) -> <#ty as ::ipis::class::Class>::Cursor {
                            let mut data = self.0;
                            data.push_en_us(
                                stringify!(#ident),
                                <#ty as ::ipis::class::Class>::__class_metadata_leaf(),
                            );
                            data.into()
                        }
                    }
                });

                // implement the trait
                Ok(quote! {
                    const _: () = {
                        use ::std::borrow::Cow;

                        impl #impl_generics_for_class ::ipis::class::Class for #ident #ty_generics #where_clause_for_class {
                            type Cursor = Cursor;

                            fn __class_name() -> ::ipis::class::metadata::ClassName {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_name()
                            }

                            fn __class_doc() -> ::ipis::class::metadata::ClassDoc {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_doc()
                            }

                            fn __class_value_ty() -> ::ipis::core::value::ValueType {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_value_ty()
                            }

                            fn __class_children() -> Option<Vec<::ipis::class::metadata::ClassMetadata>> {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_children()
                            }

                            fn __class_metadata() -> ::ipis::class::metadata::ClassMetadata {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_metadata()
                            }

                            fn __class_metadata_leaf() -> ::ipis::class::metadata::ClassLeaf {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_metadata_leaf()
                            }

                            fn class_cursor() -> <Self as ::ipis::class::Class>::Cursor {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::class_cursor()
                            }
                        }

                        impl #impl_generics_for_object ::ipis::object::Object for #ident #ty_generics #where_clause_for_object {
                            type Cursor = <Self as ::ipis::class::Class>::Cursor;

                            fn __object_name(&self) -> Cow<::ipis::class::metadata::ClassName> {
                                Cow::Owned(
                                    <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_name(),
                                )
                            }

                            fn __object_doc(&self) -> Cow<::ipis::class::metadata::ClassDoc> {
                                Cow::Owned(
                                    <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_doc(),
                                )
                            }

                            fn __object_value_ty(&self) -> ::ipis::core::value::ValueType {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_value_ty()
                            }

                            fn __object_metadata(&self) -> ::ipis::class::metadata::ClassMetadata {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_metadata()
                            }

                            fn __object_metadata_leaf(&self) -> Cow<::ipis::class::metadata::ClassLeaf> {
                                Cow::Owned(
                                    <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::__class_metadata_leaf(),
                                )
                            }

                            fn cursor(&self) -> <Self as ::ipis::class::Class>::Cursor {
                                <<Self as ::ipis::class::Class>::Cursor as ::ipis::class::Class>::class_cursor()
                            }
                        }

                        impl #impl_generics_for_object ::ipis::object::ToObjectData for #ident #ty_generics #where_clause_for_object {
                            fn __to_object_value(&self) -> Option<::ipis::core::value::Value> {
                                Some(::ipis::core::value::Value::Dyn)
                            }

                            fn __to_object_children(&self) -> Option<Vec<::ipis::object::data::ObjectData>> {
                                Some(vec![#(
                                    #to_object_children,
                                )*])
                            }

                            fn __get_object_value(&self, path: &[::ipis::core::value::text::Text]) -> Option<::ipis::core::value::Value> {
                                if path.is_empty() {
                                    self.__to_object_value()
                                } else {
                                    match path[0].msg.as_str() {
                                        #(
                                            #get_object_value,
                                        )*
                                        _ => None,
                                    }
                                }
                            }

                            fn __get_object_data(&self, path: &[::ipis::core::value::text::Text]) -> Option<::ipis::object::data::ObjectData> {
                                if path.is_empty() {
                                    Some(self.__to_object_data())
                                } else {
                                    match path[0].msg.as_str() {
                                        #(
                                            #get_object_data,
                                        )*
                                        _ => None,
                                    }
                                }
                            }
                        }

                        impl #impl_generics_for_object ::ipis::object::IntoObjectData for #ident #ty_generics #where_clause_for_object {}

                        #[derive(Clone, Default)]
                        pub struct Cursor(::ipis::class::cursor::ClassCursorData);

                        impl From<::ipis::class::cursor::ClassCursorData> for Cursor {
                            fn from(value: ::ipis::class::cursor::ClassCursorData) -> Self {
                                Self(value)
                            }
                        }

                        impl ::core::ops::Deref for Cursor {
                            type Target = ::ipis::class::cursor::ClassCursorData;

                            fn deref(&self) -> &<Self as ::core::ops::Deref>::Target {
                                &self.0
                            }
                        }

                        impl ::core::fmt::Debug for Cursor {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                ::core::fmt::Debug::fmt(&self.0, f)
                            }
                        }

                        impl ::core::fmt::Display for Cursor {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                ::core::fmt::Display::fmt(&self.0, f)
                            }
                        }

                        impl #impl_generics_for_class ::ipis::class::Class for Cursor {
                            type Cursor = Self;

                            fn __class_name() -> ::ipis::class::metadata::ClassName {
                                ::ipis::core::value::text::Text::with_en_us(stringify!(#ident))
                            }

                            fn __class_doc() -> ::ipis::class::metadata::ClassDoc {
                                #doc.map(::ipis::core::value::text::Text::with_en_us)
                            }

                            fn __class_value_ty() -> ::ipis::core::value::ValueType {
                                ::ipis::core::value::ValueType::Dyn
                            }

                            fn __class_children() -> Option<Vec<::ipis::class::metadata::ClassMetadata>> {
                                Some(vec![#(
                                    #class_children,
                                )*])
                            }
                        }

                        impl #impl_generics_for_object ::ipis::object::Object for Cursor {
                            type Cursor = Self;

                            fn __object_name(&self) -> Cow<::ipis::class::metadata::ClassName> {
                                Cow::Owned(<Self as ::ipis::class::Class>::__class_name())
                            }

                            fn __object_doc(&self) -> Cow<::ipis::class::metadata::ClassDoc> {
                                Cow::Owned(<Self as ::ipis::class::Class>::__class_doc())
                            }

                            fn __object_value_ty(&self) -> ::ipis::core::value::ValueType {
                                <Self as ::ipis::class::Class>::__class_value_ty()
                            }

                            fn __object_metadata(&self) -> ::ipis::class::metadata::ClassMetadata {
                                <Self as ::ipis::class::Class>::__class_metadata()
                            }

                            fn __object_metadata_leaf(&self) -> Cow<::ipis::class::metadata::ClassLeaf> {
                                Cow::Owned(<Self as ::ipis::class::Class>::__class_metadata_leaf())
                            }

                            fn cursor(&self) -> <Self as ::ipis::class::Class>::Cursor {
                                self.clone()
                            }
                        }

                        impl #impl_generics_for_class Cursor {
                            #(
                                #cursor_methods
                            )*
                        }
                    };
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
fn add_trait_bounds_for_class(generics: &Generics) -> Generics {
    let mut generics = generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::ipis::class::Class));
        }
    }
    generics
}

// Add a bound `T: Object` to every type parameter T.
fn add_trait_bounds_for_object(generics: &Generics) -> Generics {
    let mut generics = generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::ipis::object::Object));
        }
    }
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
