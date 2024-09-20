use proc_macro2::TokenStream;
use crate::struct_builder::BuilderContext;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, Fields, ImplItemFn, Index, ItemImpl, ItemStruct};

pub struct ImplBuilderFns {
    idents: BuilderContext,
    fields: Fields
}

impl From<&ItemStruct> for ImplBuilderFns {
    fn from(value: &ItemStruct) -> Self {
        let idents = BuilderContext::from(value);
        let fields = value.fields.clone();

        Self { idents, fields }
    }
}

impl ToTokens for ImplBuilderFns {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            subject,
            builder,
            builder_subject_field,
            ..
        } = &self.idents;

        let optional_functions: Option<Vec<ImplItemFn>> = match &self.fields {
            Fields::Named(named_fields) => {
                let fns = named_fields.named
                    .iter()
                    .map(|field| {
                        let field_ident = field.ident.as_ref().expect("named field missing ident");
                        let field_type = &field.ty;
                        let fn_ident = format_ident!("with_{}", &field_ident);

                        parse_quote! {
                            pub fn #fn_ident(mut self, value: #field_type) -> Self {
                                self.#builder_subject_field.#field_ident = value;
                                self
                            }
                        }
                    })
                    .collect::<Vec<ImplItemFn>>();

                Some(fns)
            },

            Fields::Unnamed(unnamed_fields) => {
                let fns = unnamed_fields.unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let field_index = Index::from(i);
                        let field_type = &field.ty;
                        let fn_ident = format_ident!("with_{}", &field_index);

                        parse_quote! {
                            pub fn #fn_ident(mut self, value: #field_type) -> Self {
                                self.#builder_subject_field.#field_index = value;
                                self
                            }
                        }
                    })
                    .collect::<Vec<ImplItemFn>>();

                Some(fns)
            },

            Fields::Unit => None
        };

        if let Some(functions) = optional_functions {
            let item_impl: ItemImpl = parse_quote! {
                impl #builder {
                    #(#functions)*

                    pub fn build(self) -> #subject {
                        self.#builder_subject_field
                    }
                }
            };

            item_impl.to_tokens(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, ItemImpl, ItemStruct};
    use crate::components::ImplBuilderFns;

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
            impl MyStructBuilder {
                pub fn with_public_field(mut self, value: String) -> Self {
                    self.inner.public_field = value;
                    self
                }
                
                pub fn with_private_field(mut self, value: String) -> Self {
                    self.inner.private_field = value;
                    self
                }
                
                pub fn with_optional(mut self, value: Option<usize>) -> Self {
                    self.inner.optional = value;
                    self
                }
                
                pub fn with_test(mut self, value: std::option::Option<String>) -> Self {
                    self.inner.test = value;
                    self
                }
                
                pub fn with_test2(mut self, value: option::Option<T>) -> Self {
                    self.inner.test2 = value;
                    self
                }
                
                pub fn with_dynamic(mut self, value: Box<dyn Send>) -> Self {
                    self.inner.dynamic = value;
                    self
                }
                
                pub fn with_dynamic2(mut self, value: Box<Option<dyn Send>>) -> Self {
                    self.inner.dynamic2 = value;
                    self
                }
                
                pub fn build(self) -> MyStruct {
                    self.inner
                }
            }
        };

        let impl_subject_fn_builder = ImplBuilderFns::from(&item_struct);

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
            impl MyStructBuilder {
                pub fn with_0(mut self, value: String) -> Self {
                    self.inner.0 = value;
                    self
                }
                
                pub fn with_1(mut self, value: String) -> Self {
                    self.inner.1 = value;
                    self
                }
                
                pub fn with_2(mut self, value: Option<usize>) -> Self {
                    self.inner.2 = value;
                    self
                }
                
                pub fn with_3(mut self, value: std::option::Option<String>) -> Self {
                    self.inner.3 = value;
                    self
                }
                
                pub fn with_4(mut self, value: option::Option<T>) -> Self {
                    self.inner.4 = value;
                    self
                }
                
                pub fn with_5(mut self, value: Box<dyn Send>) -> Self {
                    self.inner.5 = value;
                    self
                }
                
                pub fn with_6(mut self, value: Box<Option<dyn Send>>) -> Self {
                    self.inner.6 = value;
                    self
                }
                
                pub fn build(self) -> MyStruct {
                    self.inner
                }
            }
        };

        let subject_impl = ImplBuilderFns::from(&item_struct);

        assert_eq!(
            subject_impl.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unit_struct() {
        let item_struct = parse_quote! { pub struct MyStruct; };

        let impl_builder_fns = ImplBuilderFns::from(&item_struct);

        assert_eq!(
            impl_builder_fns.to_token_stream().to_string(),
            TokenStream::new().to_string()
        );       
    }
}
