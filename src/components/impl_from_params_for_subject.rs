use crate::struct_builder::{BuilderContext, GenericsContext};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Fields, ItemImpl, ItemStruct};

pub struct ImplFromParamsForSubject {
    ctx: BuilderContext,
    unit: bool
}

impl From<&ItemStruct> for ImplFromParamsForSubject {
    fn from(value: &ItemStruct) -> Self {
        let ctx: BuilderContext = value.into();
        let unit = matches!(&value.fields, Fields::Unit);

        Self { ctx, unit }
    }
}

impl ToTokens for ImplFromParamsForSubject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            subject,
            params,
            generics,
            ..
        } = &self.ctx;
        let GenericsContext {
            generics_def,
            generics_expr,
            where_clause
        } = &generics;

        if !self.unit {
            let item_impl: ItemImpl = parse_quote! {
                impl #generics_def From<#params #generics_expr> for #subject #generics_expr #where_clause {
                    fn from(value: #params #generics_expr) -> Self {
                        Self::builder(value).build()
                    }
                }
            };

            item_impl.to_tokens(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::ImplFromParamsForSubject;
    use crate::test_util::{sample_named_item_struct, sample_unit_item_struct, sample_unnamed_item_struct};
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, ItemImpl};

    #[test]
    fn test_with_named_fields() {
        let item_struct = sample_named_item_struct();
        let expected: ItemImpl = parse_quote! {
            impl<T, I: Send, W> From<MyStructParams<T, I, W>> for MyStruct<T, I, W>
            where
                W: Sync
            {
                fn from(value: MyStructParams<T, I, W>) -> Self {
                    Self::builder(value).build()
                }
            }
        };
        
        let impl_from_params_for_subject = ImplFromParamsForSubject::from(&item_struct);

        assert_eq!(
            impl_from_params_for_subject.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unnamed_fields() {
        let item_struct = sample_unnamed_item_struct();
        let expected: ItemImpl = parse_quote! {
            impl<T, I: Send, W> From<MyStructParams<T, I, W>> for MyStruct<T, I, W>
            where
                W: Sync
            {
                fn from(value: MyStructParams<T, I, W>) -> Self {
                    Self::builder(value).build()
                }
            }
        };

        let impl_from_params_for_subject = ImplFromParamsForSubject::from(&item_struct);

        assert_eq!(
            impl_from_params_for_subject.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unit_struct() {
        let item_struct = sample_unit_item_struct();

        let subject_impl = ImplFromParamsForSubject::from(&item_struct);

        assert_eq!(
            subject_impl.to_token_stream().to_string(),
            TokenStream::new().to_string()
        );
    }
}
