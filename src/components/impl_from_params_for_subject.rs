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
