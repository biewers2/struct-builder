use crate::struct_builder::BuilderIdents;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Fields, ItemImpl, ItemStruct};

pub struct ImplFromSubjectForBuilder {
    idents: BuilderIdents,
    unit: bool
}

impl From<&ItemStruct> for ImplFromSubjectForBuilder {
    fn from(value: &ItemStruct) -> Self {
        let idents = BuilderIdents::from(value);
        let unit = if let Fields::Unit = &value.fields { true } else { false };

        Self { idents, unit }
    }
}

impl ToTokens for ImplFromSubjectForBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderIdents {
            subject,
            builder,
            builder_subject_field,
            ..
        } = &self.idents;

        if !self.unit {
            let item_impl: ItemImpl = parse_quote! {
                impl From<#subject> for #builder {
                    fn from(value: #subject) -> Self {
                        Self { #builder_subject_field: value }
                    }
                }
            };

            item_impl.to_tokens(tokens);
        }
    }
}
