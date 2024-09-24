use crate::struct_builder::{BuilderContext, GenericsContext};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Expr, Field, FieldValue, Fields, Index, ItemImpl, ItemStruct, Token, Type};

pub struct ImplSubjectFnBuilder {
    ctx: BuilderContext,
    fields: Fields
}

impl From<&ItemStruct> for ImplSubjectFnBuilder {
    fn from(value: &ItemStruct) -> Self {
        let ctx: BuilderContext = value.into();
        let fields = value.fields.clone();
        
        Self { ctx, fields }
    }
}

impl ToTokens for ImplSubjectFnBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext { 
            subject,
            params,
            params_argument,
            builder,
            builder_subject_field,
            generics,
            fields_metadata,
            ..
        } = &self.ctx;
        
        let optional_expr: Option<Expr> = match &self.fields {
            Fields::Named(named_fields) => {
                let punctuated_fields = named_fields.named
                    .iter()
                    .map::<FieldValue, _>(|field| {
                        let field_ident = field.ident.as_ref().expect("named field missing ident");
                        if is_required(&field) {
                            parse_quote! { #field_ident: #params_argument.#field_ident }
                        } else {
                            parse_quote! { #field_ident: ::std::option::Option::None }
                        }
                    })
                    .collect::<Punctuated<FieldValue, Token![,]>>();
                
                Some(parse_quote! { Self { #punctuated_fields } })
            },
            
            Fields::Unnamed(unnamed_fields) => {
                let mut next_index = 0;
                let punctuated_fields = unnamed_fields.unnamed
                    .iter()
                    .map::<Expr, _>(|field|
                        if is_required(&field) {
                            let index = Index::from(next_index);
                            next_index += 1;
                            parse_quote! { #params_argument.#index }
                        } else {
                            parse_quote! { ::std::option::Option::None }
                        }
                    )
                    .collect::<Punctuated<Expr, Token![,]>>();
                
                Some(parse_quote! { Self(#punctuated_fields) })
            },
            
            Fields::Unit => None
        };

        if let Some(expr) = optional_expr {
            let GenericsContext {
                generics_def,
                generics_expr,
                where_clause
            } = &generics;

            let include_params_generics = fields_metadata.generic_required_fields_count > 0;
            
            let item_impl: ItemImpl = if include_params_generics {
                parse_quote! {
                    impl #generics_def #subject #generics_expr #where_clause {
                        pub fn builder(#params_argument: #params #generics_expr) -> #builder #generics_expr {
                            #builder {
                                #builder_subject_field: #expr
                            }
                        }
                    }
                }
            } else {
                parse_quote! {
                    impl #generics_def #subject #generics_expr #where_clause {
                        pub fn builder(#params_argument: #params) -> #builder #generics_expr {
                            #builder {
                                #builder_subject_field: #expr
                            }
                        }
                    }
                }
            };

            item_impl.to_tokens(tokens);   
        }
    }
}

pub fn is_required(field: &Field) -> bool {
    match &field.ty {
        Type::Path(path_type) => path_type.path.segments
            .last()
            .map(|seg| seg.ident != "Option")
            .unwrap_or(true),
        
        _ => true
    }
}

#[cfg(test)]
mod tests {
    use crate::components::impl_subject_fn_builder::ImplSubjectFnBuilder;
    use crate::test_util::{sample_named_item_struct, sample_unit_item_struct, sample_unnamed_item_struct};
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, ItemImpl};

    #[test]
    fn test_with_named_fields() {
        let item_struct = sample_named_item_struct();
        let expected: ItemImpl = parse_quote! {
            impl<T, I: Send, W> MyStruct<T, I, W>
            where
                W: Sync
            {
                pub fn builder(params: MyStructParams<T, I, W>) -> MyStructBuilder<T, I, W> {
                    MyStructBuilder {
                        inner: Self {
                            public_field: params.public_field,
                            private_field: params.private_field,
                            optional: ::std::option::Option::None,
                            test: ::std::option::Option::None,
                            test2: ::std::option::Option::None,
                            dynamic: params.dynamic,
                            dynamic2: params.dynamic2,
                            generic: params.generic,
                            generic_inline: params.generic_inline,
                            generic_where: params.generic_where
                        }
                    }
                }
            }
        };
        
        let impl_subject_fn_builder = ImplSubjectFnBuilder::from(&item_struct);

        assert_eq!(
            impl_subject_fn_builder.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_with_unnamed_fields() {
        let item_struct = sample_unnamed_item_struct();
        let expected: ItemImpl = parse_quote! {
            impl<T, I: Send, W> MyStruct<T, I, W>
            where
                W: Sync
            {
                pub fn builder(params: MyStructParams<T, I, W>) -> MyStructBuilder<T, I, W> {
                    MyStructBuilder {
                        inner: Self(
                            params.0,
                            params.1,
                            ::std::option::Option::None,
                            ::std::option::Option::None,
                            ::std::option::Option::None,
                            params.2,
                            params.3,
                            params.4,
                            params.5,
                            params.6
                        )
                    }
                }
            }
        };

        let subject_impl = ImplSubjectFnBuilder::from(&item_struct);

        assert_eq!(
            subject_impl.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unit_struct() {
        let item_struct = sample_unit_item_struct();
        
        let subject_impl = ImplSubjectFnBuilder::from(&item_struct);
        
        assert_eq!(
            subject_impl.to_token_stream().to_string(),
            TokenStream::new().to_string()
        );
    }
}
