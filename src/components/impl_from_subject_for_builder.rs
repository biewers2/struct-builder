use crate::struct_builder::{BuilderContext, GenericsContext};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Fields, ItemImpl, ItemStruct};

pub struct ImplFromSubjectForBuilder {
    ctx: BuilderContext,
    unit: bool
}

impl From<&ItemStruct> for ImplFromSubjectForBuilder {
    fn from(value: &ItemStruct) -> Self {
        let ctx: BuilderContext = value.into();
        let unit = matches!(&value.fields, Fields::Unit);

        Self { ctx, unit }
    }
}

impl ToTokens for ImplFromSubjectForBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            subject,
            builder,
            builder_subject_field,
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
                impl #generics_def From<#subject #generics_expr> for #builder #generics_expr #where_clause {
                    fn from(value: #subject #generics_expr) -> Self {
                        Self { #builder_subject_field: value }
                    }
                }
            };

            item_impl.to_tokens(tokens);
        }
    }
}
