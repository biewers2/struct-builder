use crate::struct_builder::BuilderIdents;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Expr, Field, FieldValue, Fields, Index, ItemImpl, ItemStruct, Token, Type};

pub struct ImplSubjectFnBuilder {
    idents: BuilderIdents,
    fields: Fields
}

impl From<&ItemStruct> for ImplSubjectFnBuilder {
    fn from(value: &ItemStruct) -> Self {
        let idents = BuilderIdents::from(value);
        let fields = value.fields.clone();
        
        Self { idents, fields }
    }
}

impl ToTokens for ImplSubjectFnBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderIdents { 
            subject,
            params,
            params_argument,
            builder,
            builder_subject_field
        } = &self.idents;
        
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
            let item_impl: ItemImpl = parse_quote! {
                impl #subject {
                    pub fn builder(#params_argument: #params) -> #builder {
                        #builder {
                            #builder_subject_field: #expr
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
    use proc_macro2::TokenStream;
    use crate::components::impl_subject_fn_builder::ImplSubjectFnBuilder;
    use quote::ToTokens;
    use syn::{parse_quote, ItemImpl, ItemStruct};
    
    #[test]
    fn test_with_named_fields() {
        let item_struct: ItemStruct = parse_quote! {
            pub struct MyStruct {
                pub public_field: String,
                private_field: String,
                optional: Option<usize>,
                pub test: std::option::Option<String>,
                test2: option::Option<T>,
                pub dynamic: Box<dyn Send>,
                pub dynamic2: Box<Option<dyn Send>>
            }
        };
        let expected: ItemImpl = parse_quote! {
            impl MyStruct {
                pub fn builder(params: MyStructParams) -> MyStructBuilder {
                    MyStructBuilder {
                        inner: Self {
                            public_field: params.public_field,
                            private_field: params.private_field,
                            optional: ::std::option::Option::None,
                            test: ::std::option::Option::None,
                            test2: ::std::option::Option::None,
                            dynamic: params.dynamic,
                            dynamic2: params.dynamic2
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
        let item_struct: ItemStruct = parse_quote! {
            pub struct MyStruct(
                pub String,
                String,
                Option<usize>,
                pub std::option::Option<String>,
                option::Option<T>,
                pub Box<dyn Send>,
                pub Box<Option<dyn Send>>
            );
        };
        let expected: ItemImpl = parse_quote! {
            impl MyStruct {
                pub fn builder(params: MyStructParams) -> MyStructBuilder {
                    MyStructBuilder {
                        inner: Self(
                            params.0,
                            params.1,
                            ::std::option::Option::None,
                            ::std::option::Option::None,
                            ::std::option::Option::None,
                            params.2,
                            params.3
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
        let item_struct = parse_quote! { pub struct MyStruct; };
        
        let subject_impl = ImplSubjectFnBuilder::from(&item_struct);
        
        assert_eq!(
            subject_impl.to_token_stream().to_string(),
            TokenStream::new().to_string()
        );
    }
}
