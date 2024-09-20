use crate::struct_builder::BuilderContext;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Fields, ItemImpl, ItemStruct};

pub struct ImplFromParamsForSubject {
    idents: BuilderContext,
    unit: bool
}

impl From<&ItemStruct> for ImplFromParamsForSubject {
    fn from(value: &ItemStruct) -> Self {
        let idents = BuilderContext::from(value);
        let unit = if let Fields::Unit = &value.fields { true } else { false };

        Self { idents, unit }
    }
}

impl ToTokens for ImplFromParamsForSubject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            subject,
            params,
            ..
        } = &self.idents;

        if !self.unit {
            let item_impl: ItemImpl = parse_quote! {
                impl From<#params> for #subject {
                    fn from(value: #params) -> Self {
                        Self::builder(value).build()
                    }
                }
            };

            item_impl.to_tokens(tokens);
        }
    }
}
