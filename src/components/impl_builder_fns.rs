use proc_macro2::TokenStream;
use crate::struct_builder::{BuilderContext, GenericsContext};
use quote::{format_ident, ToTokens};
use syn::{parse_quote, Fields, ImplItemFn, Index, ItemImpl, ItemStruct};

pub struct ImplBuilderFns {
    ctx: BuilderContext,
    fields: Fields
}

impl From<&ItemStruct> for ImplBuilderFns {
    fn from(value: &ItemStruct) -> Self {
        let ctx: BuilderContext = value.into();
        let fields = value.fields.clone();

        Self { ctx, fields }
    }
}

impl ToTokens for ImplBuilderFns {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            subject,
            builder,
            builder_subject_field,
            generics,
            ..
        } = &self.ctx;

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
            let GenericsContext {
                generics_def,
                generics_expr,
                where_clause
            } = &generics;
            
            let item_impl: ItemImpl = parse_quote! {
                impl #generics_def #builder #generics_expr #where_clause {
                    #(#functions)*

                    pub fn build(self) -> #subject #generics_expr {
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
    use crate::test_util::{sample_named_item_struct, sample_unit_item_struct, sample_unnamed_item_struct};

    #[test]
    fn test_with_named_fields() {
        let item_struct = sample_named_item_struct();
        let expected: ItemImpl = parse_quote! {
            impl<T, I: Send, W> MyStructBuilder<T, I, W>
            where
                W: Sync
            {
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
                
                pub fn with_generic(mut self, value: T) -> Self {
                    self.inner.generic = value;
                    self
                }
                
                pub fn with_generic_inline(mut self, value: I) -> Self {
                    self.inner.generic_inline = value;
                    self
                }
                
                pub fn with_generic_where(mut self, value: W) -> Self {
                    self.inner.generic_where = value;
                    self
                }
                
                pub fn build(self) -> MyStruct<T, I, W> {
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
        let item_struct = sample_unnamed_item_struct();
        let expected: ItemImpl = parse_quote! {
            impl<T, I: Send, W> MyStructBuilder<T, I, W>
            where
                W: Sync
            {
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
                
                pub fn with_7(mut self, value: T) -> Self {
                    self.inner.7 = value;
                    self
                }
                
                pub fn with_8(mut self, value: I) -> Self {
                    self.inner.8 = value;
                    self
                }
                
                pub fn with_9(mut self, value: W) -> Self {
                    self.inner.9 = value;
                    self
                }
                
                pub fn build(self) -> MyStruct<T, I, W> {
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
        let item_struct = sample_unit_item_struct();

        let impl_builder_fns = ImplBuilderFns::from(&item_struct);

        assert_eq!(
            impl_builder_fns.to_token_stream().to_string(),
            TokenStream::new().to_string()
        );       
    }
}
